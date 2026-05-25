import { faPlus } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { Badge, Checkbox, Group, Modal, MultiSelect, Stack } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { basename } from 'pathe';
import { useEffect, useState } from 'react';
import { httpErrorToHuman } from '@/api/axios.ts';
import getNodes from '@/api/admin/nodes/getNodes.ts';
import Button from '@/elements/Button.tsx';
import AdminContentContainer from '@/elements/containers/AdminContentContainer.tsx';
import Select from '@/elements/input/Select.tsx';
import TextInput from '@/elements/input/TextInput.tsx';
import Table, { TableData, TableRow } from '@/elements/Table.tsx';
import { useToast } from '@/providers/ToastProvider.tsx';
import { useTranslations } from '@/providers/TranslationProvider.tsx';
import { createRegion, deleteRegion, listRegions, updateRegion } from '../api/admin.ts';
import RegionBadge from '../components/RegionBadge.tsx';
import type { Region } from '../types/index.ts';

const regionTableColumns = ['Region', 'Nodes', 'Visible', ''];

const flags = import.meta.glob('/node_modules/svg-country-flags/svg/*.svg', { import: 'metadata' });

interface NodeOption {
  value: string;
  label: string;
}

export default function AdminRegionsPage() {
  const { addToast } = useToast();
  const { language } = useTranslations();
  const [regions, setRegions] = useState<Region[]>([]);
  const [nodes, setNodes] = useState<NodeOption[]>([]);
  const [loading, setLoading] = useState(true);
  const [modalOpen, { open: openModal, close: closeModal }] = useDisclosure(false);
  const [editing, setEditing] = useState<Region | null>(null);
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState<string | null>(null);

  const [form, setForm] = useState({
    name: '',
    countryCode: null as string | null,
    city: '',
    visible: true,
    nodeUuids: [] as string[],
  });

  const flagData = Object.keys(flags)
    .filter((f) => basename(f, '.svg').length === 2)
    .map((f) => {
      const code = basename(f, '.svg');
      const regionNames = new Intl.DisplayNames([language], { type: 'region' });
      return { value: code, label: regionNames.of(code.toUpperCase()) || code };
    });

  const refresh = async () => {
    try {
      const data = await listRegions();
      setRegions(data);
    } catch (err) {
      addToast(httpErrorToHuman(err), 'error');
    } finally {
      setLoading(false);
    }
  };

  const loadNodes = async () => {
    try {
      const pagination = await getNodes(1);
      setNodes(pagination.data.map((n) => ({ value: n.uuid, label: n.name })));
    } catch {
      /* nodes optional */
    }
  };

  useEffect(() => {
    refresh();
    loadNodes();
  }, []);

  const openCreate = () => {
    setEditing(null);
    setForm({ name: '', countryCode: null, city: '', visible: true, nodeUuids: [] });
    openModal();
  };

  const openEdit = (region: Region) => {
    setEditing(region);
    setForm({
      name: region.name,
      countryCode: region.countryCode ? region.countryCode.toLowerCase() : null,
      city: region.city ?? '',
      visible: region.visible,
      nodeUuids: region.nodeUuids,
    });
    openModal();
  };

  const handleSave = async () => {
    try {
      setSaving(true);
      const payload = {
        name: form.name.trim(),
        countryCode: (form.countryCode ?? '').toUpperCase(),
        city: form.city.trim() || undefined,
        visible: form.visible,
        nodeUuids: form.nodeUuids,
      };

      if (editing) {
        const updated = await updateRegion(editing.uuid, payload);
        setRegions((prev) => prev.map((r) => (r.uuid === updated.uuid ? updated : r)));
        addToast('Region updated.', 'success');
      } else {
        const created = await createRegion(payload);
        setRegions((prev) => [...prev, created]);
        addToast('Region created.', 'success');
      }
      closeModal();
    } catch (err) {
      addToast(httpErrorToHuman(err), 'error');
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = async (uuid: string) => {
    try {
      setDeleting(uuid);
      await deleteRegion(uuid);
      setRegions((prev) => prev.filter((r) => r.uuid !== uuid));
      addToast('Region deleted.', 'success');
    } catch (err) {
      addToast(httpErrorToHuman(err), 'error');
    } finally {
      setDeleting(null);
    }
  };

  const isValid = form.name.trim().length > 0 && (form.countryCode?.length ?? 0) === 2;

  const pagination = {
    total: regions.length,
    perPage: Math.max(regions.length, 1),
    page: 1,
    data: regions,
  };

  return (
    <AdminContentContainer
      title='Regions'
      contentRight={
        <Button onClick={openCreate} color='blue' leftSection={<FontAwesomeIcon icon={faPlus} />}>
          Add Region
        </Button>
      }
    >
      <Table columns={regionTableColumns} loading={loading} pagination={pagination}>
        {regions.map((region) => (
          <TableRow key={region.uuid}>
            <TableData>
              <RegionBadge region={region} size='sm' />
            </TableData>
            <TableData>
              {region.nodeUuids.length === 0
                ? '-'
                : `${region.nodeUuids.length} node${region.nodeUuids.length === 1 ? '' : 's'}`}
            </TableData>
            <TableData>
              <Badge color={region.visible ? 'green' : 'gray'} variant='light' size='sm'>
                {region.visible ? 'Visible' : 'Hidden'}
              </Badge>
            </TableData>
            <TableData>
              <Group gap='xs' justify='flex-end'>
                <Button size='xs' variant='light' onClick={() => openEdit(region)}>
                  Edit
                </Button>
                <Button
                  size='xs'
                  variant='light'
                  color='red'
                  loading={deleting === region.uuid}
                  onClick={() => handleDelete(region.uuid)}
                >
                  Delete
                </Button>
              </Group>
            </TableData>
          </TableRow>
        ))}
      </Table>

      <Modal
        opened={modalOpen}
        onClose={closeModal}
        title={editing ? 'Edit Region' : 'Add Region'}
        size='md'
      >
        <Stack gap='sm'>
          <Group grow>
            <TextInput
              label='Name'
              placeholder='United Kingdom'
              value={form.name}
              onChange={(e) => setForm((f) => ({ ...f, name: e.currentTarget.value }))}
              required
            />
            <TextInput
              label='City'
              placeholder='London'
              value={form.city}
              onChange={(e) => setForm((f) => ({ ...f, city: e.currentTarget.value }))}
            />
          </Group>

          <Select
            label='Flag'
            placeholder='Select a country…'
            renderOption={({ option }) => (
              <div className='flex items-center gap-2'>
                <img
                  src={`/flags/${option.value}.svg`}
                  alt={option.label}
                  className='w-4 h-4 rounded-md shrink-0'
                />
                <span className='truncate'>{option.label}</span>
              </div>
            )}
            data={flagData}
            value={form.countryCode}
            onChange={(v) => setForm((f) => ({ ...f, countryCode: v }))}
            searchable
            clearable
            required
            leftSection={
              form.countryCode ? (
                <img
                  src={`/flags/${form.countryCode}.svg`}
                  alt={form.countryCode}
                  className='w-4 h-4 rounded-md'
                />
              ) : null
            }
          />

          <MultiSelect
            label='Assigned Nodes'
            description='Servers on these nodes will show this region.'
            data={nodes}
            value={form.nodeUuids}
            onChange={(v) => setForm((f) => ({ ...f, nodeUuids: v }))}
            searchable
            clearable
            placeholder={nodes.length === 0 ? 'No nodes available' : 'Select nodes…'}
          />

          <Checkbox
            label='Visible to users'
            checked={form.visible}
            onChange={(e) => setForm((f) => ({ ...f, visible: e.currentTarget.checked }))}
          />

          <Group justify='flex-end' mt='xs'>
            <Button variant='default' onClick={closeModal}>
              Cancel
            </Button>
            <Button loading={saving} disabled={!isValid} onClick={handleSave}>
              {editing ? 'Save Changes' : 'Create Region'}
            </Button>
          </Group>
        </Stack>
      </Modal>
    </AdminContentContainer>
  );
}

import { axiosInstance } from '@/api/axios.ts';
import type { Region } from '../types/index.ts';

const normalize = (r: any): Region => ({
  uuid: r.uuid,
  name: r.name,
  countryCode: r.countryCode,
  city: r.city ?? null,
  visible: r.visible,
  nodeUuids: r.nodeUuids ?? [],
  created: r.created,
  updated: r.updated,
});

export const listRegions = async (): Promise<Region[]> => {
  const { data } = await axiosInstance.get('/api/admin/regions');
  return (data.regions as any[]).map(normalize);
};

export const getRegion = async (uuid: string): Promise<Region> => {
  const { data } = await axiosInstance.get(`/api/admin/regions/${uuid}`);
  return normalize(data.region);
};

export const createRegion = async (payload: {
  name: string;
  countryCode: string;
  city?: string;
  visible?: boolean;
  nodeUuids?: string[];
}): Promise<Region> => {
  const { data } = await axiosInstance.post('/api/admin/regions', {
    name: payload.name,
    country_code: payload.countryCode,
    city: payload.city || null,
    visible: payload.visible ?? true,
    node_uuids: payload.nodeUuids ?? [],
  });
  return normalize(data.region);
};

export const updateRegion = async (
  uuid: string,
  payload: {
    name?: string;
    countryCode?: string;
    city?: string | null;
    visible?: boolean;
    nodeUuids?: string[];
  },
): Promise<Region> => {
  const { data } = await axiosInstance.patch(`/api/admin/regions/${uuid}`, {
    name: payload.name,
    country_code: payload.countryCode,
    city: payload.city,
    visible: payload.visible,
    node_uuids: payload.nodeUuids,
  });
  return normalize(data.region);
};

export const deleteRegion = async (uuid: string): Promise<void> => {
  await axiosInstance.delete(`/api/admin/regions/${uuid}`);
};

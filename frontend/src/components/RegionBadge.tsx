import { Group, Text } from '@mantine/core';
import type { ServerRegion } from '../types/index.ts';

interface Props {
  region: ServerRegion;
  size?: 'sm' | 'md';
}

export default function RegionBadge({ region, size = 'md' }: Props) {
  const cc = region.countryCode.toLowerCase();
  const flagUrl = `https://flagcdn.com/w40/${cc}.png`;

  return (
    <Group gap={size === 'sm' ? 6 : 8} wrap='nowrap' align='center'>
      <img
        src={flagUrl}
        alt={region.countryCode}
        width={size === 'sm' ? 18 : 22}
        style={{ borderRadius: 2, flexShrink: 0 }}
      />
      <div>
        <Text size={size === 'sm' ? 'xs' : 'sm'} c='dimmed' lh={1.1}>
          Region
        </Text>
        <Text size={size === 'sm' ? 'sm' : 'md'} fw={600} lh={1.2}>
          {region.name}
          {region.city && (
            <Text span size='xs' c='dimmed' fw={400}>
              {' '}/ {region.city}
            </Text>
          )}
        </Text>
      </div>
    </Group>
  );
}

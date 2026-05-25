import { useEffect, useState } from 'react';
import Card from '@/elements/Card.tsx';
import { useServerStore } from '@/stores/server.ts';
import { getServerRegion } from '../api/client.ts';
import type { ServerRegion } from '../types/index.ts';

export default function ServerRegionCard() {
  const server = useServerStore((s) => s.server);
  const [region, setRegion] = useState<ServerRegion | null>(null);

  useEffect(() => {
    if (!server?.uuid) return;
    getServerRegion(server.uuid)
      .then(setRegion)
      .catch(() => {});
  }, [server?.uuid]);

  if (!region) return null;

  const cc = region.countryCode.toLowerCase();

  return (
    <Card className='flex flex-row! items-center' style={{ order: 90 }}>
      <img
        src={`https://flagcdn.com/w40/${cc}.png`}
        alt={region.countryCode}
        style={{ width: 36, height: 'auto', borderRadius: 4, flexShrink: 0 }}
      />
      <div className='flex flex-col ml-4 w-full min-w-0'>
        <span className='text-sm text-left text-(--mantine-color-dimmed) font-bold'>Region</span>
        <span className='text-lg font-bold truncate max-w-full block'>
          {region.name}
          {region.city && (
            <span className='text-sm text-(--mantine-color-dimmed) font-normal'> {region.city}</span>
          )}
        </span>
      </div>
    </Card>
  );
}

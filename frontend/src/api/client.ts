import { axiosInstance } from '@/api/axios.ts';
import type { ServerRegion } from '../types/index.ts';

export const getServerRegion = async (serverUuid: string): Promise<ServerRegion | null> => {
  const { data } = await axiosInstance.get(`/api/client/servers/${serverUuid}/region`);
  if (!data.region) return null;
  return {
    uuid: data.region.uuid,
    name: data.region.name,
    countryCode: data.region.countryCode,
    city: data.region.city ?? null,
  };
};

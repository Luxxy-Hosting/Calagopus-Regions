export interface Region {
  uuid: string;
  name: string;
  countryCode: string;
  city: string | null;
  visible: boolean;
  nodeUuids: string[];
  created: string;
  updated: string;
}

export interface ServerRegion {
  uuid: string;
  name: string;
  countryCode: string;
  city: string | null;
}

CREATE TABLE IF NOT EXISTS ext_region_regions (
    uuid uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar(100) NOT NULL,
    country_code char(2) NOT NULL,
    city varchar(100),
    visible boolean NOT NULL DEFAULT TRUE,
    created timestamp NOT NULL DEFAULT NOW(),
    updated timestamp NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS ext_region_regions_visible_idx
    ON ext_region_regions(visible, name);

CREATE TABLE IF NOT EXISTS ext_region_node_regions (
    node_uuid uuid NOT NULL REFERENCES nodes(uuid) ON DELETE CASCADE,
    region_uuid uuid NOT NULL REFERENCES ext_region_regions(uuid) ON DELETE CASCADE,
    PRIMARY KEY (node_uuid)
);

CREATE INDEX IF NOT EXISTS ext_region_node_regions_region_idx
    ON ext_region_node_regions(region_uuid);

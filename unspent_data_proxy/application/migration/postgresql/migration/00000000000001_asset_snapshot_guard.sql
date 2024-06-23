-- migrate:up
CREATE TABLE public.asset_snapshot_guard (
    id SMALLINT NOT NULL,
    last_inserted_timestamp TIMESTAMP(6) WITH TIME ZONE NOT NULL
)

-- migrate:down
DROP TABLE public.asset_snapshot_guard;
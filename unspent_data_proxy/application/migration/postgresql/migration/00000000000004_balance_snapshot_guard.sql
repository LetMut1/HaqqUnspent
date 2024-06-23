-- migrate:up
CREATE TABLE public.balance_snapshot_guard (
    id SMALLINT NOT NULL,
    last_inserted_id BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.balance_snapshot_guard;
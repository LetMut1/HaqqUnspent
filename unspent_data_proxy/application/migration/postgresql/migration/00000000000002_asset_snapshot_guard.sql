-- migrate:up
CREATE UNIQUE INDEX asset_snapshot_guard_1 ON public.asset_snapshot_guard USING btree (id);

-- migrate:down
DROP INDEX asset_snapshot_guard_1;
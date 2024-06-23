-- migrate:up
ALTER TABLE public.asset_snapshot_guard
ADD CONSTRAINT asset_snapshot_guard_2 UNIQUE USING INDEX asset_snapshot_guard_1;

-- migrate:down
ALTER TABLE public.asset_snapshot_guard
DROP CONSTRAINT asset_snapshot_guard_2;
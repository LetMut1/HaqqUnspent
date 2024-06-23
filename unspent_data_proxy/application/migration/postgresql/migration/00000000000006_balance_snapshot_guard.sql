-- migrate:up
ALTER TABLE public.balance_snapshot_guard
ADD CONSTRAINT balance_snapshot_guard_2 UNIQUE USING INDEX balance_snapshot_guard_1;

-- migrate:down
ALTER TABLE public.balance_snapshot_guard
DROP CONSTRAINT balance_snapshot_guard_2;
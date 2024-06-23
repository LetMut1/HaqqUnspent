-- migrate:up
CREATE UNIQUE INDEX balance_snapshot_guard_1 ON public.balance_snapshot_guard USING btree (id);

-- migrate:down
DROP INDEX balance_snapshot_guard_1;
-- migrate:up
CREATE UNIQUE INDEX aislm_stake_2 ON public.aislm_stake USING btree (id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS aislm_stake_2;
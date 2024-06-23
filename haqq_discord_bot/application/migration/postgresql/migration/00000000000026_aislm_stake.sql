-- migrate:up
ALTER TABLE public.aislm_stake
ADD CONSTRAINT aislm_stake_3 UNIQUE USING INDEX aislm_stake_2;

-- migrate:down
ALTER TABLE public.aislm_stake
DROP CONSTRAINT aislm_stake_3;
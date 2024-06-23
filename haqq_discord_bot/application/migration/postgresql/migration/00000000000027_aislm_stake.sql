-- migrate:up
CREATE UNIQUE INDEX aislm_stake_4 ON public.aislm_stake USING btree (raffle_id, created_at, discord_user_id, bech32_address ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS aislm_stake_4;
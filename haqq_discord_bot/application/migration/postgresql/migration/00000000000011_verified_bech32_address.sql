-- migrate:up
CREATE INDEX verified_bech32_address_3 ON public.verified_bech32_address USING btree (discord_user_id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS verified_bech32_address_3;
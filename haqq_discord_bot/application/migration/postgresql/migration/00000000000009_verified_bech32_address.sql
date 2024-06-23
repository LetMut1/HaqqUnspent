-- migrate:up
CREATE UNIQUE INDEX verified_bech32_address_1 ON public.verified_bech32_address USING btree (value ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS verified_bech32_address_1;
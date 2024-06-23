-- migrate:up
CREATE UNIQUE INDEX verified_address_blacklist_1 ON public.verified_address_blacklist USING btree (bech32_address ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS verified_address_blacklist_1;
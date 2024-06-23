-- migrate:up
CREATE UNIQUE INDEX address_verification_data_1 ON public.address_verification_data USING btree (discord_user_id, recipient_bech32_address ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS address_verification_data_1;
-- migrate:up
ALTER TABLE public.verified_bech32_address
ADD CONSTRAINT verified_bech32_address_2 UNIQUE USING INDEX verified_bech32_address_1;

-- migrate:down
ALTER TABLE public.verified_bech32_address
DROP CONSTRAINT verified_bech32_address_2;
-- migrate:up
ALTER TABLE public.verified_address_blacklist
ADD CONSTRAINT verified_address_blacklist_2 UNIQUE USING INDEX verified_address_blacklist_1;

-- migrate:down
ALTER TABLE public.verified_address_blacklist
DROP CONSTRAINT verified_address_blacklist_2;
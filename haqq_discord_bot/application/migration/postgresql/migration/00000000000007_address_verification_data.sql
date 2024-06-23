-- migrate:up
ALTER TABLE public.address_verification_data
ADD CONSTRAINT address_verification_data_2 UNIQUE USING INDEX address_verification_data_1;

-- migrate:down
ALTER TABLE public.address_verification_data
DROP CONSTRAINT address_verification_data_2;
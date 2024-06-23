-- migrate:up
ALTER TABLE public.raffle
ADD CONSTRAINT raffle_3 UNIQUE USING INDEX raffle_2;

-- migrate:down
ALTER TABLE public.raffle
DROP CONSTRAINT raffle_3;
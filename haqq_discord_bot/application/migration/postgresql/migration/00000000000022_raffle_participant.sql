-- migrate:up
ALTER TABLE public.raffle_participant
ADD CONSTRAINT raffle_participant_2 UNIQUE USING INDEX raffle_participant_1;

-- migrate:down
ALTER TABLE public.raffle_participant
DROP CONSTRAINT raffle_participant_2;
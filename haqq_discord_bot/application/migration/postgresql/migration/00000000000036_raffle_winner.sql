-- migrate:up
ALTER TABLE public.raffle_winner
ADD CONSTRAINT raffle_winner_2 UNIQUE USING INDEX raffle_winner_1;

-- migrate:down
ALTER TABLE public.raffle_winner
DROP CONSTRAINT raffle_winner_2;
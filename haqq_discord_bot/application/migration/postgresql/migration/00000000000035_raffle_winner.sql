-- migrate:up
CREATE UNIQUE INDEX raffle_winner_1 ON public.raffle_winner USING btree (raffle_id, discord_user_id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS raffle_winner_1;
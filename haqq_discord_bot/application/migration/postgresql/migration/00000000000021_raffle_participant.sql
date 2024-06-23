-- migrate:up
CREATE UNIQUE INDEX raffle_participant_1 ON public.raffle_participant USING btree (raffle_id, discord_user_id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS raffle_participant_1;
-- migrate:up
CREATE UNIQUE INDEX prize_transfer_proof_1 ON public.prize_transfer_proof USING btree (raffle_id, discord_user_id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS prize_transfer_proof_1;
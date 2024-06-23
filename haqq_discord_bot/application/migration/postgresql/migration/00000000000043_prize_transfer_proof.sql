-- migrate:up
ALTER TABLE public.prize_transfer_proof
ADD CONSTRAINT prize_transfer_proof_2 UNIQUE USING INDEX prize_transfer_proof_1;

-- migrate:down
ALTER TABLE public.prize_transfer_proof
DROP CONSTRAINT prize_transfer_proof_2;
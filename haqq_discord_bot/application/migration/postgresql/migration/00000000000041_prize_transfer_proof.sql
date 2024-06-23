-- migrate:up
CREATE TABLE public.prize_transfer_proof (
    raffle_id BIGINT NOT NULL,
    discord_user_id TEXT NOT NULL,
    evm_transaction_hash TEXT NOT NULL,
    created_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.prize_transfer_proof;
-- migrate:up
CREATE TABLE public.raffle_winner (
    raffle_id BIGINT NOT NULL,
    discord_user_id TEXT NOT NULL,
    bech32_address TEXT NOT NULL,
    created_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.raffle_winner;
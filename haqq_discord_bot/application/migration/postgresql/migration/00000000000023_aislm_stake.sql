-- migrate:up
CREATE TABLE public.aislm_stake (
    id BIGINT NOT NULL,
    amount TEXT NOT NULL,
    raffle_id BIGINT NOT NULL,
    bech32_address TEXT NOT NULL,
    discord_user_id TEXT NOT NULL,
    created_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.aislm_stake;
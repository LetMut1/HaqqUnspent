-- migrate:up
CREATE TABLE public.verified_bech32_address (
    value TEXT NOT NULL,
    discord_user_id TEXT NOT NULL,
    created_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.verified_bech32_address;
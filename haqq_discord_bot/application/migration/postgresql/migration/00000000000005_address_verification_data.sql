-- migrate:up
CREATE TABLE public.address_verification_data (
    discord_user_id TEXT NOT NULL,
    recipient_bech32_address TEXT NOT NULL,
    expected_token_quantity TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    expired_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.address_verification_data;
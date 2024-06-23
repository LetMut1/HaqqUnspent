-- migrate:up
CREATE TABLE public.verified_address_blacklist (
    bech32_address TEXT NOT NULL,
    created_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.verified_address_blacklist;
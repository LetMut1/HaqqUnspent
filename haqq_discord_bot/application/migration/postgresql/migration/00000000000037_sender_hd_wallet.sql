-- migrate:up
CREATE TABLE public.sender_hd_wallet (
    id BIGINT NOT NULL,
    mnemonic_phrase TEXT NOT NULL,
    mnemonic_derevation_path_index INTEGER NOT NULL,
    created_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.sender_hd_wallet;
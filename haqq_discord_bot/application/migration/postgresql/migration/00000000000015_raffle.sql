-- migrate:up
CREATE TABLE public.raffle (
    id BIGINT NOT NULL,
    islm_prize_amount BIGINT NOT NULL,
    winners_number BIGINT NOT NULL,
    seed TEXT NOT NULL,
    aes_key TEXT NOT NULL,
    status SMALLINT NOT NULL,
    created_at BIGINT NOT NULL,
    expired_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.raffle;
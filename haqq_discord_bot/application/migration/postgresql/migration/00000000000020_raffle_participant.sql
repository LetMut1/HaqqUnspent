-- migrate:up
CREATE TABLE public.raffle_participant (
    raffle_id BIGINT NOT NULL,
    discord_user_id TEXT NOT NULL,
    created_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.raffle_participant;
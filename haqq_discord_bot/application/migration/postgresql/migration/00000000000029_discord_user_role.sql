-- migrate:up
CREATE TABLE public.discord_user_role (
    discord_user_id TEXT NOT NULL,
    wallet_verified BOOLEAN NOT NULL,
    stakers_club_member BOOLEAN NOT NULL,
    updated_at BIGINT NOT NULL
)

-- migrate:down
DROP TABLE public.discord_user_role;
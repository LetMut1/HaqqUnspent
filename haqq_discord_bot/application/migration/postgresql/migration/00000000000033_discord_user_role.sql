-- migrate:up
CREATE INDEX discord_user_role_4 ON public.discord_user_role USING btree (stakers_club_member ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS discord_user_role_4;
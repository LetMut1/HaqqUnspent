-- migrate:up
CREATE UNIQUE INDEX discord_user_role_1 ON public.discord_user_role USING btree (discord_user_id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS discord_user_role_1;
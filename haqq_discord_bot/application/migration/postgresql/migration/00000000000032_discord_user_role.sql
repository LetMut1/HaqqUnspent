-- migrate:up
CREATE INDEX discord_user_role_3 ON public.discord_user_role USING btree (wallet_verified ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS discord_user_role_3;
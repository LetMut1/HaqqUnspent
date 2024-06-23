-- migrate:up
ALTER TABLE public.discord_user_role
ADD CONSTRAINT discord_user_role_2 UNIQUE USING INDEX discord_user_role_1;

-- migrate:down
ALTER TABLE public.discord_user_role
DROP CONSTRAINT discord_user_role_2;
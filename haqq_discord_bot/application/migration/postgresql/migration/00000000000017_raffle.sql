-- migrate:up
CREATE UNIQUE INDEX raffle_2 ON public.raffle USING btree (id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS raffle_2;
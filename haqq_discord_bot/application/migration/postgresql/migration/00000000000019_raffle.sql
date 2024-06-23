-- migrate:up
CREATE INDEX raffle_4 ON public.raffle USING btree (status ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS raffle_4;
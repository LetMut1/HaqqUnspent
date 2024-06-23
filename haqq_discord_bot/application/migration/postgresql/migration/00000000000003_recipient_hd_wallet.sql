-- migrate:up
CREATE UNIQUE INDEX recipient_hd_wallet_2 ON public.recipient_hd_wallet USING btree (id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS recipient_hd_wallet_2;
-- migrate:up
CREATE UNIQUE INDEX sender_hd_wallet_2 ON public.sender_hd_wallet USING btree (id ASC NULLS LAST);

-- migrate:down
DROP INDEX IF EXISTS sender_hd_wallet_2;
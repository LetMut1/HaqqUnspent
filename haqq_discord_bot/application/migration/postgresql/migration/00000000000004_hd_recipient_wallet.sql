-- migrate:up
ALTER TABLE public.recipient_hd_wallet
ADD CONSTRAINT recipient_hd_wallet_3 UNIQUE USING INDEX recipient_hd_wallet_2;

-- migrate:down
ALTER TABLE public.recipient_hd_wallet
DROP CONSTRAINT recipient_hd_wallet_3;
-- migrate:up
ALTER TABLE public.sender_hd_wallet
ADD CONSTRAINT sender_hd_wallet_3 UNIQUE USING INDEX sender_hd_wallet_2;

-- migrate:down
ALTER TABLE public.sender_hd_wallet
DROP CONSTRAINT sender_hd_wallet_3;
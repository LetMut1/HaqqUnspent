-- migrate:up
CREATE SEQUENCE public.sender_hd_wallet_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.sender_hd_wallet.id;

-- migrate:down
DROP SEQUENCE public.sender_hd_wallet_1;
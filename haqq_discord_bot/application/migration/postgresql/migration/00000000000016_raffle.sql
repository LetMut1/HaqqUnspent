-- migrate:up
CREATE SEQUENCE public.raffle_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.raffle.id;

-- migrate:down
DROP SEQUENCE public.raffle_1;
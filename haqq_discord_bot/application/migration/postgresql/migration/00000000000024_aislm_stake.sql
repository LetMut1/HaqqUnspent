-- migrate:up
CREATE SEQUENCE public.aislm_stake_1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE
START WITH 1 CACHE 1 NO CYCLE OWNED BY public.aislm_stake.id;

-- migrate:down
DROP SEQUENCE public.aislm_stake_1;
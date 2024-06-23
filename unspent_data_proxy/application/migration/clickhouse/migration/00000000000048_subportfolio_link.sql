-- migrate:up
ALTER TABLE unspentio.subportfolio_link MATERIALIZE PROJECTION projection_1;

-- migrate:down
SELECT TRUE;

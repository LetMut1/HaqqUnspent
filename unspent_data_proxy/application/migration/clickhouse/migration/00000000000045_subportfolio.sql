-- migrate:up
ALTER TABLE unspentio.subportfolio MATERIALIZE PROJECTION projection_1;

-- migrate:down
SELECT TRUE;

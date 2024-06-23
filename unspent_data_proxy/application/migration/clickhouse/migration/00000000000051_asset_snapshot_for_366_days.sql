-- migrate:up
ALTER TABLE unspentio.asset_snapshot_for_366_days MATERIALIZE PROJECTION projection_1;

-- migrate:down
SELECT TRUE;

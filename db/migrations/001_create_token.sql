CREATE TABLE IF NOT EXISTS tokens (
    name TEXT,
    address  TEXT PRIMARY KEY,
    symbol TEXT,
    decimals INT,
    first_seen_block BIGINT NOT NULL,
    last_seen_block BIGINT NOT NULL
)
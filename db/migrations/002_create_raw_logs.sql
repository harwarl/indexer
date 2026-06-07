CREATE TABLE IF NOT EXISTS raw_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    block_number BIGINT NOT NULL,
    block_timestamp BIGINT NOT NULL,
    tx_hash TEXT,
    tx_index BIGINT,
    log_index BIGINT,
    address TEXT NOT NULL, 
    topics TEXT NOT NULL,
    data TEXT NOT NULL,
    indexed_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (tx_hash, log_index)
);

CREATE INDEX idx_raw_logs_block_number ON raw_logs (block_number);
CREATE INDEX idx_raw_logs_address ON raw_logs (address);
CREATE INDEX idx_raw_logs_tx_hash ON raw_logs (tx_hash);
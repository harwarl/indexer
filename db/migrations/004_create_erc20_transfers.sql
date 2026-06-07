CREATE TABLE IF NOT EXISTS  erc20_transfers (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    raw_log_id      UUID REFERENCES raw_logs(id) ON DELETE CASCADE,
    block_number    BIGINT NOT NULL,
    block_timestamp BIGINT NOT NULL,
    tx_hash         TEXT NOT NULL,
    address TEXT NOT NULL REFERENCES tokens(address),
    from_address    TEXT NOT NULL,
    to_address      TEXT NOT NULL,
    value           NUMERIC NOT NULL,
    indexed_at      TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_erc20_transfers_block_number ON erc20_transfers (block_number);
CREATE INDEX idx_erc20_transfers_address ON erc20_transfers (address);
CREATE INDEX idx_erc20_transfers_from ON erc20_transfers (from_address);
CREATE INDEX idx_erc20_transfers_to ON erc20_transfers (to_address);
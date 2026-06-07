CREATE TABLE IF NOT EXISTS erc20_approvals (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    raw_log_id      UUID REFERENCES raw_logs(id) ON DELETE CASCADE,
    block_number    BIGINT NOT NULL,
    block_timestamp BIGINT NOT NULL,
    tx_hash         TEXT NOT NULL,
    address TEXT NOT NULL REFERENCES tokens(address),
    owner           TEXT NOT NULL,
    spender         TEXT NOT NULL,
    value           NUMERIC NOT NULL,
    indexed_at      TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_erc20_approvals_block_number ON erc20_approvals (block_number);
CREATE INDEX idx_erc20_approvals_address ON erc20_approvals (address);
CREATE INDEX idx_erc20_approvals_owner ON erc20_approvals (owner);
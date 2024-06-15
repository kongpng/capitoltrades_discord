CREATE TABLE tracked_politicians (
    user_id BIGINT NOT NULL,
    politician_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_price DOUBLE PRECISION,
    last_filing TEXT
);

CREATE UNIQUE INDEX tracked_politicians_idx ON tracked_politicians (
    user_id, politician_id
);

CREATE TABLE tracked_issuers (
    user_id BIGINT NOT NULL,
    issuer_id BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_price DOUBLE PRECISION,
    last_filing TEXT
);

CREATE UNIQUE INDEX tracked_issuers_idx ON tracked_issuers (user_id, issuer_id);

CREATE TABLE notified_trades (
    user_id BIGINT NOT NULL,
    tx_id BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX notified_trades_idx ON notified_trades (user_id, tx_id);

-- New table to record notifications sent to users
CREATE TABLE notifications (
    notification_id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    politician_id TEXT,
    issuer_id BIGINT,
    notification_type TEXT NOT NULL, -- e.g., 'price_update', 'filing_update'
    notification_content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
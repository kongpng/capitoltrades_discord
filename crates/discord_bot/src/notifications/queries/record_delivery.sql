INSERT INTO notified_trades (user_id, tx_id)
VALUES ($1, $2) ON CONFLICT DO NOTHING;

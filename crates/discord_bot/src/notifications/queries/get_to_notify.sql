SELECT user_id::BIGINT
FROM tracked_politicians
WHERE politician_id = $1
  AND user_id NOT IN (
    SELECT user_id
    FROM notified_trades
    WHERE tx_id = $3::BIGINT
  )
UNION
SELECT user_id::BIGINT
FROM tracked_issuers
WHERE issuer_id = $2::BIGINT
  AND user_id NOT IN (
    SELECT user_id
    FROM notified_trades
    WHERE tx_id = $3::BIGINT
  )
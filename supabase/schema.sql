-- X Community Bot - Supabase Schema
-- Run this in Supabase SQL Editor

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tokens table
CREATE TABLE IF NOT EXISTS tokens (
    pool_address TEXT PRIMARY KEY,
    base_token TEXT,
    chain TEXT DEFAULT 'sol',
    token_symbol TEXT,
    token_name TEXT,
    community_id TEXT,
    admin_username TEXT,
    twitter_url TEXT,
    website_url TEXT,
    market_cap REAL,
    price_usd REAL,
    liquid_usd REAL,
    curve_percent REAL,
    holder_count INTEGER,
    total_supply TEXT,
    created_at BIGINT,
    detected_at BIGINT,
    price_change_percent REAL,
    total_volume REAL,
    buy_volume REAL,
    sell_volume REAL,
    trade_count INTEGER,
    buy_count INTEGER,
    sell_count INTEGER,
    ath_market_cap TEXT,
    ath_detected_at BIGINT,
    is_migrated INTEGER DEFAULT 0,
    reached_final_stretch INTEGER DEFAULT 0,
    reached_10k INTEGER DEFAULT 0,
    last_updated BIGINT,
    migrated_at BIGINT,
    market_cap_at_migration REAL,
    community_added_via_dex INTEGER DEFAULT 0,
    token_score INTEGER,
    is_manipulated INTEGER DEFAULT 0,
    manipulation_reason TEXT,
    dev_wallet TEXT,
    manipulation_checked INTEGER DEFAULT 0,
    platform TEXT DEFAULT 'pump.fun'
);

-- Admins table
CREATE TABLE IF NOT EXISTS admins (
    admin_username TEXT PRIMARY KEY,
    score INTEGER,
    total_rating REAL,
    tokens_launched INTEGER DEFAULT 0,
    tokens_score_0 INTEGER DEFAULT 0,
    tokens_score_1 INTEGER DEFAULT 0,
    tokens_score_2 INTEGER DEFAULT 0,
    tokens_score_3 INTEGER DEFAULT 0,
    tokens_score_4 INTEGER DEFAULT 0,
    tokens_score_5 INTEGER DEFAULT 0,
    tokens_score_6 INTEGER DEFAULT 0,
    last_updated BIGINT,
    total_tokens_created INTEGER DEFAULT 0,
    winrate REAL,
    dex_comm_added INTEGER DEFAULT 0
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_tokens_base_token ON tokens(base_token);
CREATE INDEX IF NOT EXISTS idx_tokens_admin_username ON tokens(admin_username);
CREATE INDEX IF NOT EXISTS idx_tokens_community_id ON tokens(community_id);
CREATE INDEX IF NOT EXISTS idx_tokens_is_migrated ON tokens(is_migrated);
CREATE INDEX IF NOT EXISTS idx_tokens_reached_10k ON tokens(reached_10k);
CREATE INDEX IF NOT EXISTS idx_tokens_platform ON tokens(platform);
CREATE INDEX IF NOT EXISTS idx_tokens_admin_ath ON tokens(admin_username, ath_market_cap);
CREATE INDEX IF NOT EXISTS idx_tokens_community_ath ON tokens(community_id, ath_market_cap);

-- Function to update admin stats (call this periodically)
CREATE OR REPLACE FUNCTION update_admin_stats()
RETURNS void AS $$
BEGIN
    INSERT INTO admins (admin_username, total_rating, tokens_launched,
                        tokens_score_0, tokens_score_1, tokens_score_2,
                        tokens_score_3, tokens_score_4, tokens_score_5, tokens_score_6,
                        last_updated, total_tokens_created, winrate)
    SELECT
        admin_username,
        AVG(CAST(token_score AS REAL)),
        COUNT(CASE WHEN token_score <= 5 THEN 1 END) as tokens_launched,
        SUM(CASE WHEN token_score = 0 THEN 1 ELSE 0 END),
        SUM(CASE WHEN token_score = 1 THEN 1 ELSE 0 END),
        SUM(CASE WHEN token_score = 2 THEN 1 ELSE 0 END),
        SUM(CASE WHEN token_score = 3 THEN 1 ELSE 0 END),
        SUM(CASE WHEN token_score = 4 THEN 1 ELSE 0 END),
        SUM(CASE WHEN token_score = 5 THEN 1 ELSE 0 END),
        SUM(CASE WHEN token_score = 6 THEN 1 ELSE 0 END),
        EXTRACT(EPOCH FROM NOW())::BIGINT,
        COUNT(DISTINCT community_id),
        CAST(COUNT(CASE WHEN token_score <= 4 THEN 1 END) AS REAL) / NULLIF(COUNT(DISTINCT community_id), 0)
    FROM tokens
    WHERE admin_username IS NOT NULL
    GROUP BY admin_username
    ON CONFLICT (admin_username) DO UPDATE SET
        total_rating = EXCLUDED.total_rating,
        tokens_launched = EXCLUDED.tokens_launched,
        tokens_score_0 = EXCLUDED.tokens_score_0,
        tokens_score_1 = EXCLUDED.tokens_score_1,
        tokens_score_2 = EXCLUDED.tokens_score_2,
        tokens_score_3 = EXCLUDED.tokens_score_3,
        tokens_score_4 = EXCLUDED.tokens_score_4,
        tokens_score_5 = EXCLUDED.tokens_score_5,
        tokens_score_6 = EXCLUDED.tokens_score_6,
        last_updated = EXCLUDED.last_updated,
        total_tokens_created = EXCLUDED.total_tokens_created,
        winrate = EXCLUDED.winrate;
END;
$$ LANGUAGE plpgsql;

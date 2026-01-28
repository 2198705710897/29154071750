-- ============================================================================
-- X Community Bot - Complete Supabase Schema
-- Replicates all functionality from SPREADSHEET_DOCUMENTATION.md
-- ============================================================================

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- TABLES
-- ============================================================================

-- Tokens table (matches spreadsheet token data)
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

-- Admins table (matches spreadsheet admin data)
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

-- Blacklist table (matches Sheet 8)
CREATE TABLE IF NOT EXISTS blacklist (
    admin_username TEXT PRIMARY KEY,
    added_at BIGINT DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
    reason TEXT
);

-- ============================================================================
-- INDEXES
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_tokens_base_token ON tokens(base_token);
CREATE INDEX IF NOT EXISTS idx_tokens_admin_username ON tokens(admin_username);
CREATE INDEX IF NOT EXISTS idx_tokens_community_id ON tokens(community_id);
CREATE INDEX IF NOT EXISTS idx_tokens_is_migrated ON tokens(is_migrated);
CREATE INDEX IF NOT EXISTS idx_tokens_reached_10k ON tokens(reached_10k);
CREATE INDEX IF NOT EXISTS idx_tokens_platform ON tokens(platform);
CREATE INDEX IF NOT EXISTS idx_tokens_admin_ath ON tokens(admin_username, ath_market_cap);
CREATE INDEX IF NOT EXISTS idx_tokens_community_ath ON tokens(community_id, ath_market_cap);

-- ============================================================================
-- VIEWS (These are your "Sheets")
-- ============================================================================

-- Sheet 1: "Admins" - Sorted by Total Rating (Ascending)
CREATE OR REPLACE VIEW sheet_admins AS
SELECT
    admin_username AS "Admin Username",
    total_rating AS "Total Rating",
    tokens_score_0 AS "Score 0",
    tokens_score_1 AS "Score 1",
    tokens_score_2 AS "Score 2",
    tokens_score_3 AS "Score 3",
    tokens_score_4 AS "Score 4",
    tokens_score_5 AS "Score 5",
    total_tokens_created AS "Total Tokens",
    winrate * 100 AS "Winrate %",
    dex_comm_added AS "Dex Comm Added",
    last_updated AS "Last Updated"
FROM admins
ORDER BY total_rating ASC NULLS LAST;

-- Sheet 2: "Admins - Sorted by Winrate" - Sorted by Winrate (Descending)
CREATE OR REPLACE VIEW sheet_admins_by_winrate AS
SELECT
    admin_username AS "Admin Username",
    total_rating AS "Total Rating",
    tokens_score_0 AS "Score 0",
    tokens_score_1 AS "Score 1",
    tokens_score_2 AS "Score 2",
    tokens_score_3 AS "Score 3",
    tokens_score_4 AS "Score 4",
    tokens_score_5 AS "Score 5",
    total_tokens_created AS "Total Tokens",
    winrate * 100 AS "Winrate %",
    dex_comm_added AS "Dex Comm Added",
    last_updated AS "Last Updated"
FROM admins
ORDER BY winrate DESC NULLS LAST;

-- Sheet 3: "Admins - Sorted by Score" - Same as Sheet 1
CREATE OR REPLACE VIEW sheet_admins_by_score AS
SELECT * FROM sheet_admins;

-- Sheet 4: "Admins - High Performers" - Filtered subset
-- Criteria: total_rating <= 2.5 AND (scores 0+1+2) >= 3 AND winrate 70-90% AND NOT blacklisted
CREATE OR REPLACE VIEW sheet_admins_high_performers AS
SELECT
    admin_username AS "Admin Username",
    total_rating AS "Total Rating",
    tokens_score_0 AS "Score 0",
    tokens_score_1 AS "Score 1",
    tokens_score_2 AS "Score 2",
    tokens_score_3 AS "Score 3",
    tokens_score_4 AS "Score 4",
    tokens_score_5 AS "Score 5",
    total_tokens_created AS "Total Tokens",
    winrate * 100 AS "Winrate %",
    dex_comm_added AS "Dex Comm Added",
    last_updated AS "Last Updated"
FROM admins a
WHERE
    a.total_rating <= 2.5
    AND (a.tokens_score_0 + a.tokens_score_1 + a.tokens_score_2) >= 3
    AND a.winrate >= 0.70
    AND a.winrate <= 0.90
    AND NOT EXISTS (SELECT 1 FROM blacklist b WHERE b.admin_username = a.admin_username)
ORDER BY a.total_rating ASC NULLS LAST;

-- Sheet 5: "Tokens - Sorted by Admin" - Tokens with ATH >= 10k
CREATE OR REPLACE VIEW sheet_tokens_by_admin AS
SELECT
    admin_username AS "Admin Username",
    pool_address AS "Token Address",
    token_name AS "Name",
    token_symbol AS "Symbol",
    twitter_url AS "Community Link",
    created_at AS "Age",
    market_cap AS "Current MC",
    CAST(ath_market_cap AS REAL) AS "ATH MC",
    token_score AS "Score",
    community_added_via_dex AS "Dex Comm"
FROM tokens
WHERE
    admin_username IS NOT NULL
    AND CAST(ath_market_cap AS REAL) >= 10000
ORDER BY admin_username ASC, token_score ASC NULLS LAST;

-- Sheet 6: "Tokens - Sorted by Score" - Same tokens, sorted by score
CREATE OR REPLACE VIEW sheet_tokens_by_score AS
SELECT
    admin_username AS "Admin Username",
    pool_address AS "Token Address",
    token_name AS "Name",
    token_symbol AS "Symbol",
    twitter_url AS "Community Link",
    created_at AS "Age",
    market_cap AS "Current MC",
    CAST(ath_market_cap AS REAL) AS "ATH MC",
    token_score AS "Score",
    community_added_via_dex AS "Dex Comm"
FROM tokens
WHERE
    admin_username IS NOT NULL
    AND CAST(ath_market_cap AS REAL) >= 10000
ORDER BY token_score ASC NULLS LAST;

-- Sheet 7: "Tokens - Failed (Under 10k)" - Tokens with ATH < 10k
CREATE OR REPLACE VIEW sheet_tokens_failed AS
SELECT
    admin_username AS "Admin Username",
    pool_address AS "Token Address",
    token_name AS "Name",
    token_symbol AS "Symbol",
    twitter_url AS "Community Link",
    created_at AS "Age",
    market_cap AS "Current MC",
    CAST(ath_market_cap AS REAL) AS "ATH MC",
    token_score AS "Score",
    community_added_via_dex AS "Dex Comm"
FROM tokens
WHERE
    admin_username IS NOT NULL
    AND (CAST(ath_market_cap AS REAL) < 10000 OR ath_market_cap IS NULL)
ORDER BY token_score ASC NULLS LAST;

-- Sheet 8: "Blacklist" - Direct table reference (no view needed)
-- Just use the blacklist table directly in Supabase Table Editor

-- ============================================================================
-- FUNCTIONS
-- ============================================================================

-- Main function: Update admin stats (call this periodically or after each token)
-- This calculates everything that was in the spreadsheet
CREATE OR REPLACE FUNCTION update_admin_stats()
RETURNS void AS $$
DECLARE
    admin_record RECORD;
    token_count INTEGER;
    score_counts JSON;
    avg_score REAL;
    tokens_10k INTEGER;
    winrate_val REAL;
BEGIN
    -- Loop through all tokens grouped by admin
    FOR admin_record IN
        SELECT
            admin_username,
            COUNT(*) as token_count,
            -- Get best score per community (one score per unique community)
            ARRAY_AGG(DISTINCT community_id) as communities
        FROM tokens
        WHERE admin_username IS NOT NULL
        GROUP BY admin_username
    LOOP
        -- Count scores per best token per community
        -- For each community, use the token with the best (lowest) score
        -- If no token reached 10k, count as score 6 (penalty)
        SELECT
            COALESCE(SUM(CASE WHEN best_score = 0 THEN 1 ELSE 0 END), 0) as s0,
            COALESCE(SUM(CASE WHEN best_score = 1 THEN 1 ELSE 0 END), 0) as s1,
            COALESCE(SUM(CASE WHEN best_score = 2 THEN 1 ELSE 0 END), 0) as s2,
            COALESCE(SUM(CASE WHEN best_score = 3 THEN 1 ELSE 0 END), 0) as s3,
            COALESCE(SUM(CASE WHEN best_score = 4 THEN 1 ELSE 0 END), 0) as s4,
            COALESCE(SUM(CASE WHEN best_score = 5 THEN 1 ELSE 0 END), 0) as s5,
            COALESCE(SUM(CASE WHEN best_score = 6 THEN 1 ELSE 0 END), 0) as s6,
            AVG(COALESCE(best_score, 6)) as avg_score,
            SUM(CASE WHEN best_score <= 4 THEN 1 ELSE 0 END) as tokens_10k
        INTO score_counts, avg_score, tokens_10k
        FROM (
            SELECT
                community_id,
                -- Best (lowest) score for tokens in this community that reached 10k
                -- If none reached 10k, score is 6
                COALESCE(MIN(CASE
                    WHEN CAST(ath_market_cap AS REAL) >= 10000 THEN token_score
                    ELSE NULL
                END), 6) as best_score
            FROM tokens
            WHERE admin_username = admin_record.admin_username
              AND community_id IS NOT NULL
            GROUP BY community_id
        ) sub;

        -- Calculate winrate: (communities with score <= 4) / total communities
        IF admin_record.token_count > 0 THEN
            winrate_val := tokens_10k::REAL / admin_record.token_count::REAL;
        ELSE
            winrate_val := 0;
        END IF;

        -- Insert or update admin stats
        INSERT INTO admins (
            admin_username,
            total_rating,
            tokens_launched,
            tokens_score_0, tokens_score_1, tokens_score_2,
            tokens_score_3, tokens_score_4, tokens_score_5, tokens_score_6,
            last_updated,
            total_tokens_created,
            winrate
        ) VALUES (
            admin_record.admin_username,
            avg_score,
            tokens_10k,
            score_counts->>'s0'::INTEGER,
            score_counts->>'s1'::INTEGER,
            score_counts->>'s2'::INTEGER,
            score_counts->>'s3'::INTEGER,
            score_counts->>'s4'::INTEGER,
            score_counts->>'s5'::INTEGER,
            score_counts->>'s6'::INTEGER,
            EXTRACT(EPOCH FROM NOW())::BIGINT,
            admin_record.token_count,
            winrate_val
        )
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
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- Convenience function: Update stats for a specific admin
CREATE OR REPLACE FUNCTION update_admin_stats_single(admin_username_input TEXT)
RETURNS void AS $$
BEGIN
    PERFORM update_admin_stats()
    WHERE admin_username = admin_username_input;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- CONDITIONAL FORMATTING NOTES
-- ============================================================================
-- Supabase Table Editor supports cell highlighting:
--
-- SCORE 0: #00C851 (Vibrant Green) - Excellent: Migrated + ATH >= $100k
-- SCORE 1: #90EE90 (Light Green)    - Very Good: Migrated fast + ATH < $100k
-- SCORE 2: #FFD700 (Yellow)         - Good: Migrated slow + ATH < $100k
-- SCORE 3: #FFA500 (Orange)         - Above Average: Not migrated + ATH >= $30k
-- SCORE 4: #FF6B6B (Light Red)      - Average: Not migrated + ATH $20k-$30k
-- SCORE 5: #FF0000 (Red)            - Below Average: Not migrated + ATH $10k-$20k
-- SCORE 6: #8B0000 (Dark Red)       - Poor: ATH < $10k (penalty)
--
-- To apply in Supabase Table Editor:
-- 1. Open table view
-- 2. Click column header
-- 3. Select "Conditional formatting"
-- 4. Add rules for each score value with corresponding colors
-- ============================================================================

-- ============================================================================
-- X Community Bot - Safe to add to EXISTING Supabase Project
-- Run this in Supabase SQL Editor for your existing project
-- ============================================================================

-- All tables use IF NOT EXISTS to avoid conflicts
-- All views use CREATE OR REPLACE VIEW to avoid conflicts
-- Functions use CREATE OR REPLACE FUNCTION to avoid conflicts

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
-- VIEWS (Your Spreadsheet Sheets)
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

-- ============================================================================
-- FUNCTIONS
-- ============================================================================

-- Main function: Update all admin stats
-- Run this after tokens are added/updated to refresh admin statistics
CREATE OR REPLACE FUNCTION update_admin_stats()
RETURNS void AS $$
DECLARE
    admin_record RECORD;
BEGIN
    -- Loop through all admins (tokens grouped by admin_username)
    FOR admin_record IN
        SELECT DISTINCT admin_username
        FROM tokens
        WHERE admin_username IS NOT NULL
    LOOP
        -- For each admin, calculate their stats
        INSERT INTO admins (
            admin_username,
            total_rating,
            tokens_launched,
            tokens_score_0, tokens_score_1, tokens_score_2,
            tokens_score_3, tokens_score_4, tokens_score_5, tokens_score_6,
            last_updated,
            total_tokens_created,
            winrate
        )
        SELECT
            admin_record.admin_username,
            -- Average score per community (best token per community)
            COALESCE(AVG(COALESCE(best_score, 6)), 0),
            -- Tokens with score <= 4 (reached 10k)
            COALESCE(SUM(CASE WHEN best_score <= 4 THEN 1 ELSE 0 END), 0),
            -- Score counts
            COALESCE(SUM(CASE WHEN best_score = 0 THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN best_score = 1 THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN best_score = 2 THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN best_score = 3 THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN best_score = 4 THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN best_score = 5 THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN best_score = 6 THEN 1 ELSE 0 END), 0),
            EXTRACT(EPOCH FROM NOW())::BIGINT,
            -- Total unique communities
            COUNT(DISTINCT community_id),
            -- Winrate: (communities with score <= 4) / total communities
            CASE
                WHEN COUNT(DISTINCT community_id) > 0 THEN
                    COALESCE(SUM(CASE WHEN best_score <= 4 THEN 1 ELSE 0 END), 0)::REAL / COUNT(DISTINCT community_id)
                ELSE 0
            END
        FROM (
            SELECT
                community_id,
                -- Best (lowest) score for tokens in this community
                COALESCE(MIN(CASE
                    WHEN CAST(ath_market_cap AS REAL) >= 10000 THEN token_score
                    ELSE NULL
                END), 6) as best_score
            FROM tokens
            WHERE admin_username = admin_record.admin_username
              AND community_id IS NOT NULL
            GROUP BY community_id
        ) sub
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

-- ============================================================================
-- HOW TO USE
-- ============================================================================
--
-- 1. Go to your Supabase project > SQL Editor
-- 2. Paste this entire script
-- 3. Click "Run"
-- 4. Tables and views will be created/updated safely
--
-- To update admin stats after running the bot:
--    SELECT update_admin_stats();
--
-- To view your data like a spreadsheet:
--    Go to Table Editor > Click on any "sheet_*" view
-- ============================================================================

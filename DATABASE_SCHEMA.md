# Xcomm Bot - Database Schema Documentation

## Overview

This database stores token and admin data for tracking Solana pump.fun/bags.fm tokens with X (Twitter) Community integration.

**Database File:** `tokens.db` (SQLite)
**Schema Version:** 1.0

---

## Tables

### 1. Tokens Table

Stores all tracked tokens with their market data, community info, and performance metrics.

#### Schema

| Column | Type | Primary Key | Nullable | Default | Description |
|--------|------|-------------|----------|---------|-------------|
| pool_address | TEXT | ✅ | ❌ | - | Unique pool identifier |
| base_token | TEXT | ❌ | ✅ | NULL | Token mint address |
| chain | TEXT | ❌ | ✅ | NULL | Blockchain (sol) |
| token_symbol | TEXT | ❌ | ✅ | NULL | Token ticker symbol |
| token_name | TEXT | ❌ | ✅ | NULL | Token display name |
| community_id | TEXT | ❌ | ✅ | NULL | X Community ID |
| admin_username | TEXT | ❌ | ✅ | NULL | Extracted admin username |
| twitter_url | TEXT | ❌ | ✅ | NULL | X Community URL |
| website_url | TEXT | ❌ | ✅ | NULL | Project website |
| market_cap | REAL | ❌ | ✅ | NULL | Current market cap (USD) |
| price_usd | REAL | ❌ | ✅ | NULL | Current price (USD) |
| liquid_usd | REAL | ❌ | ✅ | NULL | Liquidity (USD) |
| curve_percent | REAL | ❌ | ✅ | NULL | Bonding curve progress |
| holder_count | INTEGER | ❌ | ✅ | NULL | Number of holders |
| total_supply | TEXT | ❌ | ✅ | NULL | Total token supply |
| created_at | INTEGER | ❌ | ✅ | NULL | Unix timestamp of token creation |
| detected_at | INTEGER | ❌ | ✅ | NULL | Unix timestamp when bot detected token |
| price_change_percent | REAL | ❌ | ✅ | NULL | Price change percentage |
| total_volume | REAL | ❌ | ✅ | NULL | Total trading volume |
| buy_volume | REAL | ❌ | ✅ | NULL | Total buy volume |
| sell_volume | REAL | ❌ | ✅ | NULL | Total sell volume |
| trade_count | INTEGER | ❌ | ✅ | NULL | Total number of trades |
| buy_count | INTEGER | ❌ | ✅ | NULL | Number of buy trades |
| sell_count | INTEGER | ❌ | ✅ | NULL | Number of sell trades |
| ath_market_cap | TEXT | ❌ | ✅ | NULL | All-time high market cap |
| ath_detected_at | INTEGER | ❌ | ✅ | NULL | Unix timestamp of ATH |
| is_migrated | INTEGER | ❌ | ✅ | 0 | Migration status (0/1) |
| reached_final_stretch | INTEGER | ❌ | ✅ | 0 | Curve ≥65% flag (0/1) |
| reached_10k | INTEGER | ❌ | ✅ | 0 | ATH ≥10k flag (0/1) |
| last_updated | INTEGER | ❌ | ✅ | NULL | Last update timestamp |
| migrated_at | INTEGER | ❌ | ✅ | NULL | Migration timestamp |
| market_cap_at_migration | REAL | ❌ | ✅ | NULL | Market cap at migration |
| community_added_via_dex | INTEGER | ❌ | ✅ | 0 | DEX-paid community flag (0/1) |
| token_score | INTEGER | ❌ | ✅ | NULL | Performance score (0-6) |
| is_manipulated | INTEGER | ❌ | ✅ | 0 | Manipulation detected flag (0/1) |
| manipulation_reason | TEXT | ❌ | ✅ | NULL | Reason for manipulation flag |
| dev_wallet | TEXT | ❌ | ✅ | NULL | Developer wallet address |
| manipulation_checked | INTEGER | ❌ | ✅ | 0 | Manipulation analysis done (0/1) |
| platform | TEXT | ❌ | ✅ | 'pump.fun' | Platform identifier |

#### Token Scoring System (token_score)

| Score | Description | Criteria |
|-------|-------------|----------|
| 0 | Excellent | Migrated + ATH ≥ $100,000 |
| 1 | Very Good | Migrated + fast migration (≤2.5h) + ATH < $100k |
| 2 | Good | Migrated + slow migration (>2.5h) + ATH < $100k |
| 3 | Above Average | Not migrated + ATH ≥ $30,000 |
| 4 | Average | Not migrated + ATH $20k-$30k |
| 5 | Below Average | Not migrated + ATH $10k-$20k |
| 6 | Poor (Penalty) | ATH < $10,000 |

#### Platform Values

- `pump.fun` - pump.fun platform tokens
- `bags.fm` - bags.fm platform tokens

---

### 2. Admins Table

Stores administrator statistics calculated from their tokens' performance.

#### Schema

| Column | Type | Primary Key | Nullable | Default | Description |
|--------|------|-------------|----------|---------|-------------|
| admin_username | TEXT | ✅ | ❌ | - | Admin username (unique) |
| score | INTEGER | ❌ | ✅ | NULL | Legacy/deprecated score field |
| total_rating | REAL | ❌ | ✅ | NULL | Average token score (lower=better) |
| tokens_launched | INTEGER | ❌ | ✅ | 0 | Tokens with scores 0-5 (no penalties) |
| tokens_score_0 | INTEGER | ❌ | ✅ | 0 | Count of score 0 tokens |
| tokens_score_1 | INTEGER | ❌ | ✅ | 0 | Count of score 1 tokens |
| tokens_score_2 | INTEGER | ❌ | ✅ | 0 | Count of score 2 tokens |
| tokens_score_3 | INTEGER | ❌ | ✅ | 0 | Count of score 3 tokens |
| tokens_score_4 | INTEGER | ❌ | ✅ | 0 | Count of score 4 tokens |
| tokens_score_5 | INTEGER | ❌ | ✅ | 0 | Count of score 5 tokens |
| tokens_score_6 | INTEGER | ❌ | ✅ | 0 | Count of penalty (score 6) tokens |
| last_updated | INTEGER | ❌ | ✅ | NULL | Last stats update timestamp |
| total_tokens_created | INTEGER | ❌ | ✅ | 0 | Unique communities (one score per community) |
| winrate | REAL | ❌ | ✅ | NULL | Winning communities / total communities |
| dex_comm_added | INTEGER | ❌ | ✅ | 0 | DEX-paid communities count |

#### Admin Rating Calculation

```
total_rating = average(all token scores per unique community)

One score per unique community_id:
- Use the best (lowest) score for tokens with the same community
- Penalty score 6 for communities with no tokens reaching 10k

winrate = (communities with score ≤ 4) / total unique communities
```

---

## Indexes

Recommended indexes for performance:

```sql
-- Token lookups
CREATE INDEX IF NOT EXISTS idx_tokens_base_token ON tokens(base_token);
CREATE INDEX IF NOT EXISTS idx_tokens_admin_username ON tokens(admin_username);
CREATE INDEX IF NOT EXISTS idx_tokens_community_id ON tokens(community_id);
CREATE INDEX IF NOT EXISTS idx_tokens_is_migrated ON tokens(is_migrated);
CREATE INDEX IF NOT EXISTS idx_tokens_reached_10k ON tokens(reached_10k);
CREATE INDEX IF NOT EXISTS idx_tokens_platform ON tokens(platform);

-- Composite indexes for common queries
CREATE INDEX IF NOT EXISTS idx_tokens_admin_ath ON tokens(admin_username, ath_market_cap);
CREATE INDEX IF NOT EXISTS idx_tokens_community_ath ON tokens(community_id, ath_market_cap);
```

---

## Common Queries

### Get all tokens for an admin with scores

```sql
SELECT base_token, token_name, token_symbol, ath_market_cap,
       token_score, is_migrated, market_cap_at_migration
FROM tokens
WHERE admin_username = ?
ORDER BY CAST(ath_market_cap AS REAL) DESC;
```

### Get admin statistics

```sql
SELECT admin_username, total_rating, winrate, tokens_launched,
       total_tokens_created, tokens_score_0, tokens_score_1,
       tokens_score_2, tokens_score_3, tokens_score_4,
       tokens_score_5, tokens_score_6
FROM admins
WHERE admin_username = ?;
```

### Get tokens that reached 10k (for spreadsheet)

```sql
SELECT pool_address, base_token, admin_username, token_name,
       token_symbol, twitter_url, created_at, market_cap,
       ath_market_cap, token_score, community_added_via_dex
FROM tokens
WHERE reached_10k = 1 AND admin_username IS NOT NULL
ORDER BY admin_username ASC, CAST(ath_market_cap AS REAL) DESC;
```

### Get failed tokens (under 10k ATH)

```sql
SELECT * FROM tokens
WHERE admin_username IS NOT NULL
  AND CAST(ath_market_cap AS REAL) < 10000
ORDER BY admin_username, CAST(ath_market_cap AS REAL) DESC;
```

### Get migrated tokens

```sql
SELECT * FROM tokens
WHERE is_migrated = 1
ORDER BY migrated_at DESC;
```

### Count tokens by admin

```sql
SELECT admin_username,
       COUNT(*) as total_tokens,
       SUM(CASE WHEN reached_10k = 1 THEN 1 ELSE 0 END) as tokens_reached_10k,
       AVG(CAST(token_score AS REAL)) as avg_score
FROM tokens
WHERE admin_username IS NOT NULL
GROUP BY admin_username
ORDER BY avg_score ASC;
```

---

## Data Relationships

```
┌─────────────┐         ┌─────────────┐
│   tokens    │         │   admins    │
├─────────────┤         ├─────────────┤
│ pool_address│ (PK)    │admin_username│ (PK)
│ base_token  │         │total_rating │
│ admin_username├───────│winrate      │
│ community_id│         │tokens_launched│
│ token_score │         │...          │
│ ...         │         └─────────────┘
└─────────────┘
       │
       │ references via community_id
       ▼
┌─────────────┐
│ X Community │ (external API)
│ ID          │
└─────────────┘
```

---

## Migration Notes

### Schema Changes

When modifying the schema:

1. Create a new migration file in `src/database/migrations/`
2. Use transaction for safety
3. Test on backup database first
4. Document breaking changes

### Example Migration Pattern

```javascript
const migrate = (db) => {
  const addColumn = `
    ALTER TABLE tokens ADD COLUMN new_column TEXT DEFAULT NULL;
  `;
  db.exec(addColumn);
};

module.exports = { migrate, version: 2 };
```

---

## Google Sheets Integration

### Sheet to Column Mapping

**Admins Sheet:**
| Column | Database Field |
|--------|---------------|
| A | admin_username |
| B | total_rating |
| C | tokens_score_0 |
| D | tokens_score_1 |
| E | tokens_score_2 |
| F | tokens_score_3 |
| G | tokens_score_4 |
| H | tokens_score_5 |
| I | total_tokens_created |
| J | winrate (as percentage) |
| K | dex_comm_added |
| L | last_updated (formatted age) |

**Tokens Sheet:**
| Column | Database Field |
|--------|---------------|
| A | admin_username |
| B | base_token |
| C | token_name |
| D | token_symbol |
| E | twitter_url |
| F | created_at (formatted age) |
| G | market_cap |
| H | ath_market_cap |
| I | token_score |
| J | community_added_via_dex |

---

## Important Notes

1. **Timestamp Storage:** All timestamps stored as Unix epoch seconds (INTEGER)

2. **Market Cap as TEXT:** `ath_market_cap` is stored as TEXT to preserve precision. Cast to REAL when doing math.

3. **Unique Communities:** Admin stats count unique `community_id` values, not token count.

4. **Pool Address as Primary Key:** When a token migrates, `pool_address` changes. Use `base_token` to track across migrations.

5. **Platform Detection:** Platform determined by `factory` and `preFactory` fields:
   - pump.fun: `preFactory='pump'` and `factory='pump'`
   - bags.fm: `preFactory='dbc'` and `factory='meteora'`

6. **Migration Status:** Token is migrated when:
   - pump.fun: `factory='pumpamm'`
   - bags.fm: `factory='meteora'`

---

## Database File Location

**Development:** `./tokens.db`
**Production:** `./tokens.db` (same file reused)

**Backup before migration:**
```bash
cp tokens.db tokens.db.backup
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2024-11 | Initial schema documentation |

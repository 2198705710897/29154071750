# XComm Spreadsheet - Complete Structure Specification

This document describes the complete structure of the Google Sheets spreadsheet for replication.

---

## Spreadsheet Overview

**Total Sheets:** 8
**Purpose:** Track cryptocurrency token launches on Solana (pump.fun and bags.fm platforms) and rank admin performance

---

## Sheet 1: "Admins"

### Layout
| Row | Content |
|-----|---------|
| 1 | Column Headers |
| 2+ | Admin data rows (one per admin) |

### Columns (A-L)

| Column | Header | Data Type | Description |
|--------|--------|-----------|-------------|
| A | Admin Username | Text | X/Twitter community admin username (e.g., "@crypto_admin") |
| B | Total Rating | Number | Average score of all tokens by this admin (0-5, lower is better) |
| C | Score 0 | Number | Count of tokens with ATH >= 100k AND migrated |
| D | Score 1 | Number | Count of tokens that migrated fast (<=2.5 hours) AND ATH < 100k |
| E | Score 2 | Number | Count of tokens that migrated slowly (>2.5 hours) AND ATH < 100k |
| F | Score 3 | Number | Count of tokens that reached ATH >= 30k but did NOT migrate |
| G | Score 4 | Number | Count of tokens that reached ATH 20k-30k but did NOT migrate |
| H | Score 5 | Number | Count of tokens that reached ATH 10k-20k but did NOT migrate |
| I | Total Tokens | Number | Total number of tokens created by this admin |
| J | Winrate % | Number | Percentage of tokens that reached 10k (format: 0-100) |
| K | Dex Comm Added | Number | Count of tokens added via Dex Communities feature |
| L | Last Updated | Date/Time | Timestamp of last data update (format: MM/DD/YYYY HH:MM:SS) |

### Sorting
- **Primary:** Column B (Total Rating) - Ascending (lower ratings first)
- **Secondary:** None

### Conditional Formatting (Columns C-H)
- Score 0 cells (Column C): Background = #00C851 (Vibrant Green)
- Score 1 cells (Column D): Background = #90EE90 (Light Green)
- Score 2 cells (Column E): Background = #FFD700 (Yellow)
- Score 3 cells (Column F): Background = #FFA500 (Orange)
- Score 4 cells (Column G): Background = #FF6B6B (Light Red)
- Score 5 cells (Column H): Background = #FF0000 (Red)
- **Note:** Formatting applies only when cell value > 0

### Data Source
Rows are populated from an internal database tracking all admins who have created at least one token.

---

## Sheet 2: "Admins - Sorted by Winrate"

### Layout
| Row | Content |
|-----|---------|
| 1 | Column Headers (identical to Sheet 1) |
| 2+ | Same admin data as Sheet 1, different sort order |

### Columns (A-L)
Identical to "Admins" sheet (see above)

### Sorting
- **Primary:** Column J (Winrate %) - Descending (highest winrate first)

### Data Relationship
This sheet contains the **exact same data rows** as Sheet 1, only the sort order differs.

---

## Sheet 3: "Admins - Sorted by Score"

### Layout
| Row | Content |
|-----|---------|
| 1 | Column Headers (identical to Sheet 1) |
| 2+ | Same admin data as Sheet 1, different sort order |

### Columns (A-L)
Identical to "Admins" sheet (see above)

### Sorting
- **Primary:** Column B (Total Rating) - Ascending (best ratings first)
- **Note:** This sort is effectively the same as Sheet 1

### Data Relationship
This sheet contains the **exact same data rows** as Sheet 1, only the sort order differs.

---

## Sheet 4: "Admins - High Performers"

### Layout
| Row | Content |
|-----|---------|
| 1 | Column Headers (identical to Sheet 1) |
| 2+ | Filtered subset of admin data |

### Columns (A-L)
Identical to "Admins" sheet (see above)

### Filtering Criteria
An admin is included in this sheet **ONLY if ALL** of the following conditions are met:
1. Total Rating (Column B) >= 0 AND <= 2.5
2. (Score 0 + Score 1 + Score 2) >= 3 (at least 3 "good" tokens)
3. Winrate % (Column J) >= 70% AND <= 90%
4. Admin Username (Column A) is **NOT** found in the "Blacklist" sheet

### Conditional Formatting
Identical to "Admins" sheet (Columns C-H colored by score)

### Sorting
- **Primary:** Column B (Total Rating) - Ascending

### Data Relationship
This is a **filtered subset** of admins from Sheet 1. It excludes:
- Low-performing admins (Total Rating > 2.5)
- Admins with few good tokens (< 3 scores 0/1/2)
- Admins with too high winrate (> 90%, suspicious)
- Blacklisted admins

---

## Sheet 5: "Tokens - Sorted by Admin"

### Layout
| Row | Content |
|-----|---------|
| 1 | Column Headers |
| 2+ | Token data rows (one per token that reached 10k MC) |

### Columns (A-J)

| Column | Header | Data Type | Description |
|--------|--------|-----------|-------------|
| A | Admin Username | Text | X/Twitter admin who launched this token |
| B | Token Address | Text | Solana pool address (base58 encoded) |
| C | Name | Text | Token name |
| D | Symbol | Text | Token ticker/symbol (e.g., "DOGE") |
| E | Community Link | URL | Full X/Twitter community URL (e.g., "https://x.com/i/communities/12345") |
| F | Age | Date/Time | Token creation timestamp (format: MM/DD/YYYY HH:MM:SS) |
| G | Current MC | Number | Current market cap in USD (e.g., 15000) |
| H | ATH MC | Number | All-time high market cap in USD (e.g., 125000) |
| I | Score | Number | Token score 0-6 (see scoring section below) |
| J | Dex Comm | Number | 1 if added via Dex Communities, 0 otherwise |

### Sorting
- **Primary:** Column A (Admin Username) - Ascending A-Z
- **Secondary:** Column I (Score) - Ascending (best scores first)

### Conditional Formatting (Column I)
- Score 0: Background = #00C851 (Vibrant Green)
- Score 1: Background = #90EE90 (Light Green)
- Score 2: Background = #FFD700 (Yellow)
- Score 3: Background = #FFA500 (Orange)
- Score 4: Background = #FF6B6B (Light Red)
- Score 5: Background = #FF0000 (Red)
- Score 6: Background = #8B0000 (Dark Red)

### Inclusion Criteria
Tokens appear in this sheet **ONLY if** ATH MC (Column H) >= 10,000

---

## Sheet 6: "Tokens - Sorted by Score"

### Layout
| Row | Content |
|-----|---------|
| 1 | Column Headers (identical to Sheet 5) |
| 2+ | Same token data as Sheet 5, different sort order |

### Columns (A-J)
Identical to "Tokens - Sorted by Admin" sheet (see above)

### Sorting
- **Primary:** Column I (Score) - Ascending (score 0 first, then 1, 2, etc.)

### Conditional Formatting
Identical to Sheet 5 (Column I colored by score)

### Data Relationship
This sheet contains the **exact same data rows** as Sheet 5, only the sort order differs.

---

## Sheet 7: "Tokens - Failed (Under 10k)"

### Layout
| Row | Content |
|-----|---------|
| 1 | Column Headers (identical to Sheet 5) |
| 2+ | Token data rows for failed tokens |

### Columns (A-J)
Identical to "Tokens - Sorted by Admin" sheet (see above)

### Sorting
- **Primary:** Column I (Score) - Ascending

### Conditional Formatting
Identical to Sheet 5 (Column I colored by score)

### Inclusion Criteria
Tokens appear in this sheet **ONLY if** ATH MC (Column H) < 10,000

### Data Relationship
These tokens **mutually exclude** tokens in Sheets 5 and 6. A token is either:
- In Sheets 5/6 if ATH >= 10k
- In Sheet 7 if ATH < 10k

---

## Sheet 8: "Blacklist"

### Layout
| Row | Content |
|-----|---------|
| 1 | Column Header |
| 2+ | Blacklisted admin usernames |

### Columns (A)

| Column | Header | Data Type | Description |
|--------|--------|-----------|-------------|
| A | Admin Username | Text | Admin username to exclude from "High Performers" sheet |

### Sorting
None (user-maintained list)

### Purpose
Admins listed here are **automatically excluded** from the "Admins - High Performers" sheet, regardless of their stats.

---

## Token Scoring System

This scoring system applies to the Score column (Column I) in all token sheets.

| Score | ATH Market Cap | Migrated Status | Migration Speed |
|-------|----------------|-----------------|-----------------|
| **0** | >= 100k | Yes | Any |
| **1** | < 100k | Yes | Fast (<= 2.5 hours from creation) |
| **2** | < 100k | Yes | Slow (> 2.5 hours from creation) |
| **3** | >= 30k | No | N/A |
| **4** | 20k - 30k | No | N/A |
| **5** | 10k - 20k | No | N/A |
| **6** | < 10k | N/A | N/A (penalty score) |

### Migration Definition
A token is considered "migrated" when it transitions from bonding curve to DEX liquidity:
- **pump.fun:** factory changes from "pump" to "pumpamm" AND preFactory = "pump" AND curvePercent = 0
- **bags.fm:** factory changes from "dbc" to "meteora" AND preFactory = "dbc" AND curvePercent = 0

### Important Rules
1. **ATH < 10k = Score 6 always** (cannot have scores 0-5)
2. **Migration only counts if ATH >= 10k**
3. **Migration speed is calculated as:** migrated_at timestamp - created_at timestamp

---

## Sheet Relationships Summary

```
┌─────────────────────────────────────────────────────────────────┐
│                    SPREADSHEET STRUCTURE                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ADMIN SHEETS (same data, different views)                      │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────┐ │
│  │ 1. Admins        │  │ 2. Admins -      │  │ 3. Admins -  │ │
│  │    (by Rating)   │  │    by Winrate    │  │    by Score  │ │
│  └────────┬─────────┘  └────────┬─────────┘  └──────┬───────┘ │
│           │                     │                    │         │
│           └─────────────────────┼────────────────────┘         │
│                                 │                              │
│                        ┌────────▼────────┐                    │
│                        │ 4. High         │                    │
│                        │    Performers   │◄───── Filtered     │
│                        │    (subset)     │       from above    │
│                        └─────────────────┘       + excludes    │
│                                                  blacklist     │
│                                                                 │
│  TOKEN SHEETS (mutually exclusive by ATH)                       │
│  ┌──────────────────────────────┐  ┌──────────────────────┐   │
│  │ 5. Tokens - Sorted by Admin  │  │ 6. Tokens - Sorted   │   │
│  │    (ATH >= 10k)              │  │    by Score (ATH>=10k)│   │
│  └──────────┬───────────────────┘  └──────────┬───────────┘   │
│             │                                 │               │
│             └─────────────┬───────────────────┘               │
│                           │                                   │
│                    ┌──────▼──────────┐                        │
│                    │ 7. Failed       │                        │
│                    │    (ATH < 10k)  │                        │
│                    └─────────────────┘                        │
│                                                                 │
│  REFERENCE SHEET                                                │
│  ┌──────────────────────────────┐                             │
│  │ 8. Blacklist                 │                             │
│  └──────────────────────────────┘                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## Data Flow

1. **Token Detection** → Tokens are discovered and tracked
2. **Scoring** → Each token receives a score (0-6) based on ATH and migration
3. **Admin Aggregation** → Token scores are aggregated per admin (counts of each score)
4. **Rating Calculation** → Total Rating = weighted average of all admin's tokens
5. **Winrate Calculation** → Winrate = (Tokens with ATH >= 10k) / (Total Tokens Created)
6. **Sheet Population** → Data is written to all 8 sheets with appropriate sorting/filtering

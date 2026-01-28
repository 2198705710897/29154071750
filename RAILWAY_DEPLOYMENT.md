# Railway Deployment Guide

## Files to Commit to GitHub

These files should be in your git repo (they are NOT ignored):

```
✅ Cargo.toml
✅ Cargo.lock (uncomment in .gitignore if needed)
✅ src/
✅ Dockerfile
✅ railway.toml
✅ .env.example (safe - contains examples, not real secrets)
✅ RAILWAY_ENVS.txt (safe - contains placeholders)
✅ supabase/complete_schema.sql (safe - just schema)
```

## Files to NEVER Commit (Already in .gitignore)

```
❌ .env (contains REAL secrets)
❌ config.toml (contains REAL X API tokens/cookies)
❌ tokens.db (local database)
❌ target/ (build artifacts)
```

---

## Step-by-Step Deployment

### 1. Set Up Supabase Database

1. Go to [supabase.com](https://supabase.com) and create a new project
2. Go to **SQL Editor** in Supabase
3. Paste the contents of `supabase/complete_schema.sql`
4. Click **Run** to create all tables and views
5. Go to **Project Settings** > **Database**
6. Copy the **Connection string** (URI format):
   ```
   postgres://postgres:[YOUR-PASSWORD]@db.[PROJECT-REF].supabase.co:5432/postgres
   ```

### 2. Push Code to GitHub

```bash
# From your project directory
git add .
git commit -m "Add Supabase and Railway deployment config"
git push origin main
```

### 3. Deploy to Railway

1. Go to [railway.app](https://railway.app)
2. Click **New Project** > **Deploy from GitHub repo**
3. Select your repository
4. Railway will auto-detect it's a Rust project
5. Click **Deploy**

### 4. Add Environment Variables in Railway

1. In your Railway service, go to **Settings** > **Variables**
2. For each variable in `RAILWAY_ENVS.txt`:
   - Click **New Variable**
   - Name: (e.g., `DATABASE_URL`)
   - Value: (paste your actual value)
   - Click **Add Variable**

**Required Variables:**
| Variable | Where to Get It |
|----------|----------------|
| `DATABASE_URL` | Supabase Project Settings > Database > Connection string |
| `X_API_BEARER_TOKEN` | X.com DevTools > Network > Request Headers |
| `X_API_CSRF_TOKEN` | X.com DevTools > Network > Request Headers |
| `X_API_COOKIE` | X.com DevTools > Network > Request Headers |

### 5. Redeploy

After adding variables, Railway will automatically redeploy.
- Click the **Logs** tab to see if your bot started successfully
- You should see: `✓ Connected to WebSocket`

---

## Verifying It Works

In Railway logs, you should see:

```
🚀 X Community Bot - Phase 2: WebSocket + Database + Admin Fetch
Filter: pump.fun tokens with X Community only
✓ Configuration loaded
✓ Database connected
✓ X API client initialized
🕐 Bot started at: [timestamp]
⏱ Only accepting tokens created after bot start time
✓ Connected to WebSocket
✓ Subscribed to flash pool updates
```

---

## Accessing Your Data (Supabase as Spreadsheet)

1. Go to your Supabase project
2. Click **Table Editor**
3. You'll see:
   - `tokens` table (all your tokens)
   - `admins` table (aggregated stats)
   - `blacklist` table
   - `sheet_*` views (your spreadsheet views!)

### Using the Views

Open any view to see data like your spreadsheet:
- `sheet_admins` - All admins sorted by rating
- `sheet_admins_high_performers` - Filtered high performers
- `sheet_tokens_by_admin` - Successful tokens (ATH >= 10k)
- `sheet_tokens_failed` - Failed tokens (ATH < 10k)

### Updating Admin Stats

In Supabase SQL Editor, run:
```sql
SELECT update_admin_stats();
```

This recalculates all admin statistics based on current token data.

---

## Troubleshooting

### Bot Won't Start
- Check Railway logs for errors
- Verify `DATABASE_URL` is correct (includes password)
- Verify X API credentials are current (they expire!)

### Database Errors
- Ensure you ran `complete_schema.sql` in Supabase SQL Editor
- Check Supabase logs: Database > Logs

### Rate Limiting (429 errors)
- X API has rate limits - this is normal
- Consider reducing bot check frequency or using multiple X accounts

---

## Local Development with Supabase

To test locally before deploying:

```bash
# Install dependencies
cargo install sqlx-cli

# Set environment variables
cp .env.example .env
# Edit .env with your Supabase DATABASE_URL

# Run locally
cargo run --release --features postgres
```

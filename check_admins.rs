use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("tokens.db")?;

    // Check tokens with admin username
    let mut stmt = conn.prepare(
        "SELECT pool_address, token_symbol, admin_username, community_id, datetime(detected_at, 'unixepoch')
         FROM tokens WHERE admin_username IS NOT NULL
         ORDER BY detected_at DESC
         LIMIT 20"
    )?;

    println!("Tokens with admin username:");
    println!("{:=<100}", "");

    let token_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<String>>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;

    for token in token_iter {
        let (pool_address, symbol, admin, community, detected) = token?;
        println!("Symbol: {:12} | Admin: @{:20} | Community: {:20} | Detected: {}", symbol, admin, community.unwrap_or_default(), detected);
        println!("  Pool: {}", pool_address);
        println!();
    }

    // Count total tokens with and without admin
    let with_admin: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tokens WHERE admin_username IS NOT NULL",
        [],
        |row| row.get(0)
    )?;

    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tokens",
        [],
        |row| row.get(0)
    )?;

    println!("{:=<100}", "");
    println!("Summary: {} / {} tokens have admin_username ({}%)",
        with_admin, total, if total > 0 { with_admin * 100 / total } else { 0 });

    Ok(())
}

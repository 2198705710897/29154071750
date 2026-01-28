use crate::ws::message::PoolParams;

pub fn is_pump_fun_token(pool: &PoolParams) -> bool {
    // pump.fun: factory="pump"
    if pool.factory == "pump" {
        return true;
    }
    // Migrated pump.fun: preFactory="pump" AND factory="pumpamm"
    if pool.pre_factory.as_deref() == Some("pump") && pool.factory == "pumpamm" {
        return true;
    }
    false
}

pub fn has_x_community(pool: &PoolParams) -> bool {
    let social = pool.base_token_info.social.as_ref();

    // Check both twitter and website fields for X community links
    let twitter_has_community = social
        .and_then(|s| s.twitter.as_ref())
        .map(|url| url.contains("x.com/i/communities/"))
        .unwrap_or(false);

    let website_has_community = social
        .and_then(|s| s.website.as_ref())
        .map(|url| url.contains("x.com/i/communities/"))
        .unwrap_or(false);

    twitter_has_community || website_has_community
}

pub fn should_accept_message(pool: &PoolParams, bot_start_time: i64) -> bool {
    let is_pump = is_pump_fun_token(pool);
    let has_community = has_x_community(pool);
    let is_new = pool.created_at >= bot_start_time;

    match (is_pump, has_community, is_new) {
        (true, true, true) => true,
        (true, true, false) => {
            tracing::debug!(
                "✗ REJECTED (old token, created before bot start): {} ({}) - created_at={}, bot_start={}",
                pool.base_token_info.symbol,
                pool.pool_address,
                pool.created_at,
                bot_start_time
            );
            false
        }
        (true, false, _) => {
            tracing::debug!(
                "✗ REJECTED (no X community): {} ({})",
                pool.base_token_info.symbol,
                pool.pool_address
            );
            false
        }
        (false, _, _) => {
            tracing::debug!(
                "✗ REJECTED (not pump.fun): {} ({}) - factory={}, preFactory={:?}",
                pool.base_token_info.symbol,
                pool.pool_address,
                pool.factory,
                pool.pre_factory
            );
            false
        }
    }
}

use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bearer_token = "AAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs=1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA";
    let csrf_token = "cca5a4392dce16c75000146cc39579bef058159d64a8f67f2e0bd2de64950badb9a7c5d4554973fafc3694af83988c18a1f70c8f64da77a7f6f957ed8b92468634346c54bbd4e893a807a793c4506f93";
    let cookie = "__cuid=3e7e8cd06b4a463ebbc4f24f61cdb3ed; lang=en; kdt=lp7zRM6v5EsLv4KhRgiWfQyJzIal0Cehsz6SNbFV; g_state={\"i_l\":0,\"i_ll\":1761619818175,\"i_b\":\"xgX6acCYT6CK41PT+cDj+Uhm9l+LoNOYVASXMUHN7Ys\"}; dnt=1; auth_multi=\"1982984360778412032:411de002f9125f056da600b2ec0bf088818af5e7\"; auth_token=e7a54f834f495221d1c282656fa151e0e27d0f22; guest_id=v1%3A176187697007227146; twid=u%3D1795158587016138752; ct0=cca5a4392dce16c75000146cc39579bef058159d64a8f67f2e0bd2de64950badb9a7c5d4554973fafc3694af83988c18a1f70c8f64da77a7f6f957ed8b92468634346c54bbd4e893a807a793c4506f93; d_prefs=MToxLGNvbnNlbnRfdmVyc2lvbjoyLHRleHRfdmVyc2lvbjoxMDAw; guest_id_ads=v1%3A176187697007227146; guest_id_marketing=v1%3A176187697007227146; personalization_id=\"v1_1sMGc6gzlkqb0p+SJetcaA==\"; cf_clearance=d4rZt4RaDkDluPWbr8Kn2RttkhY.UoI1bzPrZHO5qic-1764703794-1.2.1.1-B.rDk309QR2nNP8bjNPYXVMwEJm9oehi9XeH4iiSvy0.tbsIGXGP1k09nc1kNph4ZjAOQkXAajA84KOcRbhCkbJYSkU03fuIxGUj9AKwH09ypdkJaUxXZtgYiBDlcUaK7YeiENHUA2WSEoB8kljftG9lFs06qdgmI7EjdOar1diM.cz0bGl3diQbFk92jByHNLmo9NtW0SDrkK6SQg_TaDMIOkvHyyTAo8K.SaKzveg";

    println!("Testing X API...");
    println!("Bearer token length: {}", bearer_token.len());
    println!("CSRF token length: {}", csrf_token.len());
    println!("Cookie length: {}", cookie.len());

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .http1_only()
        .build()?;

    let url = "https://x.com/i/api/graphql/uBpODvS60xZ1q2L88d-W2A/CommunityQuery";
    let variables = r#"{"communityId":"2015955517617610916"}"#;
    let features = r#"{"c9s_list_members_action_api_enabled":false,"c9s_superc9s_indication_enabled":false}"#;

    let response = client
        .get(url)
        .query(&[("variables", variables)])
        .query(&[("features", features)])
        .header("authorization", format!("Bearer {}", bearer_token))
        .header("content-type", "application/json")
        .header("x-csrf-token", csrf_token)
        .header("cookie", cookie)
        .header("x-twitter-active-user", "yes")
        .header("x-twitter-auth-type", "OAuth2Session")
        .header("x-twitter-client-language", "en")
        .header("accept", "*/*")
        .header("referer", "https://x.com/i/communities/2015955517617610916")
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    println!("\nStatus: {}", status);
    println!("Response (first 500 chars): {}...", &body.chars().take(500).collect::<String>());

    Ok(())
}

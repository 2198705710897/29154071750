use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://x.com/i/api/graphql/uBpODvS60xZ1q2L88d-W2A/CommunityQuery?variables=%7B%22communityId%22%3A%222014753966245216598%22%7D&features=%7B%22c9s_list_members_action_api_enabled%22%3Afalse%2C%22c9s_superc9s_indication_enabled%22%3Afalse%7D";

    let client = reqwest::Client::builder()
        .build()?;

    let response = client
        .get(url)
        .header("accept", "*/*")
        .header("accept-language", "en-US,en;q=0.9")
        .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .header("content-type", "application/json")
        .header("cookie", "__cuid=3e7e8cd06b4a463ebbc4f24f61cdb3ed; lang=en; kdt=lp7zRM6v5EsLv4KhRgiWfQyJzIal0Cehsz6SNbFV; dnt=1; d_prefs=MToxLGNvbnNlbnRfdmVyc2lvbjoyLHRleHRfdmVyc2lvbjoxMDAw; guest_id_ads=v1%3A176187697007227146; guest_id_marketing=v1%3A176187697007227146; personalization_id=\"v1_1sMGc6gzlkqb0p+SJetcaA==\"; cf_clearance=d4rZt4RaDkDluPWbr8Kn2RttkhY.UoI1bzPrZHO5qic-1764703794-1.2.1.1-B.rDk309QR2nNP8bjNPYXVMwEJm9oehi9XeH4iiSvy0.tbsIGXGP1k09nc1kNph4ZjAOQkXAajA84KOcRbhCkbJYSkU03fuIxGUj9AKwH09ypdkJaUxXZtgYiBDlcUaK7YeiENHUA2WSEoB8kljftG9lFs06qdgmI7EjdOar1diM.cz0bGl3diQbFk92jByHNLmo9NtW0SDrkK6SQg_TaDMIOkvHyyTAo8K.SaKzveg; g_state={\"i_l\":0,\"i_ll\":1769610896203,\"i_b\":\"eDe/1k1fU7xqcK/EGQfE5q1kmcxMN2OzbXaZ7JL34l0\",\"i_e\":{\"enable_itp_optimization\":3}}; auth_multi=\"1982984360778412032:411de002f9125f056da600b2ec0bf088818af5e7\"; auth_token=3557d59f08d32c078aba6fdbc90221092d345a8b; guest_id=v1%3A176961091850408579; twid=u%3D1795158587016138752; ct0=cabcf23c3fe8021948e04cbaefb70ff7b0f8981350f2835ee518570ee3c40c7005a594d5615120f1727fcf6f0894662f8559c8141a6e41110647f5361879ef30cc1cb597350bc326dcd6499c05a02618; __cf_bm=XGWSErEOqCd7J7udM1xv9tMfKEYo3M04HTQjYNz68LI-1769613093.358372-1.0.1.1-mlXqK3M7GRIZzpPu2DSB9IodczOqk6P142ADJ11f95Vfggg.cKsa31_gvErewgolFxsJdirddqChpQY2598uRaScyub2kU61IKSDDIJjHatJm_0XyuvZm4lfCBNxybDT")
        .header("priority", "u=1, i")
        .header("referer", "https://x.com/i/communities/2014753966245216598")
        .header("sec-ch-ua", "\"Google Chrome\";v=\"143\", \"Chromium\";v=\"143\", \"Not A(Brand\";v=\"24\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"Windows\"")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-origin")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36")
        .header("x-client-transaction-id", "5BtAfdlByWW72g6LEDh+A7DWLgUBZp5evelFguHUAX3hmVddswcrpUJo7/UdhBuCwFQnzuHaTJPPB870t2VG3wZIoscA5w")
        .header("x-csrf-token", "cabcf23c3fe8021948e04cbaefb70ff7b0f8981350f2835ee518570ee3c40c7005a594d5615120f1727fcf6f0894662f8559c8141a6e41110647f5361879ef30cc1cb597350bc326dcd6499c05a02618")
        .header("x-twitter-active-user", "yes")
        .header("x-twitter-auth-type", "OAuth2Session")
        .header("x-twitter-client-language", "en")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());

    let body = response.text().await?;
    println!("\nResponse Body:\n{}", body);

    Ok(())
}

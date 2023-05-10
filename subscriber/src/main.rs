use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, ClientId, ClientSecret, TokenResponse,
    TokenUrl,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let auth_client = BasicClient::new(
        ClientId::new("fredrik-thesis".to_string()),
        Some(ClientSecret::new(
            "a30a21b446974483a21b3ef62cc49d72".to_string(),
        )),
        AuthUrl::new(
            "https://ckey.open5glab.net/access/realms/nef/protocol/openid-connect/token"
                .to_string(),
        )?,
        Some(TokenUrl::new(
            "https://ckey.open5glab.net/access/realms/nef/protocol/openid-connect/token"
                .to_string(),
        )?),
    );

    let token_result = auth_client
        .exchange_client_credentials()
        .request_async(async_http_client)
        .await?;

    let token = token_result.access_token().secret();
    // let body = json!({
    //     "msisdn": "36204568701",
    //     "notificationDestination": "http://172.104.143.111:8789/",
    //     "monitoringType": "PDN_CONNECTIVITY_STATUS",
    //     "maximumNumberOfReports": 10
    // });

    let body = json!({
        "analyEventsSubs" : [{
        "analyEvent": "UE_MOBILITY",
        "tgtUe": {
            "gpsi" : "msisdn-36204568701"
        },
        "suppFeat": "*"}],
        "notifUri": "http://172.104.143.111:8789/",
        "notifId": "fredrik-thesis-test-1"
    });

    // let res = client
    //     .post("https://nef.open5glab.net/3gpp-monitoring-event/v1/fredrik-thesis/subscriptions")
    //     .bearer_auth(token)
    //     .json(&body)
    //     .send()
    //     .await?;

    let res = client
        .post("https://nef.open5glab.net/3gpp-analyticsexposure/v1/fredrik-thesis/subscriptions")
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("{}", res.text().await?);

    Ok(())
}

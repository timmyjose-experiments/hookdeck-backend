use crate::error::Result;
use actix_web::{
    get, post,
    web::{self, Json},
};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    status: String,
}

#[get("/status")]
pub(crate) async fn get_status() -> Result<Json<StatusResponse>> {
    Ok(web::Json(StatusResponse {
        status: "OK".to_owned(),
    }))
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookRequest {
    pub reference_id: String,
    pub event_type: String,
    pub status: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {}

#[post("/ramps/unlimit/webhook")]
pub(crate) async fn unlimit_webhook_handler(
    body: web::Json<WebhookRequest>,
) -> Result<Json<WebhookResponse>> {
    info!("Received request with body: {body:#?}");

    Ok(web::Json(WebhookResponse {}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_webhook_request_payload() -> eyre::Result<()> {
        let payload = r#"
      {
        "referenceId": "submissionId",
        "eventType": "KYC",
        "status": "IN_REVIEW",
        "metadata": {
          "id": "1a8b06c7-63ba-412a-ae4e-108f7ce4588d",
          "customerEmail": "d.dadkhoo@unlimit.com",
          "customerId": "b80ae303-d086-4ae8-beb5-da9e4306aaad",
          "kycLevel": ["Level 1", "Level 2", "Level 3"],
          "createdAt": 1699589563
        }
    }"#;

        let webhook_req: WebhookRequest = serde_json::from_str(payload)?;
        assert_eq!(
            webhook_req,
            WebhookRequest {
                reference_id: "submissionId".to_string(),
                event_type: "KYC".to_string(),
                status: "IN_REVIEW".to_string(),
                metadata: json!({
                  "id": "1a8b06c7-63ba-412a-ae4e-108f7ce4588d",
                  "customerEmail": "d.dadkhoo@unlimit.com",
                  "customerId":  "b80ae303-d086-4ae8-beb5-da9e4306aaad",
                  "kycLevel": ["Level 1", "Level 2", "Level 3"] ,
                  "createdAt": 1699589563
                })
            }
        );

        Ok(())
    }
}

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
pub struct UnlimitWebhookRequest {
    pub reference_id: String,
    pub event_type: String,
    pub status: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct UnlimitWebhookResponse {}

#[post("/ramps/unlimit/webhook")]
pub(crate) async fn unlimit_webhook_handler(
    body: web::Json<UnlimitWebhookRequest>,
) -> Result<Json<UnlimitWebhookResponse>> {
    info!("Received Unlimit webhook request with body: {body:#?}");
    Ok(web::Json(UnlimitWebhookResponse {}))
}

#[derive(Debug, Deserialize)]
pub struct PersonaWebhookRequest {
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct PersonaWebhookResponse {}

#[post("/ramps/persona/webhook")]
pub(crate) async fn persona_webhook_handler(
    body: web::Json<PersonaWebhookRequest>,
) -> Result<Json<PersonaWebhookResponse>> {
    info!("Received Persona webhook request with body: {body:#?}");
    Ok(web::Json(PersonaWebhookResponse {}))
}

#[derive(Debug, Deserialize)]
pub struct SumsubWebhookRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SumsubWebhookResponse {}

#[post("/ramps/sumsub/webhook")]
pub(crate) async fn sumsub_webhook_handler(
    body: web::Json<SumsubWebhookRequest>,
) -> Result<Json<SumsubWebhookResponse>> {
    info!("Received Sumsub webhook request with body: {body:#?}");
    Ok(web::Json(SumsubWebhookResponse {}))
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

        let webhook_req: UnlimitWebhookRequest = serde_json::from_str(payload)?;
        assert_eq!(
            webhook_req,
            UnlimitWebhookRequest {
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

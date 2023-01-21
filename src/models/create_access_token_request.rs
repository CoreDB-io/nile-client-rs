/*
 * Nile API
 *
 * Making SaaS chill.
 *
 * The version of the OpenAPI document: 0.1.0-fdd7cd5
 * Contact: support@thenile.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateAccessTokenRequest {
    /// The human-friendly label of the access token
    #[serde(rename = "label")]
    pub label: String,
    /// The intended use of the token
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Arbitrary metadata.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl CreateAccessTokenRequest {
    pub fn new(label: String) -> CreateAccessTokenRequest {
        CreateAccessTokenRequest {
            label,
            description: None,
            metadata: None,
        }
    }
}


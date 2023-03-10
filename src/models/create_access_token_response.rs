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
pub struct CreateAccessTokenResponse {
    /// The secret key to use for authentication
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "token_info", skip_serializing_if = "Option::is_none")]
    pub token_info: Option<Box<crate::models::AccessTokenInfo>>,
}

impl CreateAccessTokenResponse {
    pub fn new(token: String) -> CreateAccessTokenResponse {
        CreateAccessTokenResponse {
            token,
            token_info: None,
        }
    }
}

use serde::{Deserialize, Serialize};

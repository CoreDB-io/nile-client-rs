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
pub struct UpdateInstanceRequest {
    /// An *instance* of a JSON Schema
    #[serde(rename = "properties")]
    pub properties: serde_json::Value,
}

impl UpdateInstanceRequest {
    pub fn new(properties: serde_json::Value) -> UpdateInstanceRequest {
        UpdateInstanceRequest { properties }
    }
}

use serde::{Deserialize, Serialize};

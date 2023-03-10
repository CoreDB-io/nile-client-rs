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
pub struct Entity {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "name")]
    pub name: String,
    /// A JSON Schema
    #[serde(rename = "schema")]
    pub schema: serde_json::Value,
}

impl Entity {
    pub fn new(id: String, r#type: RHashType, name: String, schema: serde_json::Value) -> Entity {
        Entity {
            id,
            created: None,
            updated: None,
            seq: None,
            r#type,
            name,
            schema,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "entity")]
    Entity,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Entity
    }
}

use serde::{Deserialize, Serialize};

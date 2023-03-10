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
pub struct Workspace {
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
}

impl Workspace {
    pub fn new(id: String, r#type: RHashType, name: String) -> Workspace {
        Workspace {
            id,
            created: None,
            updated: None,
            seq: None,
            r#type,
            name,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "workspace")]
    Workspace,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Workspace
    }
}

use serde::{Deserialize, Serialize};

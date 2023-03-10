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
pub struct SubjectOrgMembership {
    #[serde(rename = "joined", skip_serializing_if = "Option::is_none")]
    pub joined: Option<String>,
    /// Arbitrary metadata.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl SubjectOrgMembership {
    pub fn new() -> SubjectOrgMembership {
        SubjectOrgMembership {
            joined: None,
            metadata: None,
        }
    }
}

use serde::{Deserialize, Serialize};

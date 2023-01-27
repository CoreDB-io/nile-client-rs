/*
 * Nile API
 *
 * Making SaaS chill.
 *
 * The version of the OpenAPI document: 0.1.0-fdd7cd5
 * Contact: support@thenile.dev
 * Generated by: https://openapi-generator.tech
 */

/// Action : The action to be allowed on the resource if an access policy is matched. The `deny` action is a special action that denies all access.

/// The action to be allowed on the resource if an access policy is matched. The `deny` action is a special action that denies all access.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "deny")]
    Deny,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Self::Read => String::from("read"),
            Self::Write => String::from("write"),
            Self::Deny => String::from("deny"),
        }
    }
}

impl Default for Action {
    fn default() -> Action {
        Self::Read
    }
}

use serde::{Deserialize, Serialize};

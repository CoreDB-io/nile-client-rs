/*
 * Nile API
 *
 * Making SaaS chill.
 *
 * The version of the OpenAPI document: 0.1.0-fdd7cd5
 * Contact: support@thenile.dev
 * Generated by: https://openapi-generator.tech
 */

/// Resource : A subset of properties of any custom or built-in entity instance to authorize against.  All properties on a resource are optional, and any combination of them can be specified when creating a policy. You can specify concrete values for resource and subject properties or use variables to match a subject property against a resource property.  An access policy with the following resource would allow access to clusters with location matching the subject's region: ``` {   \"type\": \"cluster\",   \"properties\": {     \"location\": ${subject.metadata.location}   } } ```   Note that: * Only exact matching of properties is supported for now. * For built-in entity instances, only the Policy entity is currently supported. * Custom properties on a resource must be specified under the \"properties\" key.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl Resource {
    /// A subset of properties of any custom or built-in entity instance to authorize against.  All properties on a resource are optional, and any combination of them can be specified when creating a policy. You can specify concrete values for resource and subject properties or use variables to match a subject property against a resource property.  An access policy with the following resource would allow access to clusters with location matching the subject's region: ``` {   \"type\": \"cluster\",   \"properties\": {     \"location\": ${subject.metadata.location}   } } ```   Note that: * Only exact matching of properties is supported for now. * For built-in entity instances, only the Policy entity is currently supported. * Custom properties on a resource must be specified under the \"properties\" key.
    pub fn new() -> Resource {
        Resource {
            id: None,
            r#type: None,
        }
    }
}

use serde::{Deserialize, Serialize};

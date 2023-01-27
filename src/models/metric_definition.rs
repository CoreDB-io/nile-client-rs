/*
 * Nile API
 *
 * Making SaaS chill.
 *
 * The version of the OpenAPI document: 0.1.0-fdd7cd5
 * Contact: support@thenile.dev
 * Generated by: https://openapi-generator.tech
 */

/// MetricDefinition : The list of metric definitions for a workspace or entity

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetricDefinition {
    /// The name of the metric
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the metric
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The entity type of the metric
    #[serde(rename = "entity_type")]
    pub entity_type: String,
}

impl MetricDefinition {
    /// The list of metric definitions for a workspace or entity
    pub fn new(name: String, r#type: RHashType, entity_type: String) -> MetricDefinition {
        MetricDefinition {
            name,
            r#type,
            entity_type,
        }
    }
}

/// The type of the metric
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "gauge")]
    Gauge,
    #[serde(rename = "sum")]
    Sum,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Gauge
    }
}

use serde::{Deserialize, Serialize};

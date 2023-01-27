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
pub struct Metric {
    /// The name of the metric that is unique in a workspace
    #[serde(rename = "name")]
    pub name: String,
    /// Type of metric. Currently sum or gauge
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The Nile entity type this metric is related to
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// Measurements associated with this metric
    #[serde(rename = "measurements")]
    pub measurements: Vec<crate::models::Measurement>,
}

impl Metric {
    pub fn new(
        name: String,
        r#type: RHashType,
        entity_type: String,
        measurements: Vec<crate::models::Measurement>,
    ) -> Metric {
        Metric {
            name,
            r#type,
            entity_type,
            measurements,
        }
    }
}

/// Type of metric. Currently sum or gauge
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

//! Provider-neutral situated anchor snapshots.

use crate::{LatticeValidationError, Pose, ReferenceSpace};
use serde::{Deserialize, Serialize};

/// Schema id for a situated anchor observation.
pub const SITUATED_ANCHOR_SCHEMA_ID: &str = "rusty.lattice.situated_anchor.v1";

/// Platform-neutral pose observation in one reference space.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SituatedAnchorSnapshot {
    /// Schema id.
    pub schema: String,
    /// Stable anchor id.
    pub anchor_id: String,
    /// Space containing the pose.
    pub reference_space: ReferenceSpace,
    /// Anchor pose in that space.
    pub pose: Pose,
    /// Provider validity bit.
    pub valid: bool,
    /// Confidence in the inclusive range 0..=1.
    pub confidence: f32,
    /// Monotonic observation time in nanoseconds, when known.
    #[serde(default)]
    pub observed_at_ns: Option<i64>,
    /// Monotonic relation revision.
    pub revision: u64,
    /// Source adapter or fixture id.
    pub source: String,
}

/// Validate a situated anchor without assigning simulation or rendering semantics.
pub fn validate_situated_anchor(
    anchor: &SituatedAnchorSnapshot,
) -> Result<(), Vec<LatticeValidationError>> {
    let mut errors = Vec::new();
    if anchor.schema != SITUATED_ANCHOR_SCHEMA_ID {
        errors.push(LatticeValidationError::new(
            "unsupported situated anchor schema",
        ));
    }
    if anchor.anchor_id.trim().is_empty() {
        errors.push(LatticeValidationError::new("anchor_id must not be empty"));
    }
    if !anchor.reference_space.is_valid() {
        errors.push(LatticeValidationError::new("reference_space must be valid"));
    }
    if !anchor.valid || !anchor.pose.is_valid() {
        errors.push(LatticeValidationError::new("anchor pose must be valid"));
    }
    if !anchor.confidence.is_finite() || !(0.0..=1.0).contains(&anchor.confidence) {
        errors.push(LatticeValidationError::new(
            "confidence must be within 0..=1",
        ));
    }
    if anchor.source.trim().is_empty() {
        errors.push(LatticeValidationError::new("source must not be empty"));
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID: &str = include_str!("../../../fixtures/anchors/particle-world-anchor.json");
    const DAMAGED: &str = include_str!("../../../fixtures/damaged/particle-anchor-app-leak.json");
    #[test]
    fn fixture_is_relation_only() {
        let value: SituatedAnchorSnapshot = serde_json::from_str(VALID).unwrap();
        validate_situated_anchor(&value).unwrap();
    }
    #[test]
    fn app_fields_are_rejected() {
        let value: serde_json::Value = serde_json::from_str(DAMAGED).unwrap();
        for key in [
            "application_scene",
            "platform_handle",
            "renderer_resource",
            "private_driver",
            "control_rate_hz",
        ] {
            assert!(value.get(key).is_some(), "damaged fixture must cover {key}");
        }
        assert!(serde_json::from_str::<SituatedAnchorSnapshot>(DAMAGED).is_err());
    }
}

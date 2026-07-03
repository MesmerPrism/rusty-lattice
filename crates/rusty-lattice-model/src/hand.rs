//! Hand provider capability snapshots.

use serde::{Deserialize, Serialize};

use crate::{LatticeValidationError, ReferenceSpace};

/// Schema id for hand provider capability snapshots.
pub const HAND_PROVIDER_CAPABILITY_SCHEMA_ID: &str = "rusty.lattice.hand_provider_capability.v1";

/// Logical hand side.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Handedness {
    /// Left hand.
    Left,
    /// Right hand.
    Right,
}

impl Handedness {
    /// Stable marker-friendly label.
    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// Provider joint layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandJointSet {
    /// OpenXR EXT hand-tracking 26-joint layout.
    #[serde(rename = "openxr_ext_hand_tracking_26")]
    OpenXrExtHandTracking26,
    /// Compact runtime layout with 21 tracked joints plus synthesized tips.
    #[serde(rename = "compact_21_with_tips")]
    Compact21WithTips,
    /// Provider-native layout whose mapping is described outside Lattice.
    #[serde(rename = "provider_native")]
    ProviderNative,
}

/// Hand mesh availability advertised by a provider.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandMeshBinding {
    /// No mesh payload is available.
    None,
    /// Provider can expose a stable bind mesh for downstream Matter payloads.
    StaticBindMesh,
    /// Provider can expose changing mesh topology or vertex data.
    DynamicProviderMesh,
}

/// Runtime signals a hand provider can report.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandRuntimeSignals {
    /// Provider emits joint poses.
    pub joint_poses: bool,
    /// Provider emits joint radii.
    pub joint_radii: bool,
    /// Provider emits per-joint location flags.
    pub joint_location_flags: bool,
    /// Provider emits per-joint confidence values.
    pub joint_confidence: bool,
    /// Provider emits pinch or gesture state.
    pub pinch_state: bool,
    /// Provider emits mesh vertices.
    pub mesh_vertices: bool,
    /// Provider emits skinning joint indices and weights.
    pub mesh_skinning_weights: bool,
}

impl HandRuntimeSignals {
    /// Whether the signal set contains at least one usable relation or mesh signal.
    #[must_use]
    pub const fn is_non_empty(&self) -> bool {
        self.joint_poses
            || self.joint_radii
            || self.joint_location_flags
            || self.joint_confidence
            || self.pinch_state
            || self.mesh_vertices
            || self.mesh_skinning_weights
    }
}

/// Capability row for one hand role.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandCapability {
    /// Logical hand role.
    pub hand: Handedness,
    /// Provider joint layout.
    pub joint_set: HandJointSet,
    /// Number of joints in provider packets before any downstream expansion.
    pub joint_count: u16,
    /// Mesh binding class.
    pub mesh_binding: HandMeshBinding,
    /// Runtime signals that may be observed from this hand.
    pub signals: HandRuntimeSignals,
}

impl HandCapability {
    /// Whether this hand capability is internally coherent.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.signals.is_non_empty()
            && (!self.signals.joint_poses || self.joint_count > 0)
            && (!self.signals.mesh_skinning_weights || self.mesh_binding != HandMeshBinding::None)
            && (!self.signals.mesh_vertices || self.mesh_binding != HandMeshBinding::None)
    }
}

/// Snapshot of a runtime hand provider's Lattice-facing capabilities.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HandProviderCapabilitySnapshot {
    /// Schema id.
    pub schema: String,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Stable provider id.
    pub provider_id: String,
    /// Source adapter or fixture id.
    pub source: String,
    /// Reference space where runtime hand relations are reported.
    pub reference_space: ReferenceSpace,
    /// Timestamp-domain id used by provider frames.
    pub timestamp_domain: String,
    /// Monotonic relation revision.
    pub revision: u64,
    /// Runtime hand capabilities available from this provider.
    pub hands: Vec<HandCapability>,
    /// Runtime validity bit.
    pub valid: bool,
    /// Snapshot confidence from 0 to 1.
    pub confidence: f32,
}

impl HandProviderCapabilitySnapshot {
    /// Create a snapshot with the current schema id.
    pub fn new(
        snapshot_id: impl Into<String>,
        provider_id: impl Into<String>,
        source: impl Into<String>,
        reference_space: ReferenceSpace,
        timestamp_domain: impl Into<String>,
        revision: u64,
        hands: Vec<HandCapability>,
    ) -> Self {
        Self {
            schema: HAND_PROVIDER_CAPABILITY_SCHEMA_ID.to_string(),
            snapshot_id: snapshot_id.into(),
            provider_id: provider_id.into(),
            source: source.into(),
            reference_space,
            timestamp_domain: timestamp_domain.into(),
            revision,
            hands,
            valid: true,
            confidence: 1.0,
        }
    }

    /// Whether this snapshot is usable by downstream adapters.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        validate_hand_provider_capability_snapshot(self).is_ok()
    }
}

/// Validate a hand provider capability snapshot and report all top-level failures.
pub fn validate_hand_provider_capability_snapshot(
    snapshot: &HandProviderCapabilitySnapshot,
) -> Result<(), Vec<LatticeValidationError>> {
    let mut errors = Vec::new();
    if snapshot.schema != HAND_PROVIDER_CAPABILITY_SCHEMA_ID {
        errors.push(LatticeValidationError::new(format!(
            "unsupported hand provider capability schema {}",
            snapshot.schema
        )));
    }
    if snapshot.snapshot_id.trim().is_empty() {
        errors.push(LatticeValidationError::new("snapshot_id must not be empty"));
    }
    if snapshot.provider_id.trim().is_empty() {
        errors.push(LatticeValidationError::new("provider_id must not be empty"));
    }
    if snapshot.source.trim().is_empty() {
        errors.push(LatticeValidationError::new("source must not be empty"));
    }
    if !snapshot.reference_space.is_valid() {
        errors.push(LatticeValidationError::new("reference_space must be valid"));
    }
    if snapshot.timestamp_domain.trim().is_empty() {
        errors.push(LatticeValidationError::new(
            "timestamp_domain must not be empty",
        ));
    }
    if snapshot.revision == 0 {
        errors.push(LatticeValidationError::new("revision must be non-zero"));
    }
    if snapshot.hands.is_empty() {
        errors.push(LatticeValidationError::new("hands must not be empty"));
    }
    if !snapshot.valid {
        errors.push(LatticeValidationError::new("snapshot must be valid"));
    }
    if !snapshot.confidence.is_finite() || !(0.0..=1.0).contains(&snapshot.confidence) {
        errors.push(LatticeValidationError::new(
            "confidence must be finite and in [0, 1]",
        ));
    }

    let mut seen_left = false;
    let mut seen_right = false;
    for hand in &snapshot.hands {
        match hand.hand {
            Handedness::Left if seen_left => {
                errors.push(LatticeValidationError::new(
                    "hands must not contain duplicate left capability",
                ));
            }
            Handedness::Left => seen_left = true,
            Handedness::Right if seen_right => {
                errors.push(LatticeValidationError::new(
                    "hands must not contain duplicate right capability",
                ));
            }
            Handedness::Right => seen_right = true,
        }

        if !hand.is_valid() {
            errors.push(LatticeValidationError::new(format!(
                "{} hand capability must be coherent",
                hand.hand.stable_id()
            )));
        }
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

    const VALID_HAND_PROVIDER: &str = include_str!(
        "../../../fixtures/hand_provider_capabilities/generic-tracked-hand-provider-capability.json"
    );
    const DAMAGED_HAND_PROVIDER: &str =
        include_str!("../../../fixtures/damaged/duplicate-hand-provider-capability.json");

    #[test]
    fn valid_hand_provider_fixture_parses_and_validates() {
        let snapshot: HandProviderCapabilitySnapshot =
            serde_json::from_str(VALID_HAND_PROVIDER).unwrap();
        validate_hand_provider_capability_snapshot(&snapshot).unwrap();
        assert_eq!(snapshot.hands[0].hand.stable_id(), "left");
        assert_eq!(snapshot.hands[1].hand.stable_id(), "right");
        assert!(snapshot.hands.iter().all(HandCapability::is_valid));
    }

    #[test]
    fn damaged_hand_provider_fixture_reports_duplicate_hand() {
        let snapshot: HandProviderCapabilitySnapshot =
            serde_json::from_str(DAMAGED_HAND_PROVIDER).unwrap();
        let errors = validate_hand_provider_capability_snapshot(&snapshot).unwrap_err();
        assert!(errors
            .iter()
            .any(|error| error.message.contains("duplicate left capability")));
    }

    #[test]
    fn mesh_skinning_requires_mesh_binding() {
        let hand = HandCapability {
            hand: Handedness::Left,
            joint_set: HandJointSet::OpenXrExtHandTracking26,
            joint_count: 26,
            mesh_binding: HandMeshBinding::None,
            signals: HandRuntimeSignals {
                joint_poses: true,
                mesh_skinning_weights: true,
                ..HandRuntimeSignals::default()
            },
        };
        assert!(!hand.is_valid());
    }
}

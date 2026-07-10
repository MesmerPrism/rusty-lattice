//! Hand provider capability snapshots.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::{LatticeValidationError, Pose, ReferenceSpace};

/// Schema id for hand provider capability snapshots.
pub const HAND_PROVIDER_CAPABILITY_SCHEMA_ID: &str = "rusty.lattice.hand_provider_capability.v1";
/// Schema id for hand provider joint mapping snapshots.
pub const HAND_JOINT_MAPPING_SCHEMA_ID: &str = "rusty.lattice.hand_joint_mapping.v1";
/// Schema id for provider-neutral hand joint frames.
pub const HAND_JOINT_FRAME_SCHEMA_ID: &str = "rusty.lattice.hand_joint_frame.v1";

/// Coordinate basis used by provider hand relations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandCoordinateBasis {
    /// Right-handed basis with +Y up and -Z forward.
    RightHandedYUpNegativeZForward,
    /// Right-handed basis with +Y up and +Z forward.
    RightHandedYUpPositiveZForward,
}

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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
    /// Coordinate basis used by provider poses.
    pub coordinate_basis: HandCoordinateBasis,
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
        coordinate_basis: HandCoordinateBasis,
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
            coordinate_basis,
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

/// One provider-joint to target bind-joint mapping entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandJointMapEntry {
    /// Provider joint index.
    pub source_joint_index: u16,
    /// Provider joint name or stable provider label.
    pub source_joint_name: String,
    /// Target bind-joint index in the downstream hand rig.
    pub target_joint_index: u16,
    /// Target bind-joint name or stable target label.
    pub target_joint_name: String,
    /// Semantic role used by tools and reviewers.
    pub joint_role: String,
}

impl HandJointMapEntry {
    /// Whether this entry is usable for the declared joint counts.
    #[must_use]
    pub fn is_valid(&self, source_joint_count: u16, target_joint_count: u16) -> bool {
        self.source_joint_index < source_joint_count
            && self.target_joint_index < target_joint_count
            && !self.source_joint_name.trim().is_empty()
            && !self.target_joint_name.trim().is_empty()
            && !self.joint_role.trim().is_empty()
    }
}

/// Snapshot mapping a provider joint set into a target hand rig joint layout.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandJointMappingSnapshot {
    /// Schema id.
    pub schema: String,
    /// Stable mapping id.
    pub mapping_id: String,
    /// Provider id that emits the source joint set.
    pub provider_id: String,
    /// Source adapter or fixture id.
    pub source: String,
    /// Provider joint layout.
    pub source_joint_set: HandJointSet,
    /// Number of joints emitted by the source provider.
    pub source_joint_count: u16,
    /// Target payload schema id, such as a Matter hand rig schema.
    pub target_schema_id: String,
    /// Number of bind joints expected by the target payload.
    pub target_joint_count: u16,
    /// Monotonic relation revision.
    pub revision: u64,
    /// Joint mapping entries.
    pub entries: Vec<HandJointMapEntry>,
    /// Runtime validity bit.
    pub valid: bool,
    /// Mapping confidence from 0 to 1.
    pub confidence: f32,
}

/// One provider-neutral joint relation in a hand frame.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandJointSample {
    /// Provider joint index in the declared joint set.
    pub joint_index: u16,
    /// Stable provider-neutral joint name.
    pub joint_name: String,
    /// Joint pose in the frame reference space and coordinate basis.
    pub pose: Pose,
    /// Optional joint radius in meters.
    pub radius_m: Option<f32>,
    /// Whether the position component is valid.
    pub position_valid: bool,
    /// Whether the orientation component is valid.
    pub orientation_valid: bool,
    /// Whether the provider reports this joint as actively tracked.
    pub tracked: bool,
    /// Per-joint confidence from 0 to 1.
    pub confidence: f32,
}

impl HandJointSample {
    fn is_valid(&self, joint_count: u16) -> bool {
        self.joint_index < joint_count
            && !self.joint_name.trim().is_empty()
            && self.pose.is_valid()
            && self
                .radius_m
                .map_or(true, |radius| radius.is_finite() && radius >= 0.0)
            && self.confidence.is_finite()
            && (0.0..=1.0).contains(&self.confidence)
            && (!self.tracked
                || (self.position_valid && self.orientation_valid && self.confidence > 0.0))
    }
}

/// Provider-neutral frame containing one hand's tracked joint relations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandJointFrameSnapshot {
    /// Schema id.
    pub schema: String,
    /// Stable frame id.
    pub frame_id: String,
    /// Provider that emitted this frame.
    pub provider_id: String,
    /// Logical hand role.
    pub hand: Handedness,
    /// Provider joint layout.
    pub joint_set: HandJointSet,
    /// Number of joints in the declared provider layout.
    pub joint_count: u16,
    /// Reference space containing all poses.
    pub reference_space: ReferenceSpace,
    /// Coordinate basis containing all poses.
    pub coordinate_basis: HandCoordinateBasis,
    /// Provider timestamp domain.
    pub timestamp_domain: String,
    /// Timestamp in the declared domain.
    pub timestamp_ns: u64,
    /// Monotonic provider sequence.
    pub sequence_id: u64,
    /// Maximum frame age before downstream consumers must treat it as stale.
    pub stale_after_ns: u64,
    /// Joint samples.
    pub joints: Vec<HandJointSample>,
    /// Provider validity bit.
    pub valid: bool,
    /// Whole-frame confidence from 0 to 1.
    pub confidence: f32,
}

/// Validate a provider-neutral hand joint frame.
pub fn validate_hand_joint_frame_snapshot(
    frame: &HandJointFrameSnapshot,
) -> Result<(), Vec<LatticeValidationError>> {
    let mut errors = Vec::new();
    if frame.schema != HAND_JOINT_FRAME_SCHEMA_ID {
        errors.push(LatticeValidationError::new(format!(
            "unsupported hand joint frame schema {}",
            frame.schema
        )));
    }
    if frame.frame_id.trim().is_empty() || frame.provider_id.trim().is_empty() {
        errors.push(LatticeValidationError::new(
            "frame_id and provider_id must not be empty",
        ));
    }
    if frame.joint_count == 0 || frame.joints.len() != usize::from(frame.joint_count) {
        errors.push(LatticeValidationError::new(
            "joints must contain exactly joint_count entries",
        ));
    }
    if !frame.reference_space.is_valid() || frame.timestamp_domain.trim().is_empty() {
        errors.push(LatticeValidationError::new(
            "reference_space and timestamp_domain must be valid",
        ));
    }
    if frame.timestamp_ns == 0 || frame.sequence_id == 0 || frame.stale_after_ns == 0 {
        errors.push(LatticeValidationError::new(
            "timestamp_ns, sequence_id, and stale_after_ns must be non-zero",
        ));
    }
    if !frame.valid || !frame.confidence.is_finite() || !(0.0..=1.0).contains(&frame.confidence) {
        errors.push(LatticeValidationError::new(
            "frame must be valid with confidence in [0, 1]",
        ));
    }
    let mut indices = BTreeSet::new();
    let mut names = BTreeSet::new();
    let mut tracked = 0_usize;
    for joint in &frame.joints {
        if !indices.insert(joint.joint_index) || !names.insert(joint.joint_name.as_str()) {
            errors.push(LatticeValidationError::new(
                "joint indices and names must be unique",
            ));
        }
        if !joint.is_valid(frame.joint_count) {
            errors.push(LatticeValidationError::new(format!(
                "joint sample {} must be coherent",
                joint.joint_index
            )));
        }
        tracked += usize::from(joint.tracked);
    }
    if tracked == 0 {
        errors.push(LatticeValidationError::new(
            "at least one joint must be actively tracked",
        ));
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate that a frame belongs to one advertised provider capability row.
pub fn validate_hand_provider_frame_match(
    capability: &HandProviderCapabilitySnapshot,
    frame: &HandJointFrameSnapshot,
) -> Result<(), Vec<LatticeValidationError>> {
    let mut errors = validate_hand_provider_capability_snapshot(capability)
        .err()
        .unwrap_or_default();
    errors.extend(
        validate_hand_joint_frame_snapshot(frame)
            .err()
            .unwrap_or_default(),
    );
    if capability.provider_id != frame.provider_id {
        errors.push(LatticeValidationError::new("provider_id mismatch"));
    }
    if capability.reference_space != frame.reference_space {
        errors.push(LatticeValidationError::new("reference_space mismatch"));
    }
    if capability.coordinate_basis != frame.coordinate_basis {
        errors.push(LatticeValidationError::new("coordinate_basis mismatch"));
    }
    if capability.timestamp_domain != frame.timestamp_domain {
        errors.push(LatticeValidationError::new("timestamp_domain mismatch"));
    }
    let advertised = capability.hands.iter().any(|hand| {
        hand.hand == frame.hand
            && hand.joint_set == frame.joint_set
            && hand.joint_count == frame.joint_count
            && hand.signals.joint_poses
    });
    if !advertised {
        errors.push(LatticeValidationError::new(
            "frame is not advertised by provider capability",
        ));
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

impl HandJointMappingSnapshot {
    /// Create a mapping with the current schema id.
    pub fn new(
        mapping_id: impl Into<String>,
        provider_id: impl Into<String>,
        source: impl Into<String>,
        source_joint_set: HandJointSet,
        source_joint_count: u16,
        target_schema_id: impl Into<String>,
        target_joint_count: u16,
        revision: u64,
        entries: Vec<HandJointMapEntry>,
    ) -> Self {
        Self {
            schema: HAND_JOINT_MAPPING_SCHEMA_ID.to_string(),
            mapping_id: mapping_id.into(),
            provider_id: provider_id.into(),
            source: source.into(),
            source_joint_set,
            source_joint_count,
            target_schema_id: target_schema_id.into(),
            target_joint_count,
            revision,
            entries,
            valid: true,
            confidence: 1.0,
        }
    }

    /// Whether this mapping is usable by downstream adapters.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        validate_hand_joint_mapping_snapshot(self).is_ok()
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

/// Validate a hand joint mapping snapshot and report all top-level failures.
pub fn validate_hand_joint_mapping_snapshot(
    mapping: &HandJointMappingSnapshot,
) -> Result<(), Vec<LatticeValidationError>> {
    let mut errors = Vec::new();
    if mapping.schema != HAND_JOINT_MAPPING_SCHEMA_ID {
        errors.push(LatticeValidationError::new(format!(
            "unsupported hand joint mapping schema {}",
            mapping.schema
        )));
    }
    if mapping.mapping_id.trim().is_empty() {
        errors.push(LatticeValidationError::new("mapping_id must not be empty"));
    }
    if mapping.provider_id.trim().is_empty() {
        errors.push(LatticeValidationError::new("provider_id must not be empty"));
    }
    if mapping.source.trim().is_empty() {
        errors.push(LatticeValidationError::new("source must not be empty"));
    }
    if mapping.source_joint_count == 0 {
        errors.push(LatticeValidationError::new(
            "source_joint_count must be non-zero",
        ));
    }
    if mapping.target_schema_id.trim().is_empty() {
        errors.push(LatticeValidationError::new(
            "target_schema_id must not be empty",
        ));
    }
    if mapping.target_joint_count == 0 {
        errors.push(LatticeValidationError::new(
            "target_joint_count must be non-zero",
        ));
    }
    if mapping.revision == 0 {
        errors.push(LatticeValidationError::new("revision must be non-zero"));
    }
    if mapping.entries.is_empty() {
        errors.push(LatticeValidationError::new("entries must not be empty"));
    }
    if !mapping.valid {
        errors.push(LatticeValidationError::new("mapping must be valid"));
    }
    if !mapping.confidence.is_finite() || !(0.0..=1.0).contains(&mapping.confidence) {
        errors.push(LatticeValidationError::new(
            "confidence must be finite and in [0, 1]",
        ));
    }

    let mut source_indices = BTreeSet::new();
    for entry in &mapping.entries {
        if !source_indices.insert(entry.source_joint_index) {
            errors.push(LatticeValidationError::new(format!(
                "duplicate source joint index {}",
                entry.source_joint_index
            )));
        }
        if !entry.is_valid(mapping.source_joint_count, mapping.target_joint_count) {
            errors.push(LatticeValidationError::new(format!(
                "joint mapping entry {} must be coherent",
                entry.source_joint_index
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
    const VALID_HAND_MAPPING: &str = include_str!(
        "../../../fixtures/hand_joint_mappings/generic-openxr-to-matter-bind-joint-map.json"
    );
    const DAMAGED_HAND_PROVIDER: &str =
        include_str!("../../../fixtures/damaged/duplicate-hand-provider-capability.json");
    const DAMAGED_HAND_MAPPING: &str =
        include_str!("../../../fixtures/damaged/duplicate-hand-joint-mapping.json");
    const VALID_HAND_FRAME: &str =
        include_str!("../../../fixtures/hand_joint_frames/generic-left-hand-joint-frame.json");
    const DAMAGED_PROVIDER_FRAME: &str =
        include_str!("../../../fixtures/damaged/hand-joint-frame-provider-mixup.json");
    const DAMAGED_BASIS_FRAME: &str =
        include_str!("../../../fixtures/damaged/hand-joint-frame-basis-mismatch.json");

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

    #[test]
    fn valid_hand_joint_mapping_fixture_parses_and_validates() {
        let mapping: HandJointMappingSnapshot = serde_json::from_str(VALID_HAND_MAPPING).unwrap();
        validate_hand_joint_mapping_snapshot(&mapping).unwrap();
        assert!(mapping
            .entries
            .iter()
            .all(|entry| entry.is_valid(mapping.source_joint_count, mapping.target_joint_count)));
    }

    #[test]
    fn damaged_hand_joint_mapping_fixture_reports_duplicate_source() {
        let mapping: HandJointMappingSnapshot = serde_json::from_str(DAMAGED_HAND_MAPPING).unwrap();
        let errors = validate_hand_joint_mapping_snapshot(&mapping).unwrap_err();
        assert!(errors
            .iter()
            .any(|error| error.message.contains("duplicate source joint index")));
    }

    #[test]
    fn valid_hand_frame_matches_provider_capability() {
        let capability: HandProviderCapabilitySnapshot =
            serde_json::from_str(VALID_HAND_PROVIDER).unwrap();
        let frame: HandJointFrameSnapshot = serde_json::from_str(VALID_HAND_FRAME).unwrap();
        validate_hand_provider_frame_match(&capability, &frame).unwrap();
        let mapping: HandJointMappingSnapshot = serde_json::from_str(VALID_HAND_MAPPING).unwrap();
        for entry in mapping.entries {
            let sample = frame
                .joints
                .iter()
                .find(|joint| joint.joint_index == entry.source_joint_index)
                .unwrap();
            assert_eq!(sample.joint_name, entry.source_joint_name);
        }
    }

    #[test]
    fn provider_mixup_and_basis_mismatch_fail_closed() {
        let capability: HandProviderCapabilitySnapshot =
            serde_json::from_str(VALID_HAND_PROVIDER).unwrap();
        let valid: HandJointFrameSnapshot = serde_json::from_str(VALID_HAND_FRAME).unwrap();
        let provider_damage: serde_json::Value =
            serde_json::from_str(DAMAGED_PROVIDER_FRAME).unwrap();
        let basis_damage: serde_json::Value = serde_json::from_str(DAMAGED_BASIS_FRAME).unwrap();
        let mut provider_mixup = valid.clone();
        provider_mixup.provider_id = provider_damage["provider_id"].as_str().unwrap().to_owned();
        let mut basis_mismatch = valid;
        basis_mismatch.coordinate_basis =
            serde_json::from_value(basis_damage["coordinate_basis"].clone()).unwrap();
        assert!(
            validate_hand_provider_frame_match(&capability, &provider_mixup)
                .unwrap_err()
                .iter()
                .any(|error| error.message.contains("provider_id mismatch"))
        );
        assert!(
            validate_hand_provider_frame_match(&capability, &basis_mismatch)
                .unwrap_err()
                .iter()
                .any(|error| error.message.contains("coordinate_basis mismatch"))
        );
    }

    #[test]
    fn unknown_platform_fields_are_rejected() {
        let damaged = VALID_HAND_FRAME.replace(
            "\"valid\": true",
            "\"platform_handle\": 42, \"valid\": true",
        );
        assert!(serde_json::from_str::<HandJointFrameSnapshot>(&damaged).is_err());
    }
}

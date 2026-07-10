//! Platform-neutral relation contracts for Rusty Lattice.

pub mod anchor;
pub mod hand;

pub use anchor::{validate_situated_anchor, SituatedAnchorSnapshot, SITUATED_ANCHOR_SCHEMA_ID};

pub use hand::{
    validate_hand_joint_mapping_snapshot, validate_hand_provider_capability_snapshot,
    HandCapability, HandJointMapEntry, HandJointMappingSnapshot, HandJointSet, HandMeshBinding,
    HandProviderCapabilitySnapshot, HandRuntimeSignals, Handedness, HAND_JOINT_MAPPING_SCHEMA_ID,
    HAND_PROVIDER_CAPABILITY_SCHEMA_ID,
};

use serde::{Deserialize, Serialize};

/// Schema id for stereo display view sets.
pub const DISPLAY_VIEW_SET_SCHEMA_ID: &str = "rusty.lattice.display_view_set.v1";

/// Three-dimensional relation vector.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl Vec3 {
    /// Zero vector.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    /// Create a vector from components.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Whether all components are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

/// Unit quaternion orientation.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quat {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
    /// W component.
    pub w: f32,
}

impl Quat {
    /// Identity orientation.
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// Create a quaternion from components.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Whether all components are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Squared quaternion length.
    #[must_use]
    pub fn length_squared(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
    }

    /// Whether the quaternion is close enough to unit length for relation use.
    #[must_use]
    pub fn is_unitish(self) -> bool {
        self.is_finite() && (self.length_squared() - 1.0).abs() <= 0.02
    }
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Rigid pose from a parent space into a child relation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    /// Translation in meters.
    pub position: Vec3,
    /// Orientation.
    pub orientation: Quat,
}

impl Pose {
    /// Identity pose.
    pub const IDENTITY: Self = Self::new(Vec3::ZERO, Quat::IDENTITY);

    /// Create a pose.
    pub const fn new(position: Vec3, orientation: Quat) -> Self {
        Self {
            position,
            orientation,
        }
    }

    /// Whether this pose is finite and has a valid orientation.
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.position.is_finite() && self.orientation.is_unitish()
    }
}

/// Logical eye for per-view metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Eye {
    /// Monoscopic view.
    Mono,
    /// Left stereo eye.
    Left,
    /// Right stereo eye.
    Right,
}

impl Eye {
    /// Stable marker-friendly label.
    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Mono => "mono",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// Reference space category.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceSpaceKind {
    /// Local runtime space.
    Local,
    /// Stage or floor-bounded runtime space.
    Stage,
    /// Viewer/head-local space.
    View,
    /// Head pose space.
    Head,
    /// App-defined stable space.
    App,
    /// Unknown or unavailable space.
    Unknown,
}

/// Reference space descriptor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceSpace {
    /// Stable reference-space id.
    pub stable_id: String,
    /// Reference-space kind.
    pub kind: ReferenceSpaceKind,
}

impl ReferenceSpace {
    /// Create a reference-space descriptor.
    pub fn new(stable_id: impl Into<String>, kind: ReferenceSpaceKind) -> Self {
        Self {
            stable_id: stable_id.into(),
            kind,
        }
    }

    /// Whether the descriptor is usable.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.stable_id.trim().is_empty() && self.kind != ReferenceSpaceKind::Unknown
    }
}

/// OpenXR-style tangent-angle field of view in radians.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldOfView {
    /// Left angle in radians.
    pub angle_left_radians: f32,
    /// Right angle in radians.
    pub angle_right_radians: f32,
    /// Up angle in radians.
    pub angle_up_radians: f32,
    /// Down angle in radians.
    pub angle_down_radians: f32,
}

impl FieldOfView {
    /// Create a field of view from OpenXR-style angles.
    pub const fn new(
        angle_left_radians: f32,
        angle_right_radians: f32,
        angle_up_radians: f32,
        angle_down_radians: f32,
    ) -> Self {
        Self {
            angle_left_radians,
            angle_right_radians,
            angle_up_radians,
            angle_down_radians,
        }
    }

    /// Whether all angles are finite and have the expected stereo signs.
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.angle_left_radians.is_finite()
            && self.angle_right_radians.is_finite()
            && self.angle_up_radians.is_finite()
            && self.angle_down_radians.is_finite()
            && self.angle_left_radians < 0.0
            && self.angle_right_radians > 0.0
            && self.angle_up_radians > 0.0
            && self.angle_down_radians < 0.0
    }

    /// Tangents in `[left, right, up, down]` order.
    #[must_use]
    pub fn tangents(self) -> [f32; 4] {
        [
            self.angle_left_radians.tan(),
            self.angle_right_radians.tan(),
            self.angle_up_radians.tan(),
            self.angle_down_radians.tan(),
        ]
    }
}

/// One display eye view in a reference space.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayEyeView {
    /// Logical eye.
    pub eye: Eye,
    /// Pose of the eye view in the reference space.
    pub pose: Pose,
    /// View field of view.
    pub fov: FieldOfView,
    /// Runtime validity bit.
    pub valid: bool,
    /// Confidence from 0 to 1.
    pub confidence: f32,
}

impl DisplayEyeView {
    /// Create a display eye view.
    pub const fn new(eye: Eye, pose: Pose, fov: FieldOfView) -> Self {
        Self {
            eye,
            pose,
            fov,
            valid: true,
            confidence: 1.0,
        }
    }

    /// Whether this view is usable.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.valid
            && self.pose.is_valid()
            && self.fov.is_valid()
            && self.confidence.is_finite()
            && (0.0..=1.0).contains(&self.confidence)
    }
}

/// Paired display view set for a stereo frame.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayViewSet {
    /// Schema id.
    pub schema: String,
    /// Stable view-set id.
    pub view_set_id: String,
    /// Reference space shared by both views.
    pub reference_space: ReferenceSpace,
    /// Predicted display time in nanoseconds, when known.
    #[serde(default)]
    pub predicted_display_time_ns: Option<i64>,
    /// Source adapter or fixture id.
    pub source: String,
    /// Monotonic relation revision.
    pub revision: u64,
    /// Left eye view.
    pub left: DisplayEyeView,
    /// Right eye view.
    pub right: DisplayEyeView,
}

impl DisplayViewSet {
    /// Create a display view set with the current schema id.
    pub fn new(
        view_set_id: impl Into<String>,
        reference_space: ReferenceSpace,
        source: impl Into<String>,
        revision: u64,
        left: DisplayEyeView,
        right: DisplayEyeView,
    ) -> Self {
        Self {
            schema: DISPLAY_VIEW_SET_SCHEMA_ID.to_string(),
            view_set_id: view_set_id.into(),
            reference_space,
            predicted_display_time_ns: None,
            source: source.into(),
            revision,
            left,
            right,
        }
    }

    /// Whether the view-set fields are usable.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.schema == DISPLAY_VIEW_SET_SCHEMA_ID
            && !self.view_set_id.trim().is_empty()
            && !self.source.trim().is_empty()
            && self.reference_space.is_valid()
            && self.left.eye == Eye::Left
            && self.right.eye == Eye::Right
            && self.left.is_valid()
            && self.right.is_valid()
    }
}

/// Lattice validation failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LatticeValidationError {
    /// Human-readable message.
    pub message: String,
}

impl LatticeValidationError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Validate a display view set and report all top-level failures.
pub fn validate_display_view_set(
    view_set: &DisplayViewSet,
) -> Result<(), Vec<LatticeValidationError>> {
    let mut errors = Vec::new();
    if view_set.schema != DISPLAY_VIEW_SET_SCHEMA_ID {
        errors.push(LatticeValidationError::new(format!(
            "unsupported display view set schema {}",
            view_set.schema
        )));
    }
    if view_set.view_set_id.trim().is_empty() {
        errors.push(LatticeValidationError::new("view_set_id must not be empty"));
    }
    if view_set.source.trim().is_empty() {
        errors.push(LatticeValidationError::new("source must not be empty"));
    }
    if !view_set.reference_space.is_valid() {
        errors.push(LatticeValidationError::new("reference_space must be valid"));
    }
    if view_set.left.eye != Eye::Left {
        errors.push(LatticeValidationError::new("left view must use eye=left"));
    }
    if view_set.right.eye != Eye::Right {
        errors.push(LatticeValidationError::new("right view must use eye=right"));
    }
    if !view_set.left.is_valid() {
        errors.push(LatticeValidationError::new("left view must be valid"));
    }
    if !view_set.right.is_valid() {
        errors.push(LatticeValidationError::new("right view must be valid"));
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

    const VALID_VIEW_SET: &str =
        include_str!("../../../fixtures/view_sets/quest-makepad-synthetic-display-view-set.json");
    const DAMAGED_VIEW_SET: &str =
        include_str!("../../../fixtures/damaged/wrong-eye-display-view-set.json");

    #[test]
    fn valid_fixture_parses_and_validates() {
        let view_set: DisplayViewSet = serde_json::from_str(VALID_VIEW_SET).unwrap();
        validate_display_view_set(&view_set).unwrap();
        assert_eq!(view_set.left.eye.stable_id(), "left");
        assert_eq!(
            view_set.left.fov.tangents(),
            [
                view_set.left.fov.angle_left_radians.tan(),
                view_set.left.fov.angle_right_radians.tan(),
                view_set.left.fov.angle_up_radians.tan(),
                view_set.left.fov.angle_down_radians.tan(),
            ]
        );
    }

    #[test]
    fn damaged_fixture_reports_eye_error() {
        let view_set: DisplayViewSet = serde_json::from_str(DAMAGED_VIEW_SET).unwrap();
        let errors = validate_display_view_set(&view_set).unwrap_err();
        assert!(errors
            .iter()
            .any(|error| error.message.contains("left view must use eye=left")));
    }

    #[test]
    fn invalid_pose_rejects_non_unit_orientation() {
        let fov = FieldOfView::new(-0.7, 0.7, 0.7, -0.7);
        let view = DisplayEyeView::new(
            Eye::Left,
            Pose::new(Vec3::ZERO, Quat::new(0.0, 0.0, 0.0, 2.0)),
            fov,
        );
        assert!(!view.is_valid());
    }
}

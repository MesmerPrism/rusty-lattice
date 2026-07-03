# Rusty Lattice Architecture

Rusty Lattice owns situated relation contracts. It records where things are,
how runtime views relate to reference spaces, and whether those relations are
valid, confident, stale, or capability-backed.

## Ownership

- reference-space descriptors;
- tracked poses and transforms;
- stereo and mono display view sets;
- spatial input roles;
- frame-state binding and display-time evidence;
- calibration, validity, confidence, and staleness metadata;
- runtime capability snapshots.

## Non-Ownership

- Matter mesh, SDF/ADF, collision, particle, field, or dynamics truth;
- Optics projection, homography, lens, appearance, and debug-visualization
  policy;
- Quest permissions, Android properties, ADB, or headset lifecycle tooling;
- Makepad event loops, shaders, widgets, textures, or app state;
- OpenXR handles and SDK calls.

## First Slice

The initial `rusty-lattice-model` crate exposes
`rusty.lattice.display_view_set.v1` for stereo eye poses and FOV in a reference
space. Platform adapters can convert runtime view data into this contract, and
Optics/Makepad consumers can read it without depending on platform SDKs.

The same crate also exposes `rusty.lattice.hand_provider_capability.v1` for
hand-provider capability snapshots. This is the Lattice-owned relation layer
for hand substrate extraction: it records hand roles, joint layout, reference
space, timestamp domain, confidence, mesh-binding availability, and runtime
signals. Matter remains the owner of hand rig, mesh payload, joint-frame, and
CPU-skinning truth; Optics remains the owner of visual profiles and debug
presentation; platform adapters remain the owner of SDK calls and runtime
handles.

`rusty.lattice.hand_joint_mapping.v1` records provider-joint to target
bind-joint mappings. It can reference target schema IDs such as Matter hand rig
payloads as strings, but it does not import Matter crates or become the owner
of rig geometry, bind poses, skinning weights, or CPU skinning. This keeps the
provider relation contract separate from computational mesh truth.

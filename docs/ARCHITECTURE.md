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

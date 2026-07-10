# Rusty Lattice

Rusty Lattice is the Morphospace relation-contract lane. It describes reference
spaces, tracked poses, view sets, frame-state binding, validity, confidence,
staleness, and capability evidence without importing platform SDKs or renderer
backends.

`rusty.lattice.situated_anchor.v1` carries a provider-neutral reference space,
pose, validity, confidence, observation time, revision, and source for
Matter/Optics/Quest consumers. Its damaged fixture rejects application fields;
it does not own particle state, simulation, appearance, or rendering.

The first slices contain `rusty-lattice-model`, a minimal model crate for
stereo display view sets and runtime capability snapshots. The display view-set
contract gives eye/view/pose data a clean Morphospace home before Optics adds
projection and homography contracts. The hand provider capability and joint
mapping contracts record what relation and mesh signals a source can report,
and how provider joint indices map into a target bind-joint layout, without
importing OpenXR, Spatial SDK, Makepad, Quest, or renderer dependencies into
Lattice. `rusty.lattice.hand_joint_frame.v1` adds provider-neutral per-joint
poses, validity, confidence, coordinate basis, timestamp domain, and staleness;
provider/basis mixups and platform-field leakage fail closed.

## Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

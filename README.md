# Rusty Lattice

Rusty Lattice is the Morphospace relation-contract lane. It describes reference
spaces, tracked poses, view sets, frame-state binding, validity, confidence,
staleness, and capability evidence without importing platform SDKs or renderer
backends.

The first slices contain `rusty-lattice-model`, a minimal model crate for
stereo display view sets and runtime capability snapshots. The display view-set
contract gives eye/view/pose data a clean Morphospace home before Optics adds
projection and homography contracts. The hand provider capability and joint
mapping contracts record what relation and mesh signals a source can report,
and how provider joint indices map into a target bind-joint layout, without
importing OpenXR, Spatial SDK, Makepad, Quest, or renderer dependencies into
Lattice.

## Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

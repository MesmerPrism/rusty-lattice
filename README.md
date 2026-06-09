# Rusty Lattice

Rusty Lattice is the Morphospace relation-contract lane. It describes reference
spaces, tracked poses, view sets, frame-state binding, validity, confidence,
staleness, and capability evidence without importing platform SDKs or renderer
backends.

The first slice contains `rusty-lattice-model`, a minimal model crate for
stereo display view sets. It is intended to unblock Quest Makepad shell
migration by giving eye/view/pose data a clean Morphospace home before Optics
adds projection and homography contracts.

## Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

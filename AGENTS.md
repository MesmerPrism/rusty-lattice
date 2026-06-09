# Rusty Lattice Agent Notes

This is the clean source repository for Rusty Lattice. Keep committed content
self-contained and free of local-only planning paths, downstream app names,
platform-specific runtime handles, SDK imports, and historical naming drift.

Rusty Morphospace is the top-level project/platform umbrella. This repo remains
the Lattice lane inside that umbrella: situated relation contracts for
reference spaces, transforms, tracked poses, view sets, spatial input roles,
frame-state binding, calibration, validity, confidence, and runtime capability
evidence. Do not introduce `rusty.morphospace.*` schemas here; use
`rusty.lattice.*` for Lattice contracts.

Project-owned source in this repo is licensed `AGPL-3.0-or-later`.

## Purpose

Rusty Lattice owns renderer-neutral and platform-neutral relation snapshots. It
does not own Matter simulation truth, Optics projection/appearance policy,
Manifold command/session authority, Quest platform tooling, Makepad event-loop
behavior, OpenXR handles, or device SDK calls.

## Read Order

1. `README.md`
2. `docs/ARCHITECTURE.md`
3. `docs/VALIDATION.md`
4. `fixtures/README.md`

## Architecture Rules

- Lattice owns relation state: spaces, poses, views, spatial roles, frame-state
  binding, validity, confidence, staleness, and capability snapshots.
- Optics owns stereo projection, lenses, homographies, appearance policy, and
  renderer-neutral visual payload preparation.
- Quest and other platform adapters convert SDK/runtime data into Lattice
  contracts outside Lattice core.
- Makepad adapters consume Lattice view sets; they do not make platform
  handles or runtime event loops part of Lattice.
- Keep `src/lib.rs` files as facades or focused model roots. Split before
  mixing independent ownership families.

## Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

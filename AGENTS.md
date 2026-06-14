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

## Sustainable Design Guardrails

- Treat monolithic file pressure as an ownership problem, not a line-count
  problem. Split only by durable authority, schema, route, validation, adapter,
  or test-family boundaries; preserve facades, schema IDs, serde fields,
  fixture outputs, CLI behavior, validation outcomes, and dependency boundaries.
- After a split, update the nearest distributed file map: this `AGENTS.md`,
  `README.md`, `docs/ARCHITECTURE.md`, fixture docs, validation docs, or the
  planning `agent-state\iteration-events.jsonl`.
- Keep `AGENTS.md`, README, and skill files as concise routing indexes. Move
  lane-specific recipes, device/build detail, compatibility ledgers, and long
  validation flows into named docs or runbooks.
- Keep legacy Rusty-XR names as explicit compatibility surfaces only. New
  schemas, routes, and types use the owning lane (`rusty.manifold.*`,
  `rusty.lattice.*`, `rusty.matter.*`, `rusty.optics.*`, `rusty.quest.*`, or
  repo-local names); do not introduce `rusty.morphospace.*` schemas or
  `Morphospace*` core types by default.
## Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

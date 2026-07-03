# Rusty Lattice Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

The gate checks formatting, model tests, display-view and hand-provider fixture
validation through unit tests, and a boundary scan for old naming or
platform-specific runtime dependencies.

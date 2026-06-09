from pathlib import Path

FORBIDDEN = (
    "debug.rustyxr",
    "RUSTY_XR_",
    "rusty.xr.",
    "/rustyxr/v1",
    "rusty_xr_",
    "rusty-xr-",
    "MesmerPrism/Rusty-XR",
    "openxr_sys",
    "ndk_sys",
    "makepad_xr",
)

SCAN_SUFFIXES = {".rs", ".toml", ".json", ".ps1", ".md"}


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    failures: list[str] = []
    for path in root.rglob("*"):
        if "target" in path.parts or ".git" in path.parts:
            continue
        if path.suffix not in SCAN_SUFFIXES:
            continue
        text = path.read_text(encoding="utf-8")
        for token in FORBIDDEN:
            if token in text:
                failures.append(f"{path.relative_to(root)} contains forbidden token {token}")
    if failures:
        print("\n".join(failures))
        return 1
    print("Rusty Lattice boundary scan passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

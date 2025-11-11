#!/usr/bin/env python3
from pathlib import Path
from typing import Dict


def main():
    # Get current directory
    cwd = Path(".")

    # Find all .py and .rs files
    py_files: Dict[str, Path] = {f.stem: f for f in cwd.glob("*.py")}
    rs_files: Dict[str, Path] = {f.stem: f for f in cwd.glob("*.rs")}

    # Get all unique basenames (union of both sets)
    all_basenames = py_files.keys() | rs_files.keys()

    # Process each basename
    for basename in all_basenames:
        # Create directory
        target_dir = cwd / basename
        target_dir.mkdir(exist_ok=True)

        # Track which files we moved
        files = []

        # Move Python file if it exists
        if basename in py_files:
            py_files[basename].rename(target_dir / "main.py")
            files.append("main.py")

        # Move Rust file if it exists
        if basename in rs_files:
            rs_files[basename].rename(target_dir / "main.rs")
            files.append("main.rs")

        print(f"Created {basename}/ with {' and '.join(files)}")

    print("Done!")


if __name__ == "__main__":
    main()

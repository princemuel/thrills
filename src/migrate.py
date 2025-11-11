#!/usr/bin/env python3
from pathlib import Path
from typing import Dict


def main():
    # Get current directory
    cwd = Path(".")

    # Find all .py files and check for corresponding .rs files
    py_files: Dict[str, Path] = {f.stem: f for f in cwd.glob("*.py")}
    rs_files: Dict[str, Path] = {f.stem: f for f in cwd.glob("*.rs")}

    # Find paired files (exist in both sets)
    paired = py_files.keys() & rs_files.keys()

    # Process each pair
    for basename in paired:
        # Create directory
        target_dir = cwd / basename
        target_dir.mkdir(exist_ok=True)

        # Move files
        py_files[basename].rename(target_dir / "main.py")
        rs_files[basename].rename(target_dir / "main.rs")

        print(f"Created {basename}/ with main.py and main.rs")

    print("Done!")


if __name__ == "__main__":
    main()

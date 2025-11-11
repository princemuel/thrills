#!/usr/bin/env bash
set -euo pipefail

# Collect all unique basenames from both .py and .rs files
declare -A basenames

for file in *.py *.rs; do
    [[ -e "$file" ]] || continue

    # Extract basename without extension
    basename="${file%.*}"
    basenames["$basename"]=1
done

# Process each unique basename
for basename in "${!basenames[@]}"; do
    pyfile="${basename}.py"
    rsfile="${basename}.rs"

    # Create directory
    mkdir -p "$basename"

    # Move Python file if it exists
    if [[ -e "$pyfile" ]]; then
        mv "$pyfile" "${basename}/main.py"
        py_status="main.py"
    else
        py_status=""
    fi

    # Move Rust file if it exists
    if [[ -e "$rsfile" ]]; then
        mv "$rsfile" "${basename}/main.rs"
        rs_status="main.rs"
    else
        rs_status=""
    fi

    # Build status message
    files="$py_status${py_status:+ }${py_status:+and }$rs_status"
    echo "Created ${basename}/ with ${files}"
done

echo "Done!"

#!/bin/bash

# Directory containing the lesson files
LESSONS_DIR="lessons"

# Find all .rs files and Cargo projects in the lessons directory
echo "===================================================================="
echo "Building all lessons..."
echo "===================================================================="
for dir in "$LESSONS_DIR"/*; do
    if [ -d "$dir" ]; then
        if [ -f "$dir/src/main.rs" ]; then
            echo "Building Cargo project in $dir..."
            (cd "$dir" && cargo build)
            if [ $? -eq 0 ]; then
                echo "Build successful: $dir"
            else
                echo "Build failed: $dir"
            fi
        fi
    elif [ -f "$dir" ] && [[ "$dir" == *.rs ]]; then
        echo "Building $dir..."
        rustc "$dir" -o "${dir%.rs}.out"
        if [ $? -eq 0 ]; then
            echo "Build successful: ${dir%.rs}.out"
        else
            echo "Build failed: $dir"
        fi
    fi
done
echo "===================================================================="
echo "Build process complete."
echo "===================================================================="
# End of build_all_lessons.sh
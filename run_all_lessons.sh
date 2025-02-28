#!/bin/bash

# Directory containing the lesson files
LESSONS_DIR="/home/noah/Documents/Tutorial/LearnRust/lessons"

echo "===================================================================="
echo "Executing all lessons..."
echo "===================================================================="
# Find all .out files and Cargo project executables in the lessons directory
for dir in "$LESSONS_DIR"/*; do
    if [ -d "$dir" ]; then
        if [ -f "$dir/target/debug/$(basename "$dir")" ]; then
            executable="$dir/target/debug/$(basename "$dir")"
            echo "Executing $executable..."
            "$executable"
            if [ $? -eq 0 ]; then
                echo "Execution successful: $executable"
            else
                echo "Execution failed: $executable"
            fi
        fi
    elif [ -f "$dir" ] && [[ "$dir" == *.out ]]; then
        echo "Executing $dir..."
        "$dir"
        if [ $? -eq 0 ]; then
            echo "Execution successful: $dir"
        else
            echo "Execution failed: $dir"
        fi
    else
        echo "No executable files found in $LESSONS_DIR"
    fi
done
echo "===================================================================="
echo "Execution process complete."
echo "===================================================================="
# End of run_all_lessons.sh
#!/bin/bash

# Directory containing the lesson files
LESSONS_DIR="lessons"

# Function to display the menu and get the user's choice
select_option() {
    PS3="Please select an option: "
    options=("$@")
    select opt in "${options[@]}" "Quit"; do
        if (( REPLY == 1 + ${#options[@]} )); then
            echo "Exiting..."
            exit 0
        elif (( REPLY > 0 && REPLY <= ${#options[@]} )); then
            echo "You selected: $opt"
            return $REPLY
        else
            echo "Invalid option. Try another one."
        fi
    done
}

# Get a list of all Rust files and Cargo packages
items=()
for dir in "$LESSONS_DIR"/*; do
    if [ -d "$dir" ] && [ -f "$dir/src/main.rs" ]; then
        items+=("Cargo package: $(basename "$dir")")
    elif [ -f "$dir" ] && [[ "$dir" == *.rs ]]; then
        items+=("Rust file: $(basename "$dir")")
    fi
done

# Display the menu and get the user's choice
echo "Select a Rust file or Cargo package to build, run, or build and run:"
select_option "${items[@]}"
choice=$?

# Get the selected item
selected_item="${items[$choice-1]}"

# Determine the action to perform
echo "Select an action:"
actions=("Build" "Check" "Run" "Build and Run")
select_option "${actions[@]}"
action=$?

# Perform the selected action
case $selected_item in
    "Cargo package: "*)
        package_name="${selected_item#Cargo package: }"
        package_dir="$LESSONS_DIR/$package_name"
        case $action in
            1)
                echo "Building Cargo package: $package_name..."
                (cd "$package_dir" && cargo build)
                ;;
            2)
                echo "Checking Cargo package: $package_name..."
                (cd "$package_dir" && cargo check)
                ;;
            3)
                echo "Running Cargo package: $package_name..."
                (cd "$package_dir" && cargo run)
                ;;
            4)
                echo "Building and running Cargo package: $package_name..."
                (cd "$package_dir" && cargo build && cargo run)
                ;;
        esac
        ;;
    "Rust file: "*)
        rust_file="${selected_item#Rust file: }"
        rust_file_path="$LESSONS_DIR/$rust_file"
        output_file="${rust_file_path%.rs}.out"
        case $action in
            1)
                echo "Building Rust file: $rust_file..."
                rustc "$rust_file_path" -o "$output_file"
                ;;
            2)
                echo "Running Rust file: $rust_file..."
                "$output_file"
                ;;
            3)
                echo "Building and running Rust file: $rust_file..."
                rustc "$rust_file_path" -o "$output_file" && "$output_file"
                ;;
        esac
        ;;
esac
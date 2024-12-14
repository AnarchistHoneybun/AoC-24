#!/bin/bash

# Check if the input number is provided
if [ -z "$1" ]; then
  echo "Please provide a number."
  exit 1
fi

# Parse the --type flag
LANGUAGE=""
if [ "$2" == "--type" ] && [ -n "$3" ]; then
  LANGUAGE=$3
fi

# Create the Day_<input_number> folder in the current directory
mkdir "Day_$1"

# Create the day<input_number> folder in the input directory
mkdir -p "input/day$1"

# Create full.txt and sample.txt inside the day<input_number> folder
touch "input/day$1/full.txt" "input/day$1/sample.txt"

# Set up the folder according to the specified language
if [ "$LANGUAGE" == "rust" ]; then
  (cd "Day_$1" && cargo init "day_$1" --vcs none)
fi

echo "Folders Day_$1 and input/day$1 created successfully."
echo "Files full.txt and sample.txt created inside input/day$1."
if [ "$LANGUAGE" == "rust" ]; then
  echo "Rust project initialized in Day_$1/day_$1."
fi
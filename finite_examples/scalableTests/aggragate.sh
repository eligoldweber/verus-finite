#!/bin/bash

# Check if the correct number of arguments is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 input_csv_file"
    exit 1
fi

input_csv_file=$1  # Get the input CSV file from the command line argument

# Check if the input CSV file exists
if [ ! -f "$input_csv_file" ]; then
    echo "Error: File '$input_csv_file' not found."
    exit 1
fi

# Get the directory and base name of the input CSV file
input_dir=$(dirname "$input_csv_file")
input_base=$(basename "$input_csv_file" .csv)

# Summary CSV file
summary_csv_file="$input_dir/${input_base}_summary.csv"
echo "ID,Sum of Averages,Standard Deviation" > "$summary_csv_file"  # Initialize summary CSV file

# Temporary file to store trial times
temp_file=$(mktemp)

# Read the original CSV file, skipping the header and capturing IDs
declare -A unique_ids
while IFS=, read -r id rest; do
    if [[ "$id" != "ID" ]]; then  # Skip header
        echo "$id,$rest" >> "$temp_file"  # Store ID with trial times
        unique_ids["$id"]=1  # Mark ID as seen
    fi
done < "$input_csv_file"

# Collect and sort the unique IDs numerically
sorted_ids=($(echo "${!unique_ids[@]}" | tr ' ' '\n' | sort -n))

# Process each unique ID in sorted order
for id in "${sorted_ids[@]}"; do
    # Initialize sums and counts
    sum_of_averages=0
    count=0

    while IFS=, read -r curr_id rest; do
        if [ "$curr_id" == "$id" ]; then
            IFS=',' read -r -a times <<< "$rest"
            if [[ "${#times[@]}" -gt 1 ]]; then  # Ensure there are enough elements
                avg="${times[${#times[@]}-2]}"  # Get the average from the second-to-last position
                if [[ "$avg" != "-1" ]]; then  # Only sum valid averages
                    sum_of_averages=$(echo "$sum_of_averages + $avg" | bc)
                    count=$((count + 1))
                fi
            fi
        fi
    done < "$temp_file"

    # Calculate population standard deviation from the averages
    stddev=0
    if (( count > 0 )); then
        mean=$(echo "scale=2; $sum_of_averages / $count" | bc)
        sum_of_squares=0

        # Calculate the squared differences
        while IFS=, read -r curr_id rest; do
            if [ "$curr_id" == "$id" ]; then
                IFS=',' read -r -a times <<< "$rest"
                if [[ "${#times[@]}" -gt 1 ]]; then
                    avg="${times[${#times[@]}-2]}"
                    if [[ "$avg" != "-1" ]]; then
                        sum_of_squares=$(echo "$sum_of_squares + ($avg - $mean)^2" | bc)
                    fi
                fi
            fi
        done < "$temp_file"

        # Final calculation of standard deviation
        variance=$(echo "scale=2; $sum_of_squares / $count" | bc)
        stddev=$(echo "scale=2; sqrt($variance)" | bc)
    fi

    # Store results in the summary CSV
    echo "$id,$sum_of_averages,$stddev" >> "$summary_csv_file"
done

# Cleanup temporary file
rm "$temp_file"

echo "Summary CSV created at: $summary_csv_file"

#!/bin/bash

# Check if the correct number of arguments is provided
if [ $# -ne 4 ]; then
    echo "Usage: $0 input_file n num_executions script_to_run"
    exit 1
fi

input_file=$1  # Get the input file from the command line argument
n=$2           # Get the upper limit from the command line argument
num_executions=$3  # Number of Verus executions per generated file
script_to_run=$4  # Python script to run (e.g., bsearch)

# Get the directory of the input file
input_dir=$(dirname "$input_file")
input_base=$(basename "$input_file" .rs)  # Strip the .rs extension

# Cleanup: Remove all previously created output files
echo "Cleaning up temporary files..."
rm -f "$input_dir/../tempFiles/"*.rs
rm -f "$input_dir/../tempFiles/verus_output_"*.txt  # Cleanup any previous Verus output files
rm -f "$input_dir/"*.csv

# CSV file to log total time
csv_file="$input_dir/${input_base}_time_log.csv"
echo "ID,$(seq -s ',' 1 $num_executions)Average,Standard Deviation" > "$csv_file"  # Initialize the CSV file

# Loop from 1 to n
for (( int_param=1; int_param<=n; int_param++ ))
do
    echo "Processing int_param = $int_param"
    
    # Set the output directory for the Python script
    output_dir="$input_dir/../tempFiles/int_param_$int_param/"

    # Create the output directory if it doesn't exist
    mkdir -p "$output_dir"

    # Call the selected Python script based on the parameter
    if [ "$script_to_run" == "bsearch" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/bsearch/scaleTestBSearchSeparate.py "$input_file" "$output_dir" "$int_param"
    else
        echo "Unknown script specified: $script_to_run."
        exit 1
    fi

    # Loop for each generated binary_search_finite_k file
    for (( k=0; k<int_param; k++ ))
    do
        # Initialize total time for averaging
        total_time=0
        successful_executions=0
        trial_times=""  # String to hold times for each trial

        for (( exec=1; exec<=num_executions; exec++ ))
        do
            temp_output_file="$input_dir/../tempFiles/verus_output_${int_param}_k_${k}_exec_${exec}.txt"  # Temporary output file
            output_file="$output_dir/binary_search_finite_${k}.rs"
            
            echo "Calling Verus for $output_file (Execution $exec)"

            # Execute the Verus command, directing output to a temp file
            ./target-verus/release/verus --log-all --time-expanded --rlimit=30000000 "$output_file" > "$temp_output_file" 2>&1
            
            # Check the Verus output file
            if grep -q "0 errors" "$temp_output_file"; then
                # Extract total time
                total_time_exec=$(grep "total-time:" "$temp_output_file" | awk '{print $2}')
                total_time=$(echo "$total_time + $total_time_exec" | bc)  # Accumulate total time
                successful_executions=$((successful_executions + 1))  # Count successful executions
                trial_times+="$total_time_exec "  # Add the execution time to the string
            else
                total_time_exec=-1  # Log -1 if there are errors
                trial_times+="-1 "  # Add -1 for this trial
            fi
        done

        # Calculate standard deviation based solely on trial times (excluding the average)
        if (( successful_executions > 0 )); then
            sum=0
            sum_of_squares=0
            IFS=' ' read -r -a trial_times_array <<< "$trial_times"

            for time in "${trial_times_array[@]}"; do
                if [[ "$time" != "-1" ]]; then  # Only include valid times
                    sum=$(echo "$sum + $time" | bc)
                    sum_of_squares=$(echo "$sum_of_squares + ($time * $time)" | bc)
                fi
            done
            
            mean=$(echo "scale=2; $sum / $successful_executions" | bc)
            variance=$(echo "scale=2; ($sum_of_squares / $successful_executions) - ($mean * $mean)" | bc)
            stddev=$(echo "scale=2; sqrt($variance)" | bc)
        else
            stddev=-1
        fi 

        # Calculate average time after standard deviation
        if (( successful_executions > 0 )); then
            average_time=$(echo "scale=2; $total_time / $successful_executions" | bc)
        else
            average_time=-1
        fi

        # Prepare the CSV row
        row="$int_param,$(IFS=,; echo "${trial_times_array[*]}"),$average_time,$stddev"
        echo "$row" >> "$csv_file"  # Log trials, average, and standard deviation to CSV
    done
done

# Summary CSV calculation
summary_csv_file="$input_dir/${input_base}_summary.csv"
echo "ID,Sum of Averages,Standard Deviation" > "$summary_csv_file"  # Initialize summary CSV file

# Temporary file to store trial times
temp_file=$(mktemp)

# Read the original CSV file, skipping the header
while IFS=, read -r id rest; do
    if [[ "$id" != "ID" ]]; then  # Skip header
        echo "$id,$rest" >> "$temp_file"  # Store ID with trial times
    fi
done < "$csv_file"

# Sort the temporary file by ID
sort -t, -k1,1 "$temp_file" -o "$temp_file"

# Process each unique ID
ids=()  # Initialize an array for unique IDs
while IFS=, read -r id rest; do
    if [[ ! " ${ids[*]} " =~ " $id " ]]; then
        ids+=("$id")  # Add ID to list if not already present
        
        # Extract averages for the current ID
        sum_of_averages=0
        count=0

        while IFS=, read -r curr_id rest; do
            if [ "$curr_id" == "$id" ]; then
                IFS=',' read -r -a times <<< "$rest"
                if [[ "${#times[@]}" -gt 1 ]]; then  # Ensure there are enough elements
                    # The average is the second-to-last element
                    avg="${times[${#times[@]}-2]}"
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
    fi
done < "$temp_file"

# Cleanup temporary file
rm "$temp_file"

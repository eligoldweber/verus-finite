#!/bin/bash

# Check if the correct number of arguments is provided
if [ $# -ne 4 ]; then
    echo "Usage: $0 input_file n num_executions script_to_run"
    exit 1
fi

input_file=$1  # Get the input file from the command line argument
n=$2           # Get the upper limit from the command line argument
num_executions=$3  # Number of Verus executions per generated file
script_to_run=$4  # Python script to run (prime or triangle)

# Get the directory of the input file
input_dir=$(dirname "$input_file")
input_base=$(basename "$input_file" .rs)  # Strip the .rs extension

# Cleanup: Remove all previously created output files
echo "Cleaning up temporary files..."
rm -f "$input_dir/../tempFiles/${input_base}_output_"*.rs
rm -f "$input_dir/../tempFiles/verus_output_"*.txt  # Cleanup any previous Verus output files
rm -f "$input_dir/"*.csv

# CSV file to log total time
csv_file="$input_dir/${input_base}_time_log.csv"
echo "ID,$(seq -s ',' 1 $num_executions)Average,Standard Deviation" > "$csv_file"  # Initialize the CSV file

# Loop from 1 to n
for (( int_param=1; int_param<=n; int_param++ ))
do
    echo "Processing int_param = $int_param"
    
    # Set the output file path
    output_file="$input_dir/../tempFiles/${input_base}_output_$int_param.rs"
    
    # Call the selected Python script based on the parameter
    if [ "$script_to_run" == "prime" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/prime/scaleTestPrime.py "$input_file" "$output_file" "$int_param"
    elif [ "$script_to_run" == "primeDataType" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/primeDataType/scaleTestPrimeDataType.py "$input_file" "$output_file" "$int_param"
    elif [ "$script_to_run" == "primeOverflow" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/primeOverflow/scaleTestPrimeOverflow.py "$input_file" "$output_file" "$int_param"
    elif [ "$script_to_run" == "triangle" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/triangle/scaleTestTriangle.py "$input_file" "$output_file" "$int_param"
    elif [ "$script_to_run" == "triangle3" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/triangle/scaleTestTriangle3.py "$input_file" "$output_file" "$int_param"
    elif [ "$script_to_run" == "indexUpTo" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/indexUpTo/scaleTestIndexUpTo.py "$input_file" "$output_file" "$int_param"
    elif [ "$script_to_run" == "indexUpToOverflow" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/indexUpTo/scaleTestIndexUpToOverflow.py "$input_file" "$output_file" "$int_param"
    elif [ "$script_to_run" == "bsearch" ]; then
        /usr/bin/python3 ../finite_examples/scalableTests/bsearch/scaleTestBSearch.py "$input_file" "$output_file" "$int_param"
    else
        echo "Unknown script specified: $script_to_run."
        exit 1
    fi
done

# Loop to call Verus command for each int_param and each execution
for (( int_param=1; int_param<=n; int_param++ ))
do
    # Initialize total time for averaging
    total_time=0
    successful_executions=0
    trial_times=()  # Array to hold times for each trial

    for (( exec=1; exec<=num_executions; exec++ ))
    do
        temp_output_file="$input_dir/../tempFiles/verus_output_${int_param}_exec_${exec}.txt"  # Temporary output file
        output_file="$input_dir/../tempFiles/${input_base}_output_$int_param.rs"
        
        echo "Calling Verus for $output_file (Execution $exec)"

        # Execute the Verus command, directing output to a temp file
        ./target-verus/release/verus --log-all --time-expanded --rlimit=30000000 "$output_file" > "$temp_output_file" 2>&1
        
        # Check the Verus output file
        if grep -q "0 errors" "$temp_output_file"; then
            # Extract total time
            total_time_exec=$(grep "total-time:" "$temp_output_file" | awk '{print $2}')
            total_time=$(echo "$total_time + $total_time_exec" | bc)  # Accumulate total time
            successful_executions=$((successful_executions + 1))  # Count successful executions
            trial_times+=("$total_time_exec")  # Add the execution time to the array
        else
            total_time_exec=-1  # Log -1 if there are errors
            trial_times+=("$total_time_exec")  # Add -1 for this trial
        fi
    done

    # Calculate standard deviation based solely on trial times (excluding the average)
    if (( successful_executions > 0 )); then
        sum=0
        sum_of_squares=0
        # echo "Calculating standard deviation for trials:"

        for time in "${trial_times[@]}"; do
            if [ "$time" -ne -1 ]; then  # Only include valid times
                # echo "Trial time: $time"
                sum=$(echo "$sum + $time" | bc)
                sum_of_squares=$(echo "$sum_of_squares + ($time * $time)" | bc)
            fi
        done
        
        if [ "$successful_executions" -gt 0 ]; then
            mean=$(echo "scale=2; $sum / $successful_executions" | bc)
            variance=$(echo "scale=2; ($sum_of_squares / $successful_executions) - ($mean * $mean)" | bc)
            stddev=$(echo "scale=2; sqrt($variance)" | bc)
            # echo "Calculated standard deviation: $stddev"
        else
            stddev=-1
        fi
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
    row="$int_param,$(IFS=,; echo "${trial_times[*]}"),$average_time,$stddev"
    echo "$row" >> "$csv_file"  # Log trials, average, and standard deviation to CSV
done

# gnuplot section remains unchanged

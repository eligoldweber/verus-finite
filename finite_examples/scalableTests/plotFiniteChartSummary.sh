#!/bin/bash

# Check if the correct number of arguments is provided
if [ $# -ne 3 ]; then
    echo "Usage: $0 input_file n num_executions"
    exit 1
fi

input_file=$1  # Get the input file from the command line argument
n=$2           # Get the upper limit from the command line argument
num_executions=$3  # Number of Verus executions per generated file

# Get the directory of the input file
input_dir=$(dirname "$input_file")
input_base=$(basename "$input_file" .rs)  # Strip the .rs extension

# CSV file paths
time_log_csv="$input_dir/${input_base}_time_log.csv"
summary_csv="$input_dir/${input_base}_summary.csv"

# Generate the plot for the summary CSV
gnuplot << EOF
set terminal pngcairo size 3200,1600 enhanced
set output '$input_dir/${input_base}_summary_plot.png'
set title "Average and Standard Deviation for Each ID"
set xlabel "ID"
set ylabel "Time (ms)"
set grid

# Set datafile separator
set datafile separator ","

# Calculate the maximum value for y range based on average time
stats "$summary_csv" using 2 nooutput

# Ensure the max value is greater than -1
set yrange [-1:STATS_max_y]

# Plot the average and standard deviation
plot "$summary_csv" using 1:2:3 with errorbars title 'Average Total Time' lc rgb "blue" pointtype 5
    #  '' using 1:3 with errorbars title 'Standard Deviation' lc rgb "red" pt 5

    #  plot "$csv_file" using 1:$(($num_executions + 1)):$(($num_executions + 3)) with errorbars title 'Average Total Time' lc rgb "blue" pointtype 5

EOF

echo "Plot generated: ${input_base}_summary_plot.png"

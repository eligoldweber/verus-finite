#!/bin/bash

# Check if the correct number of arguments is provided
if [ $# -ne 3 ]; then
    echo "Usage: $0 input_file n num_executions"
    exit 1
fi



input_file=$1  # Get the input file from the command line argument
n=$2  # Get the upper limit from the command line argument
num_executions=$3  # Number of Verus executions per generated file

# Get the directory of the input file
input_dir=$(dirname "$input_file")
input_base=$(basename "$input_file" .rs)  # Strip the .rs extension

csv_file="$input_dir/${input_base}_time_log.csv"


# gnuplot << EOF
# set terminal pngcairo enhanced
# set output '$input_dir/${input_base}_time_log_plot.png'
# set title "Average Verification Time For Prime Finite (5 trials)"
# set xlabel "Finitized Size"
# set ylabel "Average Total Time (ms)"
# set grid

# # Calculate the maximum total time for setting y range
# stats "$csv_file" using $(($num_executions + 1)) nooutput

# # Ensure the max value is greater than -1
# set yrange [-1:STATS_max_y]

# # Set datafile separator
# set datafile separator ","

# # Plot the average data from the CSV file, skipping the header
# plot "$csv_file" using 1:$(($num_executions + 1)) with linespoints title 'Average Total Time' 
# EOF

# echo "Plot generated: ${input_base}_time_log_plot.png"
# echo "Processing and Verus calls completed."


gnuplot << EOF
set terminal pngcairo size 3200,1600 enhanced
set output '$input_dir/${input_base}_time_log_plot.png'
set title "Average Verification Time For Prime (Data Type) Finite (5 trials)"
set xlabel "Finitized Size"
set ylabel "Average Total Time (ms)"
set grid


# Calculate the maximum total time for setting y range based on the average time column
stats "$csv_file" using $(($num_executions + 1)) nooutput

# Ensure the max value is greater than -1
set yrange [-1:STATS_max_y]

# Set datafile separator
set datafile separator ","

# Plot the average data from the CSV file, skipping the header
# plot "$csv_file" using 1:$(($num_executions + 1)) with linespoints title 'Average Total Time' lc rgb "blue" 
# EOF
# plot "$csv_file" using 1:$(($num_executions + 1)):($(($num_executions + 2)) * 1) with errorbars title 'Average Total Time' lc rgb "blue" pointtype 7
# plot "$csv_file" using 2:$(($num_executions + 2)):$(($num_executions + 3)) with errorbars title 'Average Total Time' lc rgb "blue" pointtype 5
plot "$csv_file" using 1:$(($num_executions + 1)):$(($num_executions + 3)) with errorbars title 'Average Total Time' lc rgb "blue" pointtype 5

# plot "$csv_file" using 1:$(($num_executions + 1)) with linespoints title 'Average Total Time' lc rgb "blue", '' using 1:$(($num_executions + 2)) with linespoints title 'Standard Deviation' lc rgb "red"
EOF
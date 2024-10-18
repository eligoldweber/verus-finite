#!/bin/bash

# # Check if the correct number of arguments is provided
# if [ $# -ne 4 ]; then
#     echo "Usage: $0 csv_file1 csv_file2 n num_executions"
#     exit 1
# fi

# csv_file1=$1  # Get the first CSV file from the command line argument
# csv_file2=$2  # Get the second CSV file from the command line argument
# n=$3          # Get the upper limit from the command line argument
# num_executions=$4  # Number of Verus executions per generated file

# # Get the directory of the first CSV file
# input_dir1=$(dirname "$csv_file1")
# input_base1=$(basename "$csv_file1" .csv)  # Strip the .csv extension

# # Get the directory of the second CSV file
# input_dir2=$(dirname "$csv_file2")
# input_base2=$(basename "$csv_file2" .csv)  # Strip the .csv extension

# # Set the output file name and directory
# output_file="$input_dir1/combined_time_log_plot.png"

# gnuplot << EOF
# set terminal pngcairo size 3200,1600 enhanced
# set output '$output_file'
# set title "Average Verification Time Comparison"
# set xlabel "Finitized Size"
# set ylabel "Average Total Time (ms)"
# set grid

# # Calculate the maximum total time for setting y range based on the average time column from both CSV files
# stats "$csv_file1" using $(($num_executions + 1)) nooutput
# stats "$csv_file2" using $(($num_executions + 1)) nooutput

# # Ensure the max value is greater than -1
# set yrange [-1:STATS_max_y]

# # Set datafile separator
# set datafile separator ","

# # Plot the average data from both CSV files, skipping the header
# plot "$csv_file1" using 1:$(($num_executions + 1)):($(($num_executions + 2)) * 1) with errorbars title '${input_base1} Average' lc rgb "blue" pointtype 7, \
#      "$csv_file2" using 1:$(($num_executions + 1)):($(($num_executions + 2)) * 1) with errorbars title '${input_base2} Average' lc rgb "red" pointtype 7
# EOF

# echo "Plot generated: $output_file"
# echo "Processing and Verus calls completed."

###########


#!/bin/bash

# Check if the correct number of arguments is provided
if [ $# -ne 4 ]; then
    echo "Usage: $0 csv_file1 csv_file2 csv_file3 csv_file4 n num_executions"
    exit 1
fi

csv_file1=$1  # Get the first CSV file from the command line argument
csv_file2=$2  # Get the second CSV file from the command line argument
n=$3          # Get the upper limit from the command line argument
num_executions=$4  # Number of Verus executions per generated file

# Get the directory of the first CSV file for the output
input_dir1=$(dirname "$csv_file1")
output_file="$input_dir1/combined_BSearch_time_log_plot.png"

gnuplot << EOF
set terminal pngcairo size 3200,1600 enhanced
set output '$output_file'
set title "Average Verification Time Comparison"
set xlabel "Finitized Size"
set ylabel "Average Total Time (ms)"
set grid

# Calculate the maximum total time for setting y range based on the average time column from all CSV files
stats "$csv_file1" using $(($num_executions + 1)) nooutput
stats "$summary_csv" using 2 nooutput

# Ensure the max value is greater than -1
set yrange [-1:STATS_max_y]

# Set datafile separator
set datafile separator ","
# Draw horizontal lines
set arrow from 0, 1000 to 100, 1000 nohead lt 1 lc rgb "red" lw 1
set arrow from 0, 5000 to 100, 5000 nohead lt 1 lc rgb "green" lw 1
set arrow from 0, 10000 to 100, 10000 nohead lt 1 lc rgb "blue" lw 1

# Plot the average data from all four CSV files, skipping the header
plot "$csv_file1" using 1:$(($num_executions + 1)):($(($num_executions + 2)) * 1) with errorbars title 'Prime Average' lc rgb "blue" pointtype 3, \
     "$csv_file2" using 1:2:3 with errorbars title 'Average Total Time' lc rgb "red" pointtype 3

     
EOF

echo "Plot generated: $output_file"
echo "Processing and Verus calls completed."

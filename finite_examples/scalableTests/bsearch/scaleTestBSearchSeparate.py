import argparse
import os

def read_file(filename):
    with open(filename, 'r') as file:
        return file.readlines()

def write_file(filename, lines):
    with open(filename, 'w') as file:
        file.writelines(lines)

def transform_function(lines, int_param, output_dir):
    # Prepare the preamble that will be included in each output file
    preamble = [
        "#![allow(unused_imports)]\n",
        "use builtin::*;\n",
        "use builtin_macros::*;\n",
        "use vstd::{prelude::*, seq::*};\n",
        "\nverus! {\n"
    ]
    
    # Prepare the main function template
    main_function = "    fn main()\n    {\n    }\n}\n"  # Empty main function

    # Generate binary_search_finite_k functions
    for k in range(int_param):
        new_lines = preamble.copy()  # Start with the preamble
        new_lines.append(f"    fn binary_search_finite_{k}() {{\n")  # Function header
        new_lines.append("        let mut v: Vec<u64> = Vec::new();\n")  # Create the vector
        for value in range(int_param):
            new_lines.append(f"        v.push({value});\n")  # Populate the vector with values
        new_lines.append(f"        let k: u64 = {k};\n")
        
        # Unroll the while loop int_param + 1 times
        new_lines.append("        let mut i1: usize = 0;\n")
        new_lines.append("        let mut i2: usize = v.len() - 1;\n")
        
        for i in range(int_param + 1):
            new_lines.append("        if i1 != i2 {\n")
            new_lines.append("            let ix = i1 + (i2 - i1) / 2;\n")
            new_lines.append("            if v[ix] < k {\n")
            new_lines.append("                i1 = ix + 1;\n")
            new_lines.append("            } else {\n")
            new_lines.append("                i2 = ix;\n")
            new_lines.append("            }\n")
            new_lines.append("        }\n")
        
        new_lines.append("        let r = i1;\n")
        new_lines.append("        assert(r == k);\n")
        new_lines.append("    }\n\n")  # Close each finite function

        # Add the main function
        new_lines.append(main_function)  # Add the main function

        # Write each finite function to its own file
        finite_file_path = os.path.join(output_dir, f"binary_search_finite_{k}.rs")
        write_file(finite_file_path, new_lines)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Transform a Verus function definition.")
    parser.add_argument("input_file", help="The name of the file to read")
    parser.add_argument("output_dir", help="The directory to write the transformed content to")
    parser.add_argument("int_param", type=int, help="The integer parameter for the finite function")

    args = parser.parse_args()

    # Ensure the output directory exists
    if not os.path.exists(args.output_dir):
        os.makedirs(args.output_dir)

    # Verify that output_dir is indeed a directory
    if not os.path.isdir(args.output_dir):
        raise NotADirectoryError(f"{args.output_dir} is not a valid directory.")

    lines = read_file(args.input_file)
    transform_function(lines, args.int_param, args.output_dir)

import argparse

def read_file(filename):
    with open(filename, 'r') as file:
        return file.readlines()

def write_file(filename, lines):
    with open(filename, 'w') as file:
        file.writelines(lines)

def transform_function(lines, int_param):
    new_lines = []
    inside_main = False

    for line in lines:
        # Check for the start of the main function
        if "fn main()" in line:
            inside_main = True
            new_lines.append(line)  # Keep the function signature
            new_lines.append("    {\n")  # Add the opening brace
            new_lines.append("    }\n")  # Add the closing brace
            continue
        
        # Skip the lines inside the main function
        if inside_main:
            if "}" in line:  # Check for the end of the main function
                inside_main = False  # Exit the main function
            continue  # Skip all lines within the main function
        
        # Comment out the original binary_search function
        if "fn binary_search(v: &Vec<u64>, k: u64)" in line:
            new_lines.append("/*\n")  # Start comment block
            new_lines.append(line)  # Original function signature
            new_lines.append("    requires\n")  # Include requires
            new_lines.append("        forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],\n")
            new_lines.append("        exists|i: int| 0 <= i < v.len() && k == v[i],\n")
            new_lines.append("    ensures\n")
            new_lines.append("        r < v.len(),\n")
            new_lines.append("        k == v[r as int],\n")
            new_lines.append("{\n")
            new_lines.append("    let mut i1: usize = 0;\n")
            new_lines.append("    let mut i2: usize = v.len() - 1;\n")
            new_lines.append("    while i1 != i2\n")
            new_lines.append("        invariant\n")
            new_lines.append("            i2 < v.len(),\n")
            new_lines.append("            exists|i: int| i1 <= i <= i2 && k == v[i],\n")
            new_lines.append("            forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],\n")
            new_lines.append("    {\n")
            new_lines.append("        let ix = i1 + (i2 - i1) / 2;\n")
            new_lines.append("        if v[ix] < k {\n")
            new_lines.append("            i1 = ix + 1;\n")
            new_lines.append("        } else {\n")
            new_lines.append("            i2 = ix;\n")
            new_lines.append("        }\n")
            new_lines.append("    }\n")
            new_lines.append("    i1\n")
            new_lines.append("}*/\n\n")  # End comment block

        # Generate binary_search_finite_k functions
        if "fn binary_search(v: &Vec<u64>, k: u64)" in line:
            for k in range(int_param):
                new_lines.append(f"fn binary_search_finite_{k}() {{\n")
                new_lines.append("    let mut v: Vec<u64> = Vec::new();\n")  # Create the vector
                for value in range(int_param):
                    new_lines.append(f"    v.push({value});\n")  # Populate the vector with values
                new_lines.append(f"    let k: u64 = {k};\n")
                
                # Unroll the while loop int_param + 1 times
                new_lines.append("    let mut i1: usize = 0;\n")
                new_lines.append("    let mut i2: usize = v.len() - 1;\n")
                
                for i in range(int_param + 1):
                    new_lines.append(f"    if i1 != i2 {{\n")
                    new_lines.append(f"        let ix = i1 + (i2 - i1) / 2;\n")
                    new_lines.append(f"        if v[ix] < k {{\n")
                    new_lines.append(f"            i1 = ix + 1;\n")
                    new_lines.append(f"        }} else {{\n")
                    new_lines.append(f"            i2 = ix;\n")
                    new_lines.append(f"        }}\n")
                    new_lines.append(f"    }}\n")
                
                new_lines.append("    let r = i1;\n")
                new_lines.append("    assert(r == k);\n")
                new_lines.append("}\n\n")  # Close each finite function

        new_lines.append(line)

    return new_lines

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Transform a Verus function definition.")
    parser.add_argument("input_file", help="The name of the file to read")
    parser.add_argument("output_file", help="The name of the file to write the transformed content to")
    parser.add_argument("int_param", type=int, help="The integer parameter for the finite function")

    args = parser.parse_args()

    lines = read_file(args.input_file)
    transformed_lines = transform_function(lines, args.int_param)
    write_file(args.output_file, transformed_lines)

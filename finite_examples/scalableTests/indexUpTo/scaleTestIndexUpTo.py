import argparse

def read_file(filename):
    with open(filename, 'r') as file:
        return file.readlines()

def write_file(filename, lines):
    with open(filename, 'w') as file:
        file.writelines(lines)

def transform_function(lines, int_param):
    new_lines = []
    function_started = False

    for line in lines:
        # Identify the start of the function to replace
        if "fn indexUpTo(n:u32)" in line:
            new_lines.append(f"fn indexUpTo_finite_{int_param}() -> (f: Vec<u32>)\n")
            new_lines.append(f"    ensures f.len() == {int_param}, \n")
            new_lines.append(f"             f[0] == 0,\n")
            new_lines.append(f"             f[{int_param}-1] != 0,\n")
            new_lines.append("{\n")
            new_lines.append("    let mut v: Vec<u32> = Vec::new();\n")
            new_lines.append("    v.push(0);\n")
            new_lines.append("    let mut i:u32 = 1;\n")
            
            # Generate `if` statements based on int_param
            for i in range(int_param):
                new_lines.append(f"    if(i < {int_param}) {{\n")
                new_lines.append(f"        v.push(i);\n")
                new_lines.append(f"        i = i + 1;\n")
                new_lines.append("    }\n")
            
            new_lines.append("    return v;\n")
            new_lines.append("}\n\n")
            function_started = True
            continue

        # If the function has started, skip old lines
        if function_started:
            continue

        # Keep any other lines outside the function
        new_lines.append(line)

    # Ensure the main function is retained and close the verus! brace
    new_lines.append("fn main()\n{\n}\n")
    new_lines.append("}\n")  # Closing brace for verus!

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

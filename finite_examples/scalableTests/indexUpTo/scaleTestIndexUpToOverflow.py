import argparse

def read_file(filename):
    with open(filename, 'r') as file:
        return file.readlines()

def write_file(filename, lines):
    with open(filename, 'w') as file:
        file.writelines(lines)

def transform_function(lines, int_param):
    new_lines = []
    functions = []

    # Generate functions from 1 to int_param
    for param in range(1, int_param + 1):
        functions.append(f"fn indexUpTo_finite_{param}() -> (f: Vec<u32>)\n")
        functions.append(f"    ensures f.len() == {param}, \n")
        
        if param == 1:
            functions.append(f"             f[0] == 0,\n")
        else:
            functions.append(f"             f[0] == 0,\n")
            functions.append(f"             f[{param}-1] != 0,\n")
        
        functions.append("{\n")
        if param == 1:
            functions.append("    let mut v: Vec<u32> = Vec::new();\n")
            functions.append("    v.push(0);\n")
        else:
            functions.append(f"    let mut v: Vec<u32> = indexUpTo_finite_{param - 1}();\n")

        functions.append("    let mut i:u32 = v.len() as u32;\n")
        functions.append(f"    if (i < {param}) {{\n")
        functions.append("        v.push(i);\n")
        functions.append("    }\n")
        
        functions.append("    return v;\n")
        functions.append("}\n\n")

    # Process the lines to replace the original indexUpTo function
    function_started = False
    for line in lines:
        if "fn indexUpTo(n:u32)" in line:
            # Replace the original function with the newly generated ones
            new_lines.extend(functions)
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

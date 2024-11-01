import argparse

def read_file(filename):
    with open(filename, 'r') as file:
        return file.readlines()

def write_file(filename, lines):
    with open(filename, 'w') as file:
        file.writelines(lines)

def transform_function(lines, int_param):
    new_lines = []
    
    for line in lines:
        # Replace 'x' with int_param in the line containing 'n < x'
        if "n < x" in line:
            new_lines.append(line.replace("x", str(int_param)))
            continue
        
        new_lines.append(line)

        # After finding the line 'let mut i:u32 = 1;', insert the new blocks
        if "let mut i:u32 = 1;" in line:
            new_lines.append(line)  # Keep this line
            # Insert the if block int_param times after this line
            for _ in range(int_param+1):
                new_lines.append("    if(i < n){\n")
                new_lines.append("        v.push(i);\n")
                new_lines.append("        i = i + 1;\n")
                new_lines.append("    }\n")
            continue
            
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

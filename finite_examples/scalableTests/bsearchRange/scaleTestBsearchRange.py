import argparse

def read_file(filename):
    with open(filename, 'r') as file:
        return file.readlines()

def write_file(filename, lines):
    with open(filename, 'w') as file:
        file.writelines(lines)

def transform_function(lines, int_param):
    new_lines = []
    insert_block = (
        "    if i1 != i2 {\n"
        "        let ix = i1 + (i2 - i1) / 2;\n"
        "        if v[ix] < k {\n"
        "            i1 = ix + 1;\n"
        "        } else {\n"
        "            i2 = ix;\n"
        "        }\n"
        "    }\n"
    )
    
    for line in lines:
        # Replace 'x' with int_param
        if 'v.len() <=  x' in line:
            line = line.replace('x', str(int_param))
        
        new_lines.append(line)
        
        # Insert the block after the line "let mut i2: usize = v.len() - 1;"
        if "let mut i2: usize = v.len() - 1;" in line:
            for _ in range(int_param + 1):
                new_lines.append(insert_block)

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

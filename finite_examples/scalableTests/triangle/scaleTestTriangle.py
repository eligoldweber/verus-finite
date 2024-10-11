import sys
import os

def calculate_triangle_number(n):
    """Calculate the nth triangle number."""
    return n * (n + 1) // 2  # Use integer division

def manipulate_triangle_file(input_file, output_file, int_param):
    """Read an input file, manipulate it, and write to an output file."""
    # Check if the input file exists
    if not os.path.isfile(input_file):
        print(f"Input file '{input_file}' does not exist.")
        sys.exit(1)

    # Check if int_param is non-negative
    if int_param < 0:
        print("int_param must be a non-negative integer.")
        sys.exit(1)

    with open(input_file, 'r') as file:
        lines = file.readlines()

    new_lines = []
    triangle_functions = []

    # Generate triangle functions based on int_param
    for i in range(int_param + 1):
        if i == 0:
            triangle_functions.append(f"""
spec fn triangleF0() -> nat
{{
    let n: nat = 0;
    if n == 0 {{
        0
    }} else {{
        n
    }}   
}}
""".strip())
        else:
            triangle_functions.append(f"""
spec fn triangleF{i}() -> nat
{{
    let n: nat = {i};
    if n == 0 {{
        0
    }} else {{
        n + triangleF{i - 1}()
    }}   
}}
""".strip())

    # Modify the main function
    for line in lines:
        # Comment out the existing assertion
        if "assert(triangle(n as nat)" in line:
            line = f"    // {line.strip()}\n"

        # Add new assertions in the correct order
        if "let n =" in line:
            # Create a list to hold the assertions
            assertions = []

            # Generate assertions for every 99th decrement
            for i in range(int_param, -1, -99):
                triangle_number_i = calculate_triangle_number(i)
                assertions.append(f"    assert(triangleF{i}() == {triangle_number_i});\n")

            # Insert the assertions into the line
            for assertion in reversed(assertions):
                line += assertion

        new_lines.append(line)

    # Insert triangle functions in the correct position
    for idx, line in enumerate(new_lines):
        if line.strip() == "verus! {" and idx + 1 < len(new_lines):
            new_lines.insert(idx + 1, "\n" + "\n".join(triangle_functions) + "\n")
            break

    # Write the modified content to the output file
    with open(output_file, 'w') as file:
        file.writelines(new_lines)

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: python manipulate_triangle.py input_file output_file int_param")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]
    int_param = int(sys.argv[3])

    manipulate_triangle_file(input_file, output_file, int_param)

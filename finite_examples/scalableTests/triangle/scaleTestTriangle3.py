import sys
import os

def generate_triangle_function(int_param):
    lines = [f"spec fn triangleTest() -> nat\n{{"]
    lines.append(f"    let n: nat = {int_param};")
    
    lines.append("    if n == 0 {")
    lines.append("        0nat")
    lines.append("    } else {")
    lines.append("        n + ")

    # Build the direct calculation for triangle number
    if int_param > 0:
        lines.append(f"            (n - 1) as nat+ ")
        for i in range(2, int_param + 1):
            lines.append(f"                (n - {i})as nat")
            if i < int_param:
                lines.append(" + ")
        lines.append("              +  0nat")
        for _ in range(2, int_param + 1):
            lines.append("                ")  # Close the additional parentheses

    lines.append("             as nat")
    lines.append("    }")
    lines.append("}")  # Close the function

    return "\n".join(lines)

# Example usage
# print(generate_triangle_function(1))



def manipulate_triangle_file(input_file, output_file, int_param):
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

    # Generate the triangleTest function
    triangle_function = generate_triangle_function(int_param)

    # Modify the main function
    for line in lines:
        # Comment out the existing assertion
        if "assert(triangle(n as nat)" in line:
            line = f"    // {line.strip()}\n"
        
        # Add new assertion for triangleTest
        if "let n =" in line:
            line += f"    assert(triangleTest() == {int_param * (int_param + 1) // 2});\n"

        new_lines.append(line)

    # Insert the triangleTest function
    for idx, line in enumerate(new_lines):
        if line.strip() == "verus! {" and idx + 1 < len(new_lines):
            new_lines.insert(idx + 1, "\n" + triangle_function + "\n")
            break

    # Write to the output file
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


    # spec fn triangleTest() -> nat
    # {
    #     let n: nat = 3;
    #     if n == 0 {
    #         0nat 
    #     }else{
    #         n + (if (n-1) == 0 {0}else{n - 1 + (if (n-2) == 0 {0}else{n-2 + (if (n-3) == 0 {0nat}else{0nat}) as nat}) as nat}) as nat}
    # }
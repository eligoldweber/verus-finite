import argparse
import re

def read_file(filename):
    try:
        with open(filename, 'r') as file:
            lines = file.readlines()  # Read all lines into a list
            return [line.strip() for line in lines]  # Strip newline characters
    except FileNotFoundError:
        print(f"Error: The file '{filename}' was not found.")
        return None
    except Exception as e:
        print(f"An error occurred: {e}")
        return None

def write_file(filename, lines):
    try:
        with open(filename, 'w') as file:
            for line in lines:
                file.write(line + '\n')  # Write each line followed by a newline
        print(f"Content successfully written to '{filename}'.")
    except Exception as e:
        print(f"An error occurred while writing to the file: {e}")

def is_prime(n):
    """Check if a number is prime."""
    if n <= 1:
        return False
    for i in range(2, int(n**0.5) + 1):
        if n % i == 0:
            return False
    return True

def manipulate_content(lines, int_param):
    new_lines = []
    inside_is_prime = False

    for line in lines:
        stripped_line = line.strip()

        # Detect the start of the `is_prime` function definition
        if stripped_line.startswith("spec fn is_prime(candidate: nat) -> bool {"):
            inside_is_prime = True
            new_lines.append(line)  # Add the line as is
            continue

        # Modify the content inside the is_prime function
        if inside_is_prime:
            # Add &&& candidate < int_param after &&& 1 < candidate
            if stripped_line == "&&& 1 < candidate":
                new_lines.append(line)  # Original line
                new_lines.append(f"    &&& candidate <= {int_param}")  # Add new condition
                continue

            # Replace forall with explicit factor lines, regardless of indentation
            if "forall|factor: nat|" in stripped_line:
                for factor in range(2, int_param):
                    new_lines.append(f"    &&& 1 < {factor} < candidate ==> !divides({factor}, candidate)")
                continue  # Skip the forall line

            # Detect the end of the function block
            if stripped_line == "}":
                inside_is_prime = False

        new_lines.append(line)  # Add unchanged lines

    return new_lines
   

def update_main(lines, int_param):
    new_lines = []
    for line in lines:
        stripped_line = line.strip()

        # Look for commented-out assert statements for is_prime
        if stripped_line == "// assert(!is_prime());" or stripped_line == "// assert(is_prime());":
            # Check if int_param is prime and modify the assert accordingly
            if is_prime(int_param):
                # Replace with an uncommented assert with int_param as argument
                line = line.replace("// assert(is_prime());", f"assert(is_prime({int_param}));")
            else:
                line = line.replace("// assert(!is_prime());", f"assert(!is_prime({int_param}));")

        new_lines.append(line)

    return new_lines


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Read a file line by line, manipulate its contents, and write to a new file.")
    parser.add_argument("input_file", help="The name of the file to read")
    parser.add_argument("output_file", help="The name of the file to write the manipulated content to")
    parser.add_argument("int_param", type=int, help="The integer parameter to replace 'candidate'")

    args = parser.parse_args()

    lines = read_file(args.input_file)
    if lines is not None:
        manipulated_lines = manipulate_content(lines, args.int_param)
        updated_lines = update_main(manipulated_lines, args.int_param)
        write_file(args.output_file, updated_lines)

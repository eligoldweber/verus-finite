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
    is_in_is_prime = False  # Track whether we are within the is_prime function

    for line in lines:
        # Handle the function signature of is_prime
        if line.startswith("spec fn is_prime"):
            line = re.sub(r'candidate:\s*nat', '', line)
            line = re.sub(r'-> bool\s*{', '-> bool {', line)  # Adjust spacing if necessary
            new_lines.append(line)  # Add the modified signature
            is_in_is_prime = True  # We are now in the is_prime function
            continue

        # Process lines that belong to the body of is_prime
        if is_in_is_prime:
            # Check for the 'forall' expression
            if 'forall|factor: nat|' in line:
                line = re.sub(r'forall\|factor: nat\| 1 < factor < candidate ==> ', '', line)

                # Generate the new conditions
                conditions = [f"&&& !divides({i}, {int_param})\n" for i in range(2, int_param)]
                conjunctions_expr = ''.join(conditions)
                line = conjunctions_expr

            # Replace instances of 'candidate' in the body of is_prime only
            line = line.replace('candidate', str(int_param))

            # If we encounter the end of the function or any closing brace, we can exit the function
            if line.strip() == '}':
                is_in_is_prime = False

        new_lines.append(line)  # Add the processed line

    return new_lines

def update_main(lines, int_param):
    new_lines = []
    for line in lines:
        if line.strip() == "// assert(!is_prime());" or line.strip() == "// assert(is_prime());":
            # Check if int_param is prime
            if is_prime(int_param):
                line = line.replace("// assert(is_prime());", "assert(is_prime());")
            else:
                line = line.replace("// assert(!is_prime());", "assert(!is_prime());")
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

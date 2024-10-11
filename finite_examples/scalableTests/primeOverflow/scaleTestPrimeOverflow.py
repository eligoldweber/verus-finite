import argparse
import re

def read_file(filename):
    try:
        with open(filename, 'r') as file:
            lines = file.readlines()
            return [line.strip() for line in lines]
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
                file.write(line + '\n')
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
    is_in_is_prime = False

    for line in lines:
        if line.startswith("spec fn is_prime"):
            line = re.sub(r'candidate:\s*nat', '', line)
            line = re.sub(r'-> bool\s*{', '-> bool {', line)
            new_lines.append(line)
            is_in_is_prime = True
            continue

        if is_in_is_prime:
            if int_param < 1600:
                if 'forall|factor: nat|' in line:
                    line = re.sub(r'forall\|factor: nat\| 1 < factor < candidate ==> ', '', line)
                    conditions = [f"&&& !divides({i}, {int_param})\n" for i in range(2, int_param)]
                    conjunctions_expr = ''.join(conditions)
                    line = conjunctions_expr
            else:
                if 'forall|factor: nat|' in line:
                    line = re.sub(r'forall\|factor: nat\| 1 < factor < candidate ==> ', '', line)
                    conditions = [f"&&& !divides({i}, {int_param})" for i in range(2, 1600)]
                    conjunctions_expr = ''.join(conditions)
                    line = conjunctions_expr

            line = line.replace('candidate', str(int_param))

            if line.strip() == '}':
                is_in_is_prime = False

        new_lines.append(line)

    return new_lines

def is_prime_overflow(int_param):
    """Generate multiple is_prime_overflow functions based on int_param."""
    overflow_functions = []
    index = 0

    for lower in range(1600, int_param + 1, 1600):
        upper = min(lower + 1600, int_param)
        conditions = [f"&&& !divides({i}, {int_param})" for i in range(lower, upper)]
        overflow_function = [f"spec fn is_prime_overflow_{index}() -> bool {{"] + conditions + ["}"]
        overflow_functions.append(overflow_function)
        index += 1

    return overflow_functions

def update_main(lines, int_param, overflow_count):
    new_lines = []
    is_prime_check = is_prime(int_param)

    for line in lines:
        if line.strip() == "// assert(!is_prime());":
            if int_param < 1600:
                line = line.replace("// assert(!is_prime());", "assert(is_prime());" if is_prime_check else "assert(!is_prime());")
            elif int_param >= 1600:
                if is_prime_check:
                    assertion = "assert(is_prime() && " + " && ".join([f"is_prime_overflow_{i}()" for i in range(overflow_count)]) + ");"
                else:
                    assertion = "assert(!(is_prime() && " + " && ".join([f"is_prime_overflow_{i}()" for i in range(overflow_count)]) + "));"
                line = line.replace("// assert(!is_prime());", assertion)
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
        
        # Add is_prime_overflow functions for int_param > 1600
        overflow_functions = []
        if args.int_param > 1600:
            overflow_functions = is_prime_overflow(args.int_param)
            index = next((i for i, line in enumerate(manipulated_lines) if line.startswith("fn main()")), len(manipulated_lines))
            manipulated_lines = manipulated_lines[:index] + [line for function in overflow_functions for line in function] + manipulated_lines[index:]

        updated_lines = update_main(manipulated_lines, args.int_param, len(overflow_functions))
        write_file(args.output_file, updated_lines)

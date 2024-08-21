import subprocess

def calculate_factorial(number):
    try:
        result = subprocess.run(
            ["./target/release/factorial_cli", str(number)], 
            check=True, 
            capture_output=True, 
            text=True
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return f"An error occurred: {e}"

if __name__ == "__main__":
    number = 10  # Example number
    factorial_result = calculate_factorial(number)
    print(f"{factorial_result}")


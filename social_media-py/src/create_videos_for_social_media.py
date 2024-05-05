import subprocess

def run_program():
    """Runs the cargo run command and handles potential errors."""
    print("Running program...")
    try:
        result = subprocess.run(["cargo", "run","--release"], check=True)
        result.check_returncode()  # Raise exception if return code is not 0 (success)
        print("Video Creation and Uploading was successful!")
    except subprocess.CalledProcessError as error:
        print("Error during program run:")
        exit(1)  # Exit program with non-zero code indicating failure


def main() :
    run_program()

if __name__ == "__main__":
    main()

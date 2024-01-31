from reddit_motion import RedditMotionCommandLineParser;

def main():
    arguments = RedditMotionCommandLineParser()
    arguments.parse()

    if arguments.is_infinite_duration():
        print(f"Processing all files in directory: {arguments.infinite_file_path()}")

    if arguments.is_limited_duration():
        print(f"Processing limited files in directories: {arguments.limited_file_paths()}")

if __name__ == '__main__':
    main()
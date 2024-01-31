import argparse

class RedditMotionCommandLineParser:
    """
    Parses command-line arguments for processing Reddit Motion data.

    This class provides methods for extracting the specified file directories and determining whether to process the directories differently, based on the duration.
    """

    def __init__(self):
        """
        Initializes the `RedditMotionCommandLineParser` object.
        """
        self.__parser = argparse.ArgumentParser(description='Process file directories')
        infinite_group = self.__parser.add_mutually_exclusive_group()
        infinite_group.add_argument(
            '--infinite',
            dest='infinite_directory',
            metavar='FILE_DIR',
        )

        limited_group = self.__parser.add_mutually_exclusive_group()
        limited_group.add_argument(
            '--limited',
            dest='limited_directories',
            metavar='FILE_DIR',
            nargs='+',
        )

    def parse(self):
        """
        Parses command-line arguments and stores them in the `__arguments` attribute.
        """
        self.__arguments = self.__parser.parse_args()
        del self.__parser

    def is_infinite_duration(self) -> bool:
        """
        Returns `True` if the parser received --infinite flag
        """
        return self.__arguments.infinite_directory is not None

    def is_limited_duration(self) -> bool:
        """
        Returns `True` if the parser is received --limited flag
        """
        return self.__arguments.limited_directories is not None

    def infinite_file_path(self) -> str:
        """
        Returns the path to the infinite file directory if `is_infinite_duration()` is `True`, raises an `AttributeError` otherwise.
        """
        if not self.is_infinite_duration():
            raise AttributeError('The parser is not configured to process an infinite stream of data.')
        return self.__arguments.infinite_directory

    def limited_file_paths(self) -> [str]:
        """
        Returns a list of file paths if `is_limited_duration()` is `True`, raises an `AttributeError` otherwise.
        """
        if not self.is_limited_duration():
            raise AttributeError('The parser is not configured to process a limited set of files.')
        
        return self.__arguments.limited_directories

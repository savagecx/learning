import os


def count_dirs_and_files(directory="."):
    """Count the amount of of directories and files in passed in "directory" arg.
    Return a tuple of (number_of_directories, number_of_files)
    """
    f = 0
    d = 0
    for _, dirs, files in os.walk(directory):
        f += len(files)
        d += len(dirs)

    return (d, f)

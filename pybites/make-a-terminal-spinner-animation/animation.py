from itertools import cycle
from time import sleep, time

SPINNER_STATES = ["-", "\\", "|", "/"]  # had to escape \
STATE_TRANSITION_TIME = 0.1


def spinner(seconds: int):
    """Make a terminal loader/spinner animation using the imports above.
    Takes seconds argument = time for the spinner to run.
    Does not return anything, only prints to stdout."""
    animation = cycle(SPINNER_STATES)
    complete = time() + seconds
    while time() < complete:
        print(next(animation), flush=True, end="\r")
        sleep(STATE_TRANSITION_TIME)


if __name__ == "__main__":
    spinner(2)

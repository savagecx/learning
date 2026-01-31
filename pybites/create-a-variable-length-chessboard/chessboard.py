WHITE, BLACK = " ", "#"


def create_chessboard(size=8):
    """Create a chessboard with of the size passed in.
    Don't return anything, print the output to stdout"""
    half = int(size / 2)
    for _ in range(half):
        print((WHITE + BLACK) * half)
        print((BLACK + WHITE) * half)

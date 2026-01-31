def countdown():
    """Write a generator that counts from 100 to 1"""
    return (num for num in range(100, 0, -1))

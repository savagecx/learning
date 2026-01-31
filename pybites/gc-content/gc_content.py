from collections import Counter


def calculate_gc_content(sequence: str):
    """
    Receives a DNA sequence (A, G, C, or T)
    Returns the percentage of GC content (rounded to the last two digits)
    """
    seq = [base.upper() for base in sequence if base.upper() in "ACGT"]
    count = Counter(seq)
    return round(((count.get("G", 0) + count.get("C", 0)) / len(seq)) * 100, 2)

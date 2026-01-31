def rgb_to_hex(rgb: tuple[int, int, int]) -> str:
    """Receives (r, g, b)  tuple, checks if each rgb int is within RGB
    boundaries (0, 255) and returns its converted hex, for example:
    Silver: input tuple = (192,192,192) -> output hex str = #C0C0C0"""
    for ch in rgb:
        if ch < 0 or ch > 255:
            raise ValueError("RGB channels must be a value between 0 and 255")

    return f"#{rgb[0]:02X}{rgb[1]:02X}{rgb[2]:02X}"

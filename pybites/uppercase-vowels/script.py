VOWELS = "aeiou"
EXTENSIONS = [".mp3", ".jpg", ".jpeg", ".pdf", ".txt", ".mp4", ".png", ".exe"]


def uppercase_vowels(text: str) -> str:
    result = ""
    extension = next((ext for ext in EXTENSIONS if text.endswith(ext)), "")

    for char in text.lower().rstrip(extension):
        if char in VOWELS:
            result += char.upper()
        elif char.isalpha():
            result += char
        else:
            result += " "

    return f"{result}{extension}"


if __name__ == "__main__":
    print(uppercase_vowels("abcDeFGHIJ"))
    print(uppercase_vowels("..-"))
    print(uppercase_vowels("file monk.exe"))

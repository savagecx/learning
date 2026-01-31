def sort_words_case_insensitively(words: list):
    """Sort the provided word list ignoring case, and numbers last
    (1995, 19ab = numbers / Happy, happy4you = strings, hence for
     numbers you only need to check the first char of the word)
    """

    words.sort(key=str.lower)
    for i, word in enumerate(words):
        if not word[0].isnumeric():
            break

    return words[i:] + words[:i]

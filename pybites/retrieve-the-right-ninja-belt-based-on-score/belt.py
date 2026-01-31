from typing import LiteralString

scores = [10, 50, 100, 175, 250, 400, 600, 800, 1000]
belts = "white yellow orange green blue brown black paneled red".split()


def get_belt(user_score: int, scores: list[int] = scores, belts: list[LiteralString] = belts) -> str | None:
    user_belt = None
    for score, belt in zip(scores, belts):
        if user_score >= score:
            user_belt = belt
        else:
            break

    return user_belt

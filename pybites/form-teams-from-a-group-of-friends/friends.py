from itertools import combinations, permutations


def friends_teams(friends: list, team_size: int = 2, order_does_matter: bool = False):
    if order_does_matter:
        return permutations(friends, team_size)

    return combinations(friends, team_size)

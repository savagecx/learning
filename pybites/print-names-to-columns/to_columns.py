def print_names_to_columns(names: list[str], cols: int = 2) -> None:
    for i, name in enumerate(names, start=1):
        end = "" if i % cols else "\n"
        print(f"| {name:<10}", end=end)

def generate_affiliation_link(url: str):
    start = url.index("/dp/")
    id = url[start:].split("/")[2]
    return f"http://www.amazon.com/dp/{id}/?tag=pyb0f-20"

from fastapi import FastAPI

app = FastAPI()


@app.get("/")
async def read_root():
    return {"message": "Welcome to PyBites' FastAPI Learning Path 🐍 🎉"}

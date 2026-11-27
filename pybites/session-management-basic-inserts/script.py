from sqlmodel import Field, Session, SQLModel, create_engine


class Workout(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    name: str


class Exercise(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    name: str


sqlite_url = "sqlite:///:memory:"
engine = create_engine(sqlite_url, echo=False)


def create_tables() -> None:
    SQLModel.metadata.create_all(engine)


def add_workout(name: str) -> Workout:
    workout = Workout(name=name)
    with Session(engine) as session:
        session.add(workout)
        session.commit()
        session.refresh(workout)

    return workout


def add_exercise(name: str) -> Exercise:
    exercise = Exercise(name=name)
    with Session(engine) as session:
        session.add(exercise)
        session.commit()
        session.refresh(exercise)

    return exercise

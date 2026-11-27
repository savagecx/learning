from sqlmodel import Field, Session, SQLModel, create_engine, select


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


def get_workout(workout_id: int) -> Workout | None:
    with Session(engine) as session:
        return session.get(Workout, workout_id)


def list_workouts() -> list[Workout]:
    with Session(engine) as session:
        statement = select(Workout).order_by(Workout.id)
        result = session.exec(statement)
        return list(result.all())


def find_exercise_by_name(name: str) -> Exercise | None:
    with Session(engine) as session:
        statement = select(Exercise).where(Exercise.name == name)
        result = session.exec(statement)
        return result.first()


def list_exercises() -> list[Exercise]:
    with Session(engine) as session:
        statement = select(Exercise).order_by(Exercise.name)
        result = session.exec(statement)
        return list(result.all())

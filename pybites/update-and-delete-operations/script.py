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


def update_workout(workout_id: int, new_name: str) -> Workout | None:
    with Session(engine) as session:
        if workout := session.get(Workout, workout_id):
            workout.name = new_name
            session.commit()
            session.refresh(workout)

        return workout


def delete_workout(workout_id: int) -> bool:
    with Session(engine) as session:
        if workout := session.get(Workout, workout_id):
            session.delete(workout)
            session.commit()
            return True

        return False


def update_exercise(exercise_id: int, new_name: str) -> Exercise | None:
    with Session(engine) as session:
        if exercise := session.get(Exercise, exercise_id):
            exercise.name = new_name
            session.commit()
            session.refresh(exercise)

        return exercise


def delete_exercise(exercise_id: int) -> bool:
    with Session(engine) as session:
        if exercise := session.get(Exercise, exercise_id):
            session.delete(exercise)
            session.commit()
            return True

        return False

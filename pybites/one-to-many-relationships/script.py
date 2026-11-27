from datetime import date

from sqlmodel import Field, Relationship, Session, SQLModel, create_engine, select


class Workout(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    name: str
    log_entries: list["LogEntry"] = Relationship(back_populates="workout")


class LogEntry(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    workout_id: int | None = Field(foreign_key="workout.id")
    set_number: int
    weight: int
    reps: int
    date_recorded: date = Field(default_factory=date.today)
    workout: Workout | None = Relationship(back_populates="log_entries")


sqlite_url = "sqlite:///:memory:"
engine = create_engine(sqlite_url, echo=False)


def create_tables() -> None:
    SQLModel.metadata.create_all(engine)


def add_log_entry(workout_id: int, set_number: int, weight: int, reps: int) -> LogEntry:
    """Create a log entry for a workout. Raise ValueError if workout doesn't exist."""
    with Session(engine) as session:
        if not session.get(Workout, workout_id):
            raise ValueError(f"Workout with id {workout_id} does not exist")

        log = LogEntry(workout_id=workout_id, set_number=set_number, weight=weight, reps=reps)
        session.add(log)
        session.commit()
        session.refresh(log)
        return log


def get_log_entries(workout_id: int) -> list[LogEntry]:
    """Get all log entries for a workout, ordered by set_number."""
    with Session(engine) as session:
        statement = select(LogEntry).where(LogEntry.workout_id == workout_id).order_by(LogEntry.set_number)
        result = session.exec(statement)
        return list(result.all())

import pytest
from script import (
    Exercise,
    Workout,
    add_exercise,
    add_workout,
    create_tables,
    engine,
)
from sqlmodel import Session, select


@pytest.fixture(autouse=True)
def setup_database():
    """Create tables before each test."""
    create_tables()


def test_add_workout_returns_workout_with_id():
    workout = add_workout("Upper Body Day")

    assert isinstance(workout, Workout)
    assert workout.id is not None, "Workout id should be populated after insert"
    assert workout.name == "Upper Body Day"


def test_add_workout_saves_to_database():
    workout = add_workout("Leg Day")

    # Query the database to confirm it was saved
    with Session(engine) as session:
        statement = select(Workout).where(Workout.id == workout.id)
        result = session.exec(statement).first()

        assert result is not None, "Workout should exist in database"
        assert result.name == "Leg Day"


def test_add_multiple_workouts():
    workout1 = add_workout("Push Day")
    workout2 = add_workout("Pull Day")

    assert workout1.id != workout2.id, "Each workout should have unique id"
    assert workout1.id is not None
    assert workout2.id is not None


def test_add_exercise_returns_exercise_with_id():
    exercise = add_exercise("Bench Press")

    assert isinstance(exercise, Exercise)
    assert exercise.id is not None, "Exercise id should be populated after insert"
    assert exercise.name == "Bench Press"


def test_add_exercise_saves_to_database():
    exercise = add_exercise("Squats")

    # Query the database to confirm it was saved
    with Session(engine) as session:
        statement = select(Exercise).where(Exercise.id == exercise.id)
        result = session.exec(statement).first()

        assert result is not None, "Exercise should exist in database"
        assert result.name == "Squats"


def test_add_multiple_exercises():
    exercise1 = add_exercise("Deadlift")
    exercise2 = add_exercise("Pull Ups")

    assert exercise1.id != exercise2.id, "Each exercise should have unique id"
    assert exercise1.id is not None
    assert exercise2.id is not None


def test_ids_are_auto_incremented():
    workout1 = add_workout("Workout 1")
    workout2 = add_workout("Workout 2")

    assert workout2.id == workout1.id + 1, "IDs should auto-increment"
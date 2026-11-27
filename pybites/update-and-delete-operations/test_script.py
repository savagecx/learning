import pytest
from script import (
    Exercise,
    Workout,
    create_tables,
    delete_exercise,
    delete_workout,
    engine,
    update_exercise,
    update_workout,
)
from sqlmodel import Session


@pytest.fixture(autouse=True)
def setup_database():
    """Create tables before each test."""
    create_tables()


@pytest.fixture
def sample_workout():
    """Create a test workout."""
    workout = Workout(name="Upper Body Day")
    with Session(engine) as session:
        session.add(workout)
        session.commit()
        session.refresh(workout)
    return workout


@pytest.fixture
def sample_exercise():
    """Create a test exercise."""
    exercise = Exercise(name="Bench Press")
    with Session(engine) as session:
        session.add(exercise)
        session.commit()
        session.refresh(exercise)
    return exercise


def test_update_workout_changes_name(sample_workout):
    updated = update_workout(sample_workout.id, "Leg Day")

    assert updated is not None
    assert updated.id == sample_workout.id
    assert updated.name == "Leg Day"


def test_update_workout_persists_to_database(sample_workout):
    update_workout(sample_workout.id, "Core Workout")

    # Query database to verify
    with Session(engine) as session:
        workout = session.get(Workout, sample_workout.id)
        assert workout.name == "Core Workout"


def test_update_workout_returns_none_for_invalid_id():
    result = update_workout(999, "New Name")
    assert result is None


def test_delete_workout_removes_from_database(sample_workout):
    result = delete_workout(sample_workout.id)

    assert result is True

    # Verify it's gone from database
    with Session(engine) as session:
        workout = session.get(Workout, sample_workout.id)
        assert workout is None


def test_delete_workout_returns_false_for_invalid_id():
    result = delete_workout(999)
    assert result is False


def test_update_exercise_changes_name(sample_exercise):
    updated = update_exercise(sample_exercise.id, "Squats")

    assert updated is not None
    assert updated.id == sample_exercise.id
    assert updated.name == "Squats"


def test_update_exercise_persists_to_database(sample_exercise):
    update_exercise(sample_exercise.id, "Deadlift")

    # Query database to verify
    with Session(engine) as session:
        exercise = session.get(Exercise, sample_exercise.id)
        assert exercise.name == "Deadlift"


def test_update_exercise_returns_none_for_invalid_id():
    result = update_exercise(999, "New Exercise")
    assert result is None


def test_delete_exercise_removes_from_database(sample_exercise):
    result = delete_exercise(sample_exercise.id)

    assert result is True

    # Verify it's gone from database
    with Session(engine) as session:
        exercise = session.get(Exercise, sample_exercise.id)
        assert exercise is None


def test_delete_exercise_returns_false_for_invalid_id():
    result = delete_exercise(999)
    assert result is False


def test_multiple_updates_on_same_workout(sample_workout):
    update_workout(sample_workout.id, "First Update")
    updated = update_workout(sample_workout.id, "Second Update")

    assert updated.name == "Second Update"

    with Session(engine) as session:
        workout = session.get(Workout, sample_workout.id)
        assert workout.name == "Second Update"
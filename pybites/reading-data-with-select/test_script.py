import pytest
from script import (
    Exercise,
    Workout,
    create_tables,
    engine,
    find_exercise_by_name,
    get_workout,
    list_exercises,
    list_workouts,
)
from sqlmodel import Session, delete


@pytest.fixture(autouse=True)
def setup_database():
    """Create tables before each test."""
    create_tables()
    yield
    # Clear all data after each test
    with Session(engine) as session:
        session.exec(delete(Workout))
        session.exec(delete(Exercise))
        session.commit()


@pytest.fixture
def seed_workouts():
    """Add some test workouts to the database."""
    workouts_data = [
        Workout(name="Upper Body Day"),
        Workout(name="Leg Day"),
        Workout(name="Core Workout"),
    ]
    with Session(engine) as session:
        for workout in workouts_data:
            session.add(workout)
        session.commit()
        for workout in workouts_data:
            session.refresh(workout)
    return workouts_data


@pytest.fixture
def seed_exercises():
    """Add some test exercises to the database."""
    exercises_data = [
        Exercise(name="Squats"),
        Exercise(name="Bench Press"),
        Exercise(name="Deadlift"),
        Exercise(name="Pull Ups"),
    ]
    with Session(engine) as session:
        for exercise in exercises_data:
            session.add(exercise)
        session.commit()
        for exercise in exercises_data:
            session.refresh(exercise)
    return exercises_data


def test_get_workout_by_id(seed_workouts):
    workout = seed_workouts[0]
    result = get_workout(workout.id)

    assert result is not None
    assert result.id == workout.id
    assert result.name == workout.name


def test_get_workout_returns_none_for_invalid_id():
    result = get_workout(999)
    assert result is None


def test_list_workouts_returns_all_workouts(seed_workouts):
    workouts = list_workouts()

    assert len(workouts) == 3
    assert all(isinstance(w, Workout) for w in workouts)


def test_list_workouts_ordered_by_id(seed_workouts):
    workouts = list_workouts()

    ids = [w.id for w in workouts]
    assert ids == sorted(ids), "Workouts should be ordered by id"


def test_list_workouts_empty_database():
    workouts = list_workouts()
    assert workouts == []
    assert isinstance(workouts, list)


def test_find_exercise_by_name(seed_exercises):
    exercise = find_exercise_by_name("Squats")

    assert exercise is not None
    assert exercise.name == "Squats"
    assert isinstance(exercise, Exercise)


def test_find_exercise_by_name_case_sensitive(seed_exercises):
    exercise = find_exercise_by_name("squats")  # lowercase
    assert exercise is None, "Search should be case-sensitive"


def test_find_exercise_returns_none_for_nonexistent():
    exercise = find_exercise_by_name("Burpees")
    assert exercise is None


def test_list_exercises_returns_all(seed_exercises):
    exercises = list_exercises()

    assert len(exercises) == 4
    assert all(isinstance(e, Exercise) for e in exercises)


def test_list_exercises_ordered_alphabetically(seed_exercises):
    exercises = list_exercises()
    names = [e.name for e in exercises]

    expected = ["Bench Press", "Deadlift", "Pull Ups", "Squats"]
    assert names == expected, "Exercises should be ordered alphabetically"


def test_list_exercises_empty_database():
    exercises = list_exercises()
    assert exercises == []
    assert isinstance(exercises, list)
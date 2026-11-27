from datetime import date

import pytest
from script import (
    LogEntry,
    Workout,
    add_log_entry,
    create_tables,
    engine,
    get_log_entries,
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


def test_log_entry_has_foreign_key():
    columns = {col.name: col for col in LogEntry.__table__.columns}
    assert "workout_id" in columns
    assert len(columns["workout_id"].foreign_keys) > 0


def test_workout_has_log_entries_relationship():
    workout = Workout(name="Test")
    assert hasattr(workout, "log_entries")


def test_log_entry_has_workout_relationship():
    log = LogEntry(set_number=1, weight=100, reps=10)
    assert hasattr(log, "workout")


def test_add_log_entry_creates_entry(sample_workout):
    log_entry = add_log_entry(
        workout_id=sample_workout.id, set_number=1, weight=100, reps=10
    )

    assert log_entry.id is not None
    assert log_entry.workout_id == sample_workout.id
    assert log_entry.set_number == 1
    assert log_entry.weight == 100
    assert log_entry.reps == 10


def test_add_log_entry_sets_date_default(sample_workout):
    log_entry = add_log_entry(
        workout_id=sample_workout.id, set_number=1, weight=100, reps=10
    )

    assert log_entry.date_recorded == date.today()


def test_add_log_entry_raises_for_invalid_workout():
    with pytest.raises(ValueError, match="does not exist"):
        add_log_entry(workout_id=999, set_number=1, weight=100, reps=10)


def test_get_log_entries_returns_entries(sample_workout):
    add_log_entry(sample_workout.id, set_number=1, weight=100, reps=10)
    add_log_entry(sample_workout.id, set_number=2, weight=110, reps=8)
    add_log_entry(sample_workout.id, set_number=3, weight=120, reps=6)

    entries = get_log_entries(sample_workout.id)

    assert len(entries) == 3
    assert all(isinstance(e, LogEntry) for e in entries)


def test_get_log_entries_ordered_by_set_number(sample_workout):
    add_log_entry(sample_workout.id, set_number=3, weight=120, reps=6)
    add_log_entry(sample_workout.id, set_number=1, weight=100, reps=10)
    add_log_entry(sample_workout.id, set_number=2, weight=110, reps=8)

    entries = get_log_entries(sample_workout.id)
    set_numbers = [e.set_number for e in entries]

    assert set_numbers == [1, 2, 3]


def test_get_log_entries_filters_by_workout(sample_workout):
    # Create another workout
    other_workout = Workout(name="Leg Day")
    with Session(engine) as session:
        session.add(other_workout)
        session.commit()
        session.refresh(other_workout)

    # Add entries to both workouts
    add_log_entry(sample_workout.id, set_number=1, weight=100, reps=10)
    add_log_entry(other_workout.id, set_number=1, weight=200, reps=5)

    entries = get_log_entries(sample_workout.id)

    assert len(entries) == 1
    assert entries[0].workout_id == sample_workout.id


def test_get_log_entries_empty_for_no_entries(sample_workout):
    entries = get_log_entries(sample_workout.id)
    assert entries == []


def test_bidirectional_relationship(sample_workout):
    log_entry = add_log_entry(sample_workout.id, 1, 100, 10)

    # Access from workout to log entries
    with Session(engine) as session:
        workout = session.get(Workout, sample_workout.id)
        assert len(workout.log_entries) > 0

        # Access from log entry to workout
        log = session.get(LogEntry, log_entry.id)
        assert log.workout is not None
        assert log.workout.id == sample_workout.id
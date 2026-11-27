from script import Exercise, Workout, create_tables, engine
from sqlalchemy import inspect


def test_workout_is_a_table():
    assert hasattr(Workout, "__table__"), "Workout must have table=True"


def test_exercise_is_a_table():
    assert hasattr(Exercise, "__table__"), "Exercise must have table=True"


def test_workout_table_name():
    assert Workout.__tablename__ == "workout"


def test_exercise_table_name():
    assert Exercise.__tablename__ == "exercise"


def test_workout_has_id_field():
    columns = {col.name: col for col in Workout.__table__.columns}
    assert "id" in columns, "Workout must have 'id' field"
    assert columns["id"].primary_key, "id must be a primary key"


def test_workout_has_name_field():
    columns = {col.name: col for col in Workout.__table__.columns}
    assert "name" in columns, "Workout must have 'name' field"


def test_exercise_has_id_field():
    columns = {col.name: col for col in Exercise.__table__.columns}
    assert "id" in columns, "Exercise must have 'id' field"
    assert columns["id"].primary_key, "id must be a primary key"


def test_exercise_has_name_field():
    columns = {col.name: col for col in Exercise.__table__.columns}
    assert "name" in columns, "Exercise must have 'name' field"


def test_create_tables_creates_tables():
    create_tables()

    inspector = inspect(engine)
    table_names = inspector.get_table_names()

    assert "workout" in table_names, "workout table should exist after create_tables()"
    assert (
        "exercise" in table_names
    ), "exercise table should exist after create_tables()"


def test_engine_is_sqlite():
    assert "sqlite" in str(engine.url), "Engine should use SQLite"
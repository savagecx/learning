import pytest
from workouts import print_workout_days

pytestmark = pytest.mark.parametrize(
    "workout,expected",
    [("lower body #2", "fri"), ("cardio", "wed"), ("body", "mon,tue,thu,fri"), ("foo", "No matching workout")],
)


def test_print_workout_days(workout, expected):
    print_workout_days(workout)

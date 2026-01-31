from pydantic import BaseModel


class Food(BaseModel):
    id: int
    name: str
    serving_size: str
    kcal_per_serving: int = 0
    protein_grams: float = 0
    fibre_grams: float = 0

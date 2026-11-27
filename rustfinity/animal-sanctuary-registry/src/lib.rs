use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let animals = registry.entry(section.to_string()).or_default();

    if !animals.iter().any(|a| a == animal) {
        animals.push(animal.to_string());
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    match registry.get(section) {
        Some(animals) => {
            let mut animals = animals.clone();
            animals.sort();
            animals
        }
        None => Vec::new(),
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut animals = registry
        .values()
        .cloned()
        .flatten()
        .collect::<Vec<String>>();
    animals.sort();
    animals
}

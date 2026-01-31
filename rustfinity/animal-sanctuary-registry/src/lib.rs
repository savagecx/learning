use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    registry
        .entry(section.to_string())
        .and_modify(|entry| {
            if !entry.contains(&animal.to_string()) {
                entry.push(animal.to_string())
            }
        })
        .or_insert(vec![animal.to_string()]);
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    match registry.get(section) {
        Some(values) => {
            let mut values = values.clone();
            values.sort();
            values
        }
        None => vec![],
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut all_animals: Vec<String> = registry.values().flatten().cloned().collect();
    all_animals.sort();
    all_animals
}

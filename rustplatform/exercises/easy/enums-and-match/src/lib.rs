enum TrafficLight {
    // define the variants here
}

fn action(light: TrafficLight) -> &'static str {
    // match on `light` and return the right string
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_light() {
        assert_eq!(action(TrafficLight::Red), "stop");
    }

    #[test]
    fn test_yellow_light() {
        assert_eq!(action(TrafficLight::Yellow), "slow down");
    }

    #[test]
    fn test_green_light() {
        assert_eq!(action(TrafficLight::Green), "go");
    }
}

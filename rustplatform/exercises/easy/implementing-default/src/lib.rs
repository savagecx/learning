#[derive(Default)]
struct Counter {
    value: i32,
    step: i32,
}

struct ServerConfig {
    host: String,
    port: u16,
    max_connections: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: String::from("localhost"),
            port: 8080,
            max_connections: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_default_value() {
        let counter = Counter::default();
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn test_counter_default_step() {
        let counter = Counter::default();
        assert_eq!(counter.step, 0);
    }

    #[test]
    fn test_server_config_default_host() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "localhost");
    }

    #[test]
    fn test_server_config_default_port() {
        let config = ServerConfig::default();
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_server_config_default_max_connections() {
        let config = ServerConfig::default();
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    fn test_server_config_struct_update() {
        let custom = ServerConfig {
            port: 3000,
            ..Default::default()
        };
        assert_eq!(custom.host, "localhost");
        assert_eq!(custom.port, 3000);
        assert_eq!(custom.max_connections, 100);
    }

    #[test]
    fn test_counter_struct_update() {
        let custom = Counter {
            value: 10,
            ..Default::default()
        };
        assert_eq!(custom.value, 10);
        assert_eq!(custom.step, 0);
    }
}

use std::fmt::{Display, Formatter};

struct SmartSocket {
    name: String,
    enabled: bool,
    current: f32,
}

impl Display for SmartSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SmartSocket {}", &self.name)
    }
}

impl SmartSocket {
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    fn get_current(&self) -> f32 {
        self.current
    }
    fn set_current(&mut self, current: f32) {
        self.current = current;
    }
}

struct SmartThermometer {
    temperature: f32,
}

impl SmartThermometer {
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
    fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }
}

fn main() {
    let mut smart_socket = SmartSocket {
        name: "qwe".to_string(),
        enabled: false,
        current: 0.0,
    };
    println!("{}", smart_socket);
    smart_socket.set_enabled(true);
    smart_socket.set_current(42f32);
    println!("{}", smart_socket.get_current());

    let mut smart_termometer = SmartThermometer { temperature: 0.0 };

    smart_termometer.set_temperature(23f32);
    println!("{}", smart_termometer.get_temperature());
}

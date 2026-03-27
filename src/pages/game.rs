use gloo::utils::window;

pub struct Game {
    pub secret: u32,
}

#[allow(clippy::new_without_default)]
impl Game {
    pub fn new() -> Self {
        let mut array = [0u8; 4];
        window()
            .crypto()
            .unwrap()
            .get_random_values_with_u8_array(&mut array)
            .unwrap();

        let secret = u32::from_be_bytes(array) % 100 + 1;
        Self { secret }
    }

    pub fn guess(&self, input: u32) -> &'static str {
        if input < self.secret {
            "Too small!"
        } else if input > self.secret {
            "Too big!"
        } else {
            "Correct!"
        }
    }
}

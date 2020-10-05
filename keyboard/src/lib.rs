pub struct Keyboard { 
    pub keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16]
        }
    }

    pub fn reset(&mut self) {
        self.keys = [false; 16];
    }

    pub fn press_key(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn unpress_key(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }

    pub fn key_is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

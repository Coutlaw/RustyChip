
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

	pub fn un_press_key(&mut self, key: u8) {
		self.keys[key as usize] = false;
	}

	pub fn key_is_pressed(&self, key: u8) -> bool {
		self.keys[key as usize]
	}

	// convert a keypress into a chip 8 key value if possible
	fn char_to_key(key: char) -> Option<u8> {
		if let Some(x) = key.to_digit(10) {
			if x <= 16 {
				return Some(x as u8)
			}
		}
		None
	}
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

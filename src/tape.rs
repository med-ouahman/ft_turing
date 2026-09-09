pub struct Tape {
	cells: Vec<char>,
	position: usize,
	blank: char,
}

impl Tape {
	pub fn new(input: &str, blank: char) -> Self {
		let mut cells: Vec<char> = input.chars().collect();
		if cells.is_empty() {
			cells.push(blank);
		}
		Self { cells, position: 0, blank }
	}

	pub fn read(&self) -> char {
		self.cells[self.position]
	}

	pub fn write(&mut self, symbol: char) {
		self.cells[self.position] = symbol;
	}

	pub fn move_left(&mut self) {
		if self.position == 0 {
			self.cells.insert(0, self.blank);
		} else {
			self.position -= 1;
		}
	}

	pub fn move_right(&mut self) {
		self.position += 1;
		if self.position == self.cells.len() {
			self.cells.push(self.blank);
		}
	}
}

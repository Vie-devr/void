const GAP_SIZE: usize = 10;
const GAP_FILLER: char = ' ';

pub struct GapBuffer {
	buffer: Vec<char>,
	gap_start: usize,
	gap_end: usize,
}

impl GapBuffer {
	pub fn new() -> Self {
		Self {
			buffer: vec![GAP_FILLER; GAP_SIZE],
			gap_start: 0,
			gap_end: GAP_SIZE,
		}
	}

	pub fn len(&self) -> usize {
		self.buffer.len() - (self.gap_end - self.gap_start)
	}

	pub fn to_vec(&self) -> Vec<char> {
		let mut vec = Vec::new();
		vec.extend(self.buffer[..self.gap_start].iter());
		if self.gap_end < self.buffer.len() {
			vec.extend(self.buffer[self.gap_end..].iter());
		}

		vec
	}

	pub fn insert(&mut self, str: &str, index: usize) {
		self.move_gap(index);

		for chr in str.chars() {
			if self.gap_start == self.gap_end {
				self.grow_gap(GAP_SIZE);
			}

			self.buffer[self.gap_start] = chr;
			self.gap_start += 1;
		}
	}

	pub fn insert_char(&mut self, chr: char, index: usize) {
		self.insert(&chr.to_string(), index);
	}

	pub fn delete_char(&mut self, index: usize) {
		self.move_gap(index + 1);
		self.gap_start -= 1;
		self.buffer[self.gap_start] = GAP_FILLER;
	}

	fn move_gap(&mut self, index: usize) {
		match index {
			i if i > self.gap_start => {
				let amount = i - self.gap_start;

				self.buffer.copy_within(
					self.gap_end..(self.gap_end + amount),
					self.gap_start,
				);

				self.gap_start += amount;
				self.gap_end += amount;
			}
			i if i < self.gap_start => {
				let amount = self.gap_start - i;

				self.buffer.copy_within(
					i..self.gap_start,
					self.gap_end - amount,
				);

				self.gap_start -= amount;
				self.gap_end -= amount;
			}
			_ => {}
		}
	}

	fn grow_gap(&mut self, amount: usize) {
		let new_gap_end = self.gap_end + amount;
		let old_len = self.buffer.len();

		self.buffer.resize(old_len + amount, GAP_FILLER);
		self.buffer.copy_within(self.gap_end..old_len, new_gap_end);
		self.gap_end = new_gap_end;
	}
}

impl ToString for GapBuffer {
	fn to_string(&self) -> String {
		let mut str = String::new();
		str.extend(self.to_vec().iter());

		str
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_insert() {
		let mut buffer = super::GapBuffer::new();

		buffer.insert("cara", 0);
		assert_eq!(String::from("cara"), buffer.to_string());
		buffer.insert("pyba", 2);
		assert_eq!(String::from("capybara"), buffer.to_string());
	}

	#[test]
	fn test_insert_char() {
		let mut buffer = super::GapBuffer::new();

		buffer.insert_char('i', 0);
		assert_eq!(String::from("i"), buffer.to_string());
		buffer.insert_char('!', 1);
		assert_eq!(String::from("i!"), buffer.to_string());
		buffer.insert_char('H', 0);
		assert_eq!(String::from("Hi!"), buffer.to_string());
	}

	#[test]
	fn test_delete_char() {
		let mut buffer = super::GapBuffer::new();

		buffer.insert("pepperoni", 0);
		buffer.delete_char(2);
		assert_eq!(String::from("peperoni"), buffer.to_string());
	}

	#[test]
	fn test_buffer_overflow() {
		let mut buffer = super::GapBuffer::new();

		buffer.insert("pepperoni", 0);
		buffer.insert(" pepperidze", 9);
		assert_eq!(String::from("pepperoni pepperidze"), buffer.to_string());
	}

	#[test]
	fn test_len() {
		let mut buffer = super::GapBuffer::new();

		buffer.insert("Capybara", 0);
		assert_eq!(8, buffer.len());
	}
}

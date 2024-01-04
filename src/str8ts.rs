use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) enum CellColor {
	#[default]
	White,
	Black,
}

impl Display for CellColor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CellColor::White => write!(f, "White"),
			CellColor::Black => write!(f, "Black"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum CellValue {
	#[default]
	Empty,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
}

impl From<u8> for CellValue {
	fn from(value: u8) -> Self {
		match value {
			1 => CellValue::One,
			2 => CellValue::Two,
			3 => CellValue::Three,
			4 => CellValue::Four,
			5 => CellValue::Five,
			6 => CellValue::Six,
			7 => CellValue::Seven,
			8 => CellValue::Eight,
			9 => CellValue::Nine,
			_ => CellValue::Empty,
		}
	}
}

impl From<usize> for CellValue {
	fn from(value: usize) -> Self {
		match value {
			1 => CellValue::One,
			2 => CellValue::Two,
			3 => CellValue::Three,
			4 => CellValue::Four,
			5 => CellValue::Five,
			6 => CellValue::Six,
			7 => CellValue::Seven,
			8 => CellValue::Eight,
			9 => CellValue::Nine,
			_ => CellValue::Empty,
		}
	}
}

impl From<CellValue> for u8 {
	fn from(value: CellValue) -> Self {
		match value {
			CellValue::One => 1,
			CellValue::Two => 2,
			CellValue::Three => 3,
			CellValue::Four => 4,
			CellValue::Five => 5,
			CellValue::Six => 6,
			CellValue::Seven => 7,
			CellValue::Eight => 8,
			CellValue::Nine => 9,
			_ => 0,
		}
	}
}

impl From<CellValue> for usize {
	fn from(value: CellValue) -> Self {
		match value {
			CellValue::One => 1,
			CellValue::Two => 2,
			CellValue::Three => 3,
			CellValue::Four => 4,
			CellValue::Five => 5,
			CellValue::Six => 6,
			CellValue::Seven => 7,
			CellValue::Eight => 8,
			CellValue::Nine => 9,
			_ => 0,
		}
	}
}

impl PartialOrd for CellValue {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		let my_value: u8 = (*self).into();
		let other_value: u8 = (*other).into();
		my_value.partial_cmp(&other_value)
	}
}

impl Display for CellValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CellValue::Empty => write!(f, " "),
			CellValue::One => write!(f, "1"),
			CellValue::Two => write!(f, "2"),
			CellValue::Three => write!(f, "3"),
			CellValue::Four => write!(f, "4"),
			CellValue::Five => write!(f, "5"),
			CellValue::Six => write!(f, "6"),
			CellValue::Seven => write!(f, "7"),
			CellValue::Eight => write!(f, "8"),
			CellValue::Nine => write!(f, "9"),
		}
	}
}

impl CellValue {
	/// Returns an iterator over all possible cell values.
	///
	/// If `with_empty` is true, the iterator will also return `CellValue::Empty`.
	/// If `with_empty` is false, the iterator will not return `CellValue::Empty`.
	///
	/// # Examples
	/// ```
	/// use str8ts::CellValue;
	///
	/// let mut iter = CellValue::into_iter(true);
	/// assert_eq!(iter.next(), Some(CellValue::Empty));
	/// assert_eq!(iter.next(), Some(CellValue::One));
	/// assert_eq!(iter.next(), Some(CellValue::Two));
	/// assert_eq!(iter.next(), Some(CellValue::Three));
	/// assert_eq!(iter.next(), Some(CellValue::Four));
	/// assert_eq!(iter.next(), Some(CellValue::Five));
	/// assert_eq!(iter.next(), Some(CellValue::Six));
	/// assert_eq!(iter.next(), Some(CellValue::Seven));
	/// assert_eq!(iter.next(), Some(CellValue::Eight));
	/// assert_eq!(iter.next(), Some(CellValue::Nine));
	/// assert_eq!(iter.next(), None);
	/// ```
	///
	/// ```
	/// use str8ts::CellValue;
	///
	/// let mut iter = CellValue::into_iter(false);
	/// assert_eq!(iter.next(), Some(CellValue::One));
	/// assert_eq!(iter.next(), Some(CellValue::Two));
	/// assert_eq!(iter.next(), Some(CellValue::Three));
	/// assert_eq!(iter.next(), Some(CellValue::Four));
	/// assert_eq!(iter.next(), Some(CellValue::Five));
	/// assert_eq!(iter.next(), Some(CellValue::Six));
	/// assert_eq!(iter.next(), Some(CellValue::Seven));
	/// assert_eq!(iter.next(), Some(CellValue::Eight));
	/// assert_eq!(iter.next(), Some(CellValue::Nine));
	/// assert_eq!(iter.next(), None);
	/// ```
	pub(crate) fn into_iter(with_empty: bool) -> CellValueIterator {
		CellValueIterator {
			value: CellValue::Empty,
			is_first: with_empty,
		}
	}
}

pub(crate) struct CellValueIterator {
	value: CellValue,
	is_first: bool,
}

impl Iterator for CellValueIterator {
	type Item = CellValue;

	fn next(&mut self) -> Option<Self::Item> {
		match self.is_first {
			true => {
				self.is_first = false;
				self.value = CellValue::Empty;
				Some(self.value)
			}
			false => {
				let new_value = match self.value {
					CellValue::Empty => Some(CellValue::One),
					CellValue::One => Some(CellValue::Two),
					CellValue::Two => Some(CellValue::Three),
					CellValue::Three => Some(CellValue::Four),
					CellValue::Four => Some(CellValue::Five),
					CellValue::Five => Some(CellValue::Six),
					CellValue::Six => Some(CellValue::Seven),
					CellValue::Seven => Some(CellValue::Eight),
					CellValue::Eight => Some(CellValue::Nine),
					CellValue::Nine => None,
				};
				if let Some(value) = new_value {
					self.value = value;
					Some(self.value)
				} else {
					None
				}
			}
		}
	}
}

impl From<char> for CellValue {
	fn from(c: char) -> Self {
		match c {
			'1' => CellValue::One,
			'2' => CellValue::Two,
			'3' => CellValue::Three,
			'4' => CellValue::Four,
			'5' => CellValue::Five,
			'6' => CellValue::Six,
			'7' => CellValue::Seven,
			'8' => CellValue::Eight,
			'9' => CellValue::Nine,
			_ => CellValue::Empty,
		}
	}
}

impl From<CellValue> for char {
	fn from(value: CellValue) -> Self {
		match value {
			CellValue::One => '1',
			CellValue::Two => '2',
			CellValue::Three => '3',
			CellValue::Four => '4',
			CellValue::Five => '5',
			CellValue::Six => '6',
			CellValue::Seven => '7',
			CellValue::Eight => '8',
			CellValue::Nine => '9',
			_ => ' ',
		}
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Cell {
	pub(crate) color: CellColor,
	pub(crate) value: CellValue,
}

impl Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.color {
			CellColor::White => write!(f, "White({})", self.value),
			CellColor::Black => write!(f, "Black({})", self.value),
		}
	}
}

impl Cell {
	pub(crate) fn new(color: CellColor, value: CellValue) -> Self {
		Cell { color, value }
	}
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Str8ts {
	pub(crate) cells: [[Cell; 9]; 9],
}

impl Display for Str8ts {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut result = String::new();
		for row in 0..9 {
			for col in 0..9 {
				result.push_str(&format!("{} ", self.cells[row][col]));
			}
			result.push('\n');
		}
		write!(f, "{}", result)
	}
}

#[allow(dead_code)]
impl Str8ts {
	pub(crate) fn new() -> Self {
		Str8ts {
			cells: [[Cell::default(); 9]; 9],
		}
	}

	pub(crate) fn set_cell(&mut self, row: u8, col: u8, cell: Cell) {
		self.cells[row as usize][col as usize] = cell;
	}

	pub(crate) fn set_cell_by_index(&mut self, index: u8, cell: Cell) {
		let (row, col) = trans_index_to_row_col!(index);
		self.set_cell(row, col, cell);
	}

	pub(crate) fn set_cell_color(&mut self, row: u8, col: u8, color: CellColor) {
		self.cells[row as usize][col as usize].color = color;
	}

	pub(crate) fn set_cell_color_by_index(&mut self, index: u8, color: CellColor) {
		let (row, col) = trans_index_to_row_col!(index);
		self.set_cell_color(row, col, color);
	}

	pub(crate) fn set_cell_value(&mut self, row: u8, col: u8, value: CellValue) {
		self.cells[row as usize][col as usize].value = value;
	}

	pub(crate) fn set_cell_value_by_index(&mut self, index: u8, value: CellValue) {
		let (row, col) = trans_index_to_row_col!(index);
		self.set_cell_value(row, col, value);
	}

	pub(crate) fn get_cell(&self, row: u8, col: u8) -> Cell {
		self.cells[row as usize][col as usize]
	}

	pub(crate) fn get_cell_by_index(&self, index: u8) -> Cell {
		let (row, col) = trans_index_to_row_col!(index);
		self.get_cell(row, col)
	}

	pub(crate) fn toggle_cell_color(&mut self, row: u8, col: u8) {
		let cell = self.get_cell(row, col);
		match cell.color {
			CellColor::White => self.set_cell_color(row, col, CellColor::Black),
			CellColor::Black => self.set_cell_color(row, col, CellColor::White),
		}
	}

	pub(crate) fn toggle_cell_color_by_index(&mut self, index: u8) {
		let (row, cell) = trans_index_to_row_col!(index);
		self.toggle_cell_color(row, cell);
	}

	pub(crate) fn copy_from(&mut self, other: &Str8ts) {
		for row in 0..9 {
			for col in 0..9 {
				let other_cell = other.get_cell(row, col);
				self.set_cell_color(row, col, other_cell.color);
				self.set_cell_value(row, col, other_cell.value);
			}
		}
	}

	pub(crate) fn clear_all(&mut self) {
		for row in 0..9 {
			for col in 0..9 {
				self.set_cell_color(row, col, CellColor::White);
				self.set_cell_value(row, col, CellValue::Empty);
			}
		}
	}

	pub(crate) fn clear_values(&mut self) {
		for row in 0..9 {
			for col in 0..9 {
				self.set_cell_value(row, col, CellValue::Empty);
			}
		}
	}
}

impl IntoIterator for Str8ts {
	type Item = Cell;
	type IntoIter = Str8tsIterator;

	fn into_iter(self) -> Self::IntoIter {
		Str8tsIterator {
			str8ts: self,
			index: 0,
		}
	}
}

pub(crate) struct Str8tsIterator {
	str8ts: Str8ts,
	index: u8,
}

impl Iterator for Str8tsIterator {
	type Item = Cell;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index < 81 {
			let value = self.str8ts.get_cell_by_index(self.index);
			self.index += 1;
			Some(value)
		} else {
			None
		}
	}
}

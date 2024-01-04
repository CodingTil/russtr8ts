#[macro_export]
macro_rules! trans_index_to_row_col {
	($index:expr) => {
		($index / 9, $index % 9)
	};
}

#[macro_export]
macro_rules! trans_row_col_to_index {
	($row:expr, $col:expr) => {
		$row * 9 + $col
	};
}

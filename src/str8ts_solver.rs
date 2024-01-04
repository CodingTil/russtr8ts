use std::collections::HashMap;
use std::collections::LinkedList;

use russcip::prelude::*;

use crate::str8ts::{Cell, CellColor, CellValue, Str8ts};

impl Str8ts {
	/// Solve the str8ts game.
	///
	/// Returns the solved Str8ts game if the str8ts game was solved successfully. Otherwise, returns None.
	pub fn solve(&self) -> Option<Str8ts> {
		// Preprocess the str8ts game.
		let compartments = find_compartments(self);
		for compartment in compartments.iter() {
			print!("Compartment: ");
			for index in compartment.iter() {
				let (row, col) = trans_index_to_row_col!(*index);
				print!("({},{}), ", row, col);
			}
			println!();
		}

		// Create the model.
		let mut model = Model::new()
			.hide_output()
			.include_default_plugins()
			.create_prob("Str8ts")
			.set_obj_sense(ObjSense::Minimize);

		// Create variables:
		// x_{i}_{k} = 1 if the cell with index i contains the value k. Only relevant for white cells.
		let mut x = HashMap::new();
		for (index, cell) in self.into_iter().enumerate() {
			if cell.color == CellColor::White {
				for value in CellValue::into_iter(false) {
					match cell.value {
						CellValue::Empty => {
							x.insert(
								(index, value),
								model.add_var(
									0.,
									1.,
									0.,
									&format!("x_{}_{}", index, value),
									VarType::Binary,
								),
							);
						}
						v if v == value => {
							// Force to be used
							x.insert(
								(index, value),
								model.add_var(
									1.,
									1.,
									0.,
									&format!("x_{}_{}", index, value),
									VarType::Binary,
								),
							);
						}
						_ => {
							// Force to be not used
							x.insert(
								(index, value),
								model.add_var(
									0.,
									0.,
									0.,
									&format!("x_{}_{}", index, value),
									VarType::Binary,
								),
							);
						}
					}
				}
			}
		}
		// y_{c}_{k} = 1 if the compartment with index c has the least value k
		let mut y = HashMap::new();
		for (compartment_index, compartment) in compartments.iter().enumerate() {
			for value in CellValue::into_iter(false) {
				let numer_value: usize = value.into();
				if compartment.len() <= 9 - numer_value + 1 {
					y.insert(
						(compartment_index, value),
						model.add_var(
							0.,
							1.,
							0.,
							&format!("y_{}_{}", compartment_index, value),
							VarType::Binary,
						),
					);
				} else {
					y.insert(
						(compartment_index, value),
						model.add_var(
							0.,
							0.,
							0.,
							&format!("y_{}_{}", compartment_index, value),
							VarType::Binary,
						),
					);
				}
			}
		}

		// Create constraints:
		// 1. Each cell contains exactly one value.
		for (index, cell) in self.into_iter().enumerate() {
			if cell.color == CellColor::White {
				// grab all the x_i_k variables for this cell with index i
				let x_i = x
					.iter()
					.filter(|(key, _)| key.0 == index)
					.map(|(_, value)| value.clone())
					.collect::<Vec<_>>();
				// create a vector of coefficients for the x_i_k variables (all 1)
				let coeffs = vec![1.; x_i.len()];
				// Add the constraint
				model.add_cons(x_i, &coeffs, 1., 1., &format!("c_1_{}", index));
			}
		}

		// 2. Each value is used at most once in each row.
		// 2.a No two white cells in the same row have the same value.
		for row in 0..9 {
			for value in CellValue::into_iter(false) {
				// grab all the x_i_k variables for this row and value
				let x_i = x
					.iter()
					.filter(|(key, _)| key.0 / 9 == row && key.1 == value)
					.map(|(_, value)| value.clone())
					.collect::<Vec<_>>();
				// create a vector of coefficients for the x_i_k variables (all 1)
				let coeffs = vec![1.; x_i.len()];
				// Add the constraint
				model.add_cons(
					x_i,
					&coeffs,
					-f64::INFINITY,
					1.,
					&format!("c_2a_{}_{}", row, value),
				);
			}
		}
		// 2.b No white cell has the same value as a black cell in the same row.
		for row in 0..9 {
			// grab all the non-empty values of black cells in this row
			let mut black_values = Vec::new();
			for col in 0..9 {
				let cell = self.get_cell(row, col);
				if cell.color == CellColor::Black && cell.value != CellValue::Empty {
					black_values.push(cell.value);
				}
			}
			// no duplicate values (otherwise would be illegal to begin with)
			assert!(
				black_values.len()
					== black_values
						.iter()
						.collect::<std::collections::HashSet<_>>()
						.len(),
				"There are duplicate values in the black cells of row {}!",
				row
			);
			for value in black_values.iter() {
				// grab all the x_i_k variables for this row and value
				let x_i = x
					.iter()
					.filter(|(key, _)| key.0 / 9 == row.into() && key.1 == *value)
					.map(|(_, value)| value.clone())
					.collect::<Vec<_>>();
				for x_i_k in x_i.iter() {
					// Add the constraint
					model.add_cons(
						vec![x_i_k.clone()],
						&[1.],
						-f64::INFINITY,
						0.,
						&format!("c_2b_{}_{}", row, value),
					);
				}
			}
		}

		// 3. Each value is used at most once in each column.
		// 3.a No two white cells in the same column have the same value.
		for col in 0..9 {
			for value in CellValue::into_iter(false) {
				// grab all the x_i_k variables for this column and value
				let x_i = x
					.iter()
					.filter(|(key, _)| key.0 % 9 == col && key.1 == value)
					.map(|(_, value)| value.clone())
					.collect::<Vec<_>>();
				// create a vector of coefficients for the x_i_k variables (all 1)
				let coeffs = vec![1.; x_i.len()];
				// Add the constraint
				model.add_cons(
					x_i,
					&coeffs,
					-f64::INFINITY,
					1.,
					&format!("c_3_{}_{}", col, value),
				);
			}
		}
		// 3.b No white cell has the same value as a black cell in the same column.
		for col in 0..9 {
			// grab all the non-empty values of black cells in this column
			let mut black_values = Vec::new();
			for row in 0..9 {
				let cell = self.get_cell(row, col);
				if cell.color == CellColor::Black && cell.value != CellValue::Empty {
					black_values.push(cell.value);
				}
			}
			// no duplicate values (otherwise would be illegal to begin with)
			assert!(
				black_values.len()
					== black_values
						.iter()
						.collect::<std::collections::HashSet<_>>()
						.len(),
				"There are duplicate values in the black cells of column {}!",
				col
			);
			for value in black_values.iter() {
				// grab all the x_i_k variables for this column and value
				let x_i = x
					.iter()
					.filter(|(key, _)| key.0 % 9 == col.into() && key.1 == *value)
					.map(|(_, value)| value.clone())
					.collect::<Vec<_>>();
				for x_i_k in x_i.iter() {
					// Add the constraint
					model.add_cons(
						vec![x_i_k.clone()],
						&[1.],
						-f64::INFINITY,
						0.,
						&format!("c_3b_{}_{}", col, value),
					);
				}
			}
		}

		// 4. Each compartment has exactly one least value.
		for (compartment_index, _) in compartments.iter().enumerate() {
			// grab all the y_c_k variables for this compartment with index c
			let y_c = y
				.iter()
				.filter(|(key, _)| key.0 == compartment_index)
				.map(|(_, value)| value.clone())
				.collect::<Vec<_>>();
			// create a vector of coefficients for the y_c_k variables (all 1)
			let coeffs = vec![1.; y_c.len()];
			// Add the constraint
			model.add_cons(y_c, &coeffs, 1., 1., &format!("c_4_{}", compartment_index));
		}

		// 5. Each compartment has adjacent values.
		for (compartment_index, compartment) in compartments.iter().enumerate() {
			for value in CellValue::into_iter(false) {
				let number_value: usize = value.into();
				if compartment.len() > 9 - number_value + 1 {
					break;
				}
				// get the y_c_k variable for this compartment and value
				let y_c_k = y.get(&(compartment_index, value)).unwrap();
				// create a vector of coefficients for the x_i_k variables (all 1) and the y_c_k variable (-1)
				let mut coeffs = vec![1.; compartment.len() + 1];
				coeffs[compartment.len()] = -1.;

				let mut count = compartment.len();
				for next_value in CellValue::into_iter(false) {
					if next_value < value {
						continue;
					}
					if count == 0 {
						break;
					}
					// grab all the x_i_k variables for this compartment and value
					let mut vars = Vec::new();
					for index in compartment {
						vars.push(x.get(&((*index as usize), next_value)).unwrap().clone());
					}
					// get the y_c_k variable for this compartment and value
					vars.push(y_c_k.clone());
					model.add_cons(
						vars,
						&coeffs,
						0.,
						f64::INFINITY,
						&format!("c_5_{}_{}_{}", compartment_index, value, next_value),
					);
					count -= 1;
				}
			}
		}

		// Solve the model.
		let solved_model = model.solve();

		if solved_model.status() != Status::Optimal {
			return None;
		}

		// Get the solution.
		let solution = solved_model.best_sol().unwrap();

		// Set the values of the str8ts game.
		let mut solved_str8ts = Str8ts::new();
		for (index, cell) in self.into_iter().enumerate() {
			if cell.color == CellColor::White {
				for value in CellValue::into_iter(false) {
					if solution.val(x.get(&(index, value)).unwrap().clone()) >= 0.5 {
						solved_str8ts
							.set_cell_by_index(index as u8, Cell::new(CellColor::White, value));
					}
				}
			} else {
				solved_str8ts.set_cell_by_index(index as u8, cell);
			}
		}

		// Assert that each white cell has a value not empty.
		for (index, cell) in solved_str8ts.into_iter().enumerate() {
			if cell.color == CellColor::White {
				assert!(
					cell.value != CellValue::Empty,
					"Cell with index {} has no value!",
					index
				);
			}
		}

		Some(solved_str8ts)
	}
}

/// Find all compartments in the str8ts game.
///
/// A compartment is a set of adjecent white cells either within the same row or within the same column.
/// Therefore, compartments are seperated by black cells and the border of the str8ts game.
fn find_compartments(str8ts: &Str8ts) -> LinkedList<LinkedList<u8>> {
	let mut compartments = LinkedList::new();

	// Search for compartments in each row.
	let row_compartments = find_compartments_rows(str8ts);
	for row_compartment in row_compartments {
		compartments.push_back(row_compartment);
	}

	// Search for compartments in each column.
	let col_compartments = find_compartments_cols(str8ts);
	for col_compartment in col_compartments {
		compartments.push_back(col_compartment);
	}

	compartments
}

/// Find all row-compartments in the str8ts game.
fn find_compartments_rows(str8ts: &Str8ts) -> LinkedList<LinkedList<u8>> {
	let mut compartments = LinkedList::new();
	// Search for compartments in each row.
	for row in 0..9 {
		// A compartment is a set of adjecent white cells within the same row.
		let mut compartment = LinkedList::new();
		for col in 0..9 {
			let cell = str8ts.get_cell(row, col);
			match cell.color {
				CellColor::Black => {
					if !compartment.is_empty() {
						// If the first cell in that row is black, we have no compartment to add.
						compartments.push_back(compartment.clone());
						compartment.clear();
					}
				}
				CellColor::White => {
					compartment.push_back(trans_row_col_to_index!(row, col));
				}
			}
		}
		if !compartment.is_empty() {
			// If the last cell in that row is white, we have a compartment to add.
			compartments.push_back(compartment);
		}
	}
	compartments
}

/// Find all column-compartments in the str8ts game.
fn find_compartments_cols(str8ts: &Str8ts) -> LinkedList<LinkedList<u8>> {
	let mut compartments = LinkedList::new();
	// Search for compartments in each column.
	for col in 0..9 {
		// A compartment is a set of adjecent white cells within the same column.
		let mut compartment = LinkedList::new();
		for row in 0..9 {
			let cell = str8ts.get_cell(row, col);
			match cell.color {
				CellColor::Black => {
					if !compartment.is_empty() {
						// If the first cell in that row is black, we have no compartment to add.
						compartments.push_back(compartment.clone());
						compartment.clear();
					}
				}
				CellColor::White => {
					compartment.push_back(trans_row_col_to_index!(row, col));
				}
			}
		}
		if !compartment.is_empty() {
			// If the last cell in that row is white, we have a compartment to add.
			compartments.push_back(compartment);
		}
	}
	compartments
}

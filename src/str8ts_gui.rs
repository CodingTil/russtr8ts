use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::{theme, Background, BorderRadius, Color, Element, Length, Sandbox, Settings};
use iced_style::{text_input, Theme};

use crate::str8ts::{CellColor, CellValue, Str8ts};

pub(crate) fn run() -> iced::Result {
	Str8tsEditor::run(Settings::default())
}

struct Str8tsEditor {
	str8ts: Str8ts,
}

#[derive(Debug, Clone)]
enum Message {
	CellInputChanged(u8, u8, String),
	CellColorToggled(u8, u8),
	SolveRequested,
	ClearAll,
	ClearValues,
}

struct CustomCellStyle {
	is_black: bool,
}

impl text_input::StyleSheet for CustomCellStyle {
	type Style = Theme;

	fn active(&self, _: &Self::Style) -> text_input::Appearance {
		text_input::Appearance {
			background: if self.is_black {
				Background::Color(Color::BLACK)
			} else {
				Background::Color(Color::WHITE)
			},
			border_color: if self.is_black {
				Color::WHITE
			} else {
				Color::BLACK
			},
			icon_color: if self.is_black {
				Color::WHITE
			} else {
				Color::BLACK
			},
			border_radius: BorderRadius::default(),
			border_width: 1.0,
		}
	}

	fn focused(&self, style: &Self::Style) -> text_input::Appearance {
		self.active(style)
	}

	fn placeholder_color(&self, _: &Self::Style) -> Color {
		if self.is_black {
			Color::WHITE
		} else {
			Color::BLACK
		}
	}

	fn value_color(&self, _: &Self::Style) -> Color {
		if self.is_black {
			Color::WHITE
		} else {
			Color::BLACK
		}
	}

	fn disabled_color(&self, _: &Self::Style) -> Color {
		if self.is_black {
			Color::WHITE
		} else {
			Color::BLACK
		}
	}

	fn selection_color(&self, _: &Self::Style) -> Color {
		if self.is_black {
			Color::WHITE
		} else {
			Color::BLACK
		}
	}

	fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
		self.active(style)
	}
}

impl Sandbox for Str8tsEditor {
	type Message = Message;

	fn new() -> Self {
		Self {
			str8ts: Str8ts::new(),
		}
	}

	fn title(&self) -> String {
		String::from("Str8ts Editor")
	}

	fn update(&mut self, message: Message) {
		match message {
			Message::CellInputChanged(row, col, value) => {
				// Update logic for changing cell input
				// Get new value
				// if not empty or in [1, 9] -> do nothing
				let value = match value.trim().parse::<u8>() {
					Ok(value) => match value {
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
					},
					Err(_) => CellValue::Empty,
				};
				// Update cell
				self.str8ts.set_cell_value(row, col, value)
			}
			Message::CellColorToggled(row, col) => {
				// Update logic for toggling cell color
				self.str8ts.toggle_cell_color(row, col);
			}
			Message::SolveRequested => {
				// Update logic for solving the str8ts game
				// Print str8ts game
				println!("{}", self.str8ts);
				// Solve str8ts game
				println!("Solving...");
				let solved_str8ts = self.str8ts.solve();
				println!("Solved!");
				// Update str8ts game
				if let Some(solved_str8ts) = solved_str8ts {
					println!("Solution found!");
					println!("{}", solved_str8ts);
					self.str8ts.copy_from(&solved_str8ts);
				} else {
					println!("No solution found!");
				}
			}
			Message::ClearAll => {
				// Update logic for clearing the str8ts game
				self.str8ts.clear_all();
			}
			Message::ClearValues => {
				// Update logic for clearing the str8ts game
				self.str8ts.clear_values();
			}
		}
	}

	fn view(&self) -> Element<Message> {
		let mut board = Column::new().spacing(10);

		for row in 0..9 {
			let mut row_cells = Row::new().spacing(10);
			for col in 0..9 {
				let cell = self.str8ts.get_cell(row, col);
				let input = TextInput::new("", cell.value.to_string().as_str())
					.on_input(move |v| Message::CellInputChanged(row, col, v))
					.width(Length::Fixed(35.0))
					.style(theme::TextInput::Custom(Box::new(CustomCellStyle {
						is_black: cell.color == CellColor::Black,
					})));

				let button = Button::new("").on_press(Message::CellColorToggled(row, col));

				row_cells = row_cells.push(Container::new(input).width(Length::Shrink));
				row_cells = row_cells.push(Container::new(button).width(Length::Shrink));
			}
			board = board.push(row_cells);
		}

		let mut button_row = Row::new().spacing(10);
		let solve_button = Button::new(Text::new("Solve")).on_press(Message::SolveRequested);
		let clear_all_button = Button::new(Text::new("Clear All")).on_press(Message::ClearAll);
		let clear_values_button =
			Button::new(Text::new("Clear Values")).on_press(Message::ClearValues);
		button_row = button_row.push(Container::new(solve_button).width(Length::Shrink));
		button_row = button_row.push(Container::new(clear_all_button).width(Length::Shrink));
		button_row = button_row.push(Container::new(clear_values_button).width(Length::Shrink));

		board = board.push(button_row);

		Container::new(board).into()
	}
}

use crate::str8ts_gui::run;

#[macro_use]
pub mod macros;
pub mod str8ts;
pub mod str8ts_gui;
pub mod str8ts_solver;

fn main() {
	let _ = run();
}

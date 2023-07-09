use crate::style::chars;
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

#[derive(Debug, Clone)]
pub struct Confirm {
	message: String,
	initial_value: bool,
	prompts: (String, String),
}

impl Confirm {
	#[must_use]
	pub fn new<S: Into<String>>(message: S) -> Confirm {
		Confirm {
			message: message.into(),
			initial_value: false,
			prompts: ("Yes".into(), "No".into()),
		}
	}

	#[must_use]
	pub fn initial_value(mut self, b: bool) -> Self {
		self.initial_value = b;
		self
	}

	#[must_use]
	pub fn prompts<S: Into<String>>(mut self, yes: S, no: S) -> Self {
		self.prompts = (yes.into(), no.into());
		self
	}

	// todo: Result
	#[must_use]
	pub fn interact(self) -> Option<bool> {
		self.init();

		let term = Term::stdout();
		// let _ = term.hide_cursor(); // todo

		let mut a = self.initial_value;
		loop {
			match term.read_key().ok()? {
				Key::ArrowUp | Key::ArrowDown | Key::ArrowLeft | Key::ArrowRight => {
					a = !a;
					self.draw(a);
				}
				Key::Char('y') | Key::Char('Y') => {
					let _ = term.show_cursor();
					self.out(true);
					return Some(true);
				}
				Key::Char('n') | Key::Char('N') => {
					let _ = term.show_cursor();
					self.out(false);
					return Some(false);
				}
				Key::Enter => {
					let _ = term.show_cursor();
					self.out(a);
					return Some(a);
				}
				_ => {}
			}
		}
	}
}

impl Confirm {
	fn radio_pnt(b: bool, w: &str) -> String {
		if b {
			format!("{} {}", style(*chars::RADIO_ACTIVE).green(), w)
		} else {
			style(format!("{} {}", *chars::RADIO_INACTIVE, w))
				.dim()
				.to_string()
		}
	}

	fn radio(&self, b: bool) -> String {
		let yes = Confirm::radio_pnt(b, &self.prompts.0);
		let no = Confirm::radio_pnt(!b, &self.prompts.1);

		format!("{} / {}", yes, no)
	}

	fn draw(&self, a: bool) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let r = self.radio(a);
		print!("{}  {}", style("│").cyan(), r);
		let _ = stdout.flush();
	}
}

impl Confirm {
	fn init(&self) {
		let mut stdout = stdout();

		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);
		println!("{}", style(*chars::BAR).cyan());
		print!("{}", style(*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		self.draw(self.initial_value);

		let _ = stdout.flush();
	}

	fn out(&self, value: bool) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		let answ = if value {
			&self.prompts.0
		} else {
			&self.prompts.1
		};

		let len = 2 + self.prompts.0.chars().count() + 3 + 2 + self.prompts.1.chars().count();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);
		println!(
			"{}  {}{}",
			*chars::BAR,
			style(answ).dim(),
			" ".repeat(len - answ.len())
		);
	}
}

#[must_use]
pub fn prompt<S: Into<String>>(message: S) -> Confirm {
	Confirm::new(message)
}

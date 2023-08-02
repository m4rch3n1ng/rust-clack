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
	pub fn new<S: Into<String>>(message: S) -> Confirm {
		Confirm {
			message: message.into(),
			initial_value: false,
			prompts: ("Yes".into(), "No".into()),
		}
	}

	pub fn initial_value(&mut self, b: bool) -> &mut Self {
		self.initial_value = b;
		self
	}

	pub fn prompts<S: Into<String>>(&mut self, yes: S, no: S) -> &mut Self {
		self.prompts = (yes.into(), no.into());
		self
	}

	// todo: Result
	pub fn interact(&self) -> Option<bool> {
		self.w_init();

		let term = Term::stdout();
		// let _ = term.hide_cursor(); // todo

		let mut val = self.initial_value;
		loop {
			match term.read_key().ok()? {
				Key::ArrowUp | Key::ArrowDown | Key::ArrowLeft | Key::ArrowRight => {
					val = !val;
					self.draw(val);
				}
				Key::Char('y' | 'Y') => {
					let _ = term.show_cursor();
					self.w_out(true);
					return Some(true);
				}
				Key::Char('n' | 'N') => {
					let _ = term.show_cursor();
					self.w_out(false);
					return Some(false);
				}
				Key::Enter => {
					let _ = term.show_cursor();
					self.w_out(val);
					return Some(val);
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
	fn w_init(&self) {
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

	fn w_out(&self, value: bool) {
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

pub fn confirm<S: Into<String>>(message: S) -> Confirm {
	Confirm::new(message)
}

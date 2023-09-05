use console::style;
use may_clack::{cancel, error::ClackError, input, intro, multi_input, outro};

fn main() -> Result<(), ClackError> {
	println!();
	intro!(style(" validate ").reverse());

	let do_validate_input = input("validate single")
		.validate(|x| (!x.is_ascii()).then_some("only use ascii characters"))
		.cancel(do_cancel)
		.required();
	let do_validate_multi_input = multi_input("validate multi")
		.validate(|x| x.parse::<u32>().err().map(|_| "invalid u32"))
		.cancel(do_cancel)
		.interact();

	outro!();

	println!("single {:?}", do_validate_input);
	println!("multi {:?}", do_validate_multi_input);

	Ok(())
}

fn do_cancel() {
	cancel!("demo cancelled");
	panic!("demo cancelled");
}

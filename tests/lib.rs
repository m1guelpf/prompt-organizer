use prompt_organizer::prompt;
use trybuild::TestCases;

#[test]
fn can_replace_a_single_arg() {
	prompt! {named_arg_test, "Hello, {name}!"}
	assert_eq!(named_arg_test("world"), "Hello, world!");
}

#[test]
fn can_replace_multiple_args() {
	prompt! {named_args_test, "{hello}, {name}!"}
	assert_eq!(named_args_test("Hello", "world"), "Hello, world!");
}

#[test]
fn works_with_no_args() {
	prompt! {no_args_test, "Hello, world!"}
	assert_eq!(no_args_test(), "Hello, world!");
}

#[test]
fn works_with_multiline_strings() {
	prompt! {multiline_string_test, "
        You are {name}, an AI assistant.

        This prompt has multiple lines.
    "}
	assert_eq!(
		multiline_string_test("test"),
		"You are test, an AI assistant.\n\nThis prompt has multiple lines."
	);
}

#[test]
fn works_with_raw_string_literals() {
	prompt! {raw_strlit_test, r#"
        You are {name}, an AI assistant.

        "this has quotes"

        This prompt has multiple lines.
    "#}
	assert_eq!(
		raw_strlit_test("test"),
		"You are test, an AI assistant.\n\n\"this has quotes\"\n\nThis prompt has multiple lines."
	);
}

#[test]
fn ensure_snapshot_panics() {
	let t = TestCases::new();
	t.compile_fail("tests/panic/*.rs");
}

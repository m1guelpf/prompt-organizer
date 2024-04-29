#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![doc = include_str!("../README.md")]

use parse_format::{ParseMode, Parser, Piece, Position};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::rc::Rc;
use syn::{parse::Parse, parse_macro_input, ExprPath, LitStr, Token};
use unindent::unindent;

struct MacroInput {
	name: Ident,
	prompt: String,
}

impl Parse for MacroInput {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let name: ExprPath = input.parse()?;
		input.parse::<Token![,]>()?;
		let prompt: LitStr = input.parse()?;

		let segments = name.path.segments;
		assert!(segments.len() == 1, "Function name is required");

		Ok(Self {
			name: segments.first().unwrap().ident.clone(),
			prompt: unindent(prompt.value().trim()),
		})
	}
}

/// Generates a function that formats a string using named arguments.
///
/// # Example
///
/// ```
/// # use prompt_organizer::prompt;
/// prompt! {example_prompt, "
/// You are {name}, an AI assistant.
/// "}
/// # assert_eq!(example_prompt("test"), "You are test, an AI assistant.");
/// ```
///
/// # Panics
///
/// This macro will panic if the function name is not provided or if the prompt string contains any unnamed arguments.
#[proc_macro]
pub fn prompt(input: TokenStream) -> TokenStream {
	let MacroInput { name, prompt } = parse_macro_input!(input as MacroInput);

	let parser = Parser::new(&prompt, None, None, false, ParseMode::Format);
	let args = parser
		.filter_map(|piece| match piece {
			Piece::NextArgument(arg) => Some(arg),
			Piece::String(_) => None,
		})
		.collect::<Rc<_>>();

	if args
		.iter()
		.any(|arg| !matches!(arg.position, Position::ArgumentNamed(_)))
	{
		panic!("Only named arguments are supported, e.g. {{name}}");
	}

	let args = args
		.iter()
		.map(|arg| {
			let Position::ArgumentNamed(name) = arg.position else {
				unreachable!()
			};

			Ident::new(name, Span::call_site())
		})
		.collect::<Vec<_>>();

	let tokens = quote! {
		pub fn #name(#(#args: &str),*) -> String {
			::std::format!(#prompt, #(#args = #args),*)
		}
	};

	tokens.into()
}

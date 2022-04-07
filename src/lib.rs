//! Stupid and cursed Rust procedural macro that runs a C preprocessor on the input
//!
//! # Example
//!
//! ```no_run
//! cpreprocess::cpreprocess!(r#"
//!     #define MACRO(NAME) fn print_ ## NAME () { println!("hello world"); }
//!
//!     MACRO(hello_world)
//!
//!     print_hello_world()
//! "#);
//! ```

mod cpp;

use proc_macro::TokenStream;

#[proc_macro]
/// Stupid and cursed Rust procedural macro that runs a C preprocessor on the input
///
/// # Example
///
/// ```no_run
/// cpreprocess::cpreprocess!(r#"
///     #define MACRO(NAME) fn print_ ## NAME () { println!("hello world"); }
///
///     MACRO(hello_world)
///
///     print_hello_world()
/// "#);
/// ```
pub fn cpreprocess(tokens: TokenStream) -> TokenStream {
	#[cfg(not(feature = "nightly"))]
	let tokens = syn::parse_macro_input!(tokens as syn::LitStr).value();

	#[cfg(feature = "nightly")]
	let tokens = match syn::parse::<syn::LitStr>(tokens.clone()) {
		Ok(tokens) => tokens.value(),
		Err(_) => proc_macro_faithful_display::faithful_display(&tokens).to_string()
	};

	match cpp::preprocess(tokens.as_bytes())
		.map(|result| {
			result.and_then(|code| {
				String::from_utf8_lossy(&code).parse().map_err(Into::into)
			})
		})
	{
		Some(Ok(code)) => code,
		Some(Err(err)) => format!("compile_error!(\"{}\")", err.to_string().replace('\\', "\\\\").replace('"', "\\\"")).parse().unwrap(),
		None => "compile_error!(\"Couldn't find a compatible C compiler on this system\")".parse().unwrap()
	}
}
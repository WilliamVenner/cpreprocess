mod cpp;

use proc_macro::TokenStream;

#[proc_macro]
pub fn cpreprocess(tokens: TokenStream) -> TokenStream {
	let tokens = syn::parse_macro_input!(tokens as syn::LitStr).value();
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
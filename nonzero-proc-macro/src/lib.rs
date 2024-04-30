use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Ident, LitInt, Token,
};

#[proc_macro]
pub fn __nonzero(args: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(args as Args);
    expand(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(args: Args) -> syn::Result<proc_macro2::TokenStream> {
    let Args {
        crate_token: _,
        eq_token: _,
        krate,
        semi_token: _,
        lit_int,
    } = args;
    let ty: syn::Ident = match lit_int.suffix() {
        // we know the suffix, so use the concrete
        "i8" => parse_quote!(i8),
        "i16" => parse_quote!(i16),
        "i32" => parse_quote!(i32),
        "i64" => parse_quote!(i64),
        "i128" => parse_quote!(i128),
        "isize" => parse_quote!(isize),
        "u8" => parse_quote!(u8),
        "u16" => parse_quote!(u16),
        "u32" => parse_quote!(u32),
        "u64" => parse_quote!(u64),
        "u128" => parse_quote!(u128),
        "usize" => parse_quote!(usize),
        "" => {
            return Err(syn::Error::new(
                lit_int.span(),
                "must have a suffix like the `usize` in `1usize`",
            ))
        }
        _ => {
            return Err(syn::Error::new(
                lit_int.span(),
                "unknown integer literal suffix",
            ))
        }
    };
    Ok(quote! {
        #krate::#ty!(#lit_int)
    })
}

#[allow(unused)]
struct Args {
    crate_token: Token![crate],
    eq_token: Token![=],
    krate: Path,
    semi_token: Token![;],
    lit_int: LitInt,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            crate_token: input.parse()?,
            eq_token: input.parse()?,
            krate: input.parse()?,
            semi_token: input.parse()?,
            lit_int: input.parse()?,
        })
    }
}

struct Path {
    leading_colon: Option<Token![::]>,
    segments: Punctuated<Ident, Token![::]>,
}

impl Parse for Path {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            leading_colon: input.parse()?,
            segments: input.call(Punctuated::parse_separated_nonempty)?,
        })
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            leading_colon,
            segments,
        } = self;
        leading_colon.to_tokens(tokens);
        segments.to_tokens(tokens);
    }
}

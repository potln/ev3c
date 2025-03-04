use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

macro_rules! seperate {
    () => { vec![] };
    ($elem:expr; $n:expr) => { vec![$elem, $n] };
    ($($x:expr),+ $(,)?) => { vec![$($x, TokenTree::Punct(Punct::new(',', Spacing::Joint))),+] };
}

#[proc_macro]
pub fn assemble(input: TokenStream) -> TokenStream {
    for token in input.clone() {
        println!("{:?}", input);
    }

    let mut numbers = TokenStream::new();

    numbers.extend(seperate![
        TokenTree::Literal(Literal::u16_unsuffixed(1)),
        TokenTree::Literal(Literal::u16_unsuffixed(1)),
        TokenTree::Literal(Literal::u16_unsuffixed(1)),
        TokenTree::Literal(Literal::u16_unsuffixed(1)),
    ]);

    let mut tokens: TokenStream = TokenStream::new();

    tokens.extend(vec![
        TokenTree::Ident(Ident::new("vec", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Joint)),
        TokenTree::Group(Group::new(Delimiter::Brace, numbers)),
    ]);

    return tokens;
}

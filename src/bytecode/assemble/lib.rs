use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

macro_rules! byte {
    ($e: expr) => {
        TokenTree::Literal(Literal::u8_suffixed($e))
    }
}

macro_rules! expect {
    ($t: expr, TokenTree::Ident, $m: literal) => {
        match $t {
            TokenTree::Ident(inner) => inner,
            _ => panic!($m)
        }
    };

    ($t: expr, TokenTree::Literal, $m: literal) => {
        match $t {
            TokenTree::Literal(inner) => inner,
            _ => panic!($m)
        }
    };

    ($t: expr, TokenTree::Punct, $m: literal) => {
        match $t {
            TokenTree::Punct(inner) => inner,
            _ => panic!($m)
        }
    };

    ($t: expr, TokenTree::Group, $m: literal) => {
        match $t {
            TokenTree::Group(inner) => inner,
            _ => panic!($m)
        }
    };
}

#[proc_macro]
pub fn assemble(input: TokenStream) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut bytes = TokenStream::new();

    for token in input {
        let bytecode = parse(&mut bytes, token);
        bytes.extend(bytecode);
    }

    tokens.extend(vec![
        TokenTree::Ident(Ident::new("vec", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Joint)),
        TokenTree::Group(Group::new(Delimiter::Brace, bytes)),
    ]);

    return tokens;
}

fn parse(bytes: &mut TokenStream, token: TokenTree) -> Vec<TokenTree> {
    let op = expect!(token, TokenTree::Ident, "Hello");
    let mnemonic = op.to_string().as_str();

    return match mnemonic {
        "err" => vec![byte!(ev3c::bytecodes::opcodes::Op::Error)],
        _ => panic!("Unknown mnemonic {}", mnemonic)
    }
}
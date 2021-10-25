use proc_macro::{Ident, TokenStream, TokenTree};
use proc_macro2::LineColumn;
use quote::quote;
use syn::{parse_quote, Expr, LitStr};

#[proc_macro_attribute]
pub fn procbind(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro]
pub fn proc_code(input: TokenStream) -> TokenStream {
    let mut stream = vec![];
    build_proc_code(&mut stream, input);
    let exprs = stream
        .into_iter()
        .map(ProcCode::into_expr)
        .collect::<Vec<_>>();

    let mut spaces = Vec::new();
    for i in 1..exprs.len() {
        spaces.push(spaces_between(&exprs[i - 1].1, &exprs[i].1));
    }
    spaces.push("\n".to_owned());

    let exprs = exprs.into_iter().map(|(expr, _)| expr).collect::<Vec<_>>();

    // let mut output: Expr = parse_quote!("");
    // for expr in exprs {
    //     output = parse_quote!(#output + #expr);
    // }
    let expanded = quote!(
        {
            let mut __proc_code_output = std::string::String::new();
            #(
                __proc_code_output.push_str(#exprs);
                __proc_code_output.push_str(#spaces);
            )*
            __proc_code_output
        }
    );

    TokenStream::from(expanded)
}

fn build_proc_code(stream: &mut Vec<ProcCode>, input: TokenStream) {
    let mut iter = input.into_iter();
    while let Some(token) = iter.next() {
        match token {
            TokenTree::Ident(ident) => {
                push_token(stream, ident.span().into(), ident);
            }
            TokenTree::Literal(lit) => {
                push_token(stream, lit.span().into(), lit);
            }
            TokenTree::Punct(punct) if punct.as_char() == '#' => {
                let ident = iter.next().expect("expected identifier");
                if let TokenTree::Ident(ident) = ident {
                    let span: Span = punct.span().into();
                    let span = span.join(ident.span().into());

                    stream.push(ProcCode::Variable(ident, span));
                } else {
                    panic!("expected identifier");
                }
            }
            TokenTree::Punct(punct) => {
                push_token(stream, punct.span().into(), punct);
            }
            TokenTree::Group(group) => match group.delimiter() {
                proc_macro::Delimiter::Brace => {
                    push_token(stream, group.span_open().into(), "{");
                    build_proc_code(stream, group.stream());
                    push_token(stream, group.span_close().into(), "}");
                }
                proc_macro::Delimiter::Bracket => {
                    push_token(stream, group.span_open().into(), "[");
                    build_proc_code(stream, group.stream());
                    push_token(stream, group.span_close().into(), "]");
                }
                proc_macro::Delimiter::Parenthesis => {
                    push_token(stream, group.span_open().into(), "(");
                    build_proc_code(stream, group.stream());
                    push_token(stream, group.span_close().into(), ")");
                }
                proc_macro::Delimiter::None => {
                    build_proc_code(stream, group.stream());
                }
            },
        }
    }
}

fn push_token<T: ToString>(stream: &mut Vec<ProcCode>, token_span: Span, token: T) {
    let last = stream.last_mut();

    match last {
        Some(ProcCode::String(str, span)) => {
            println!(
                "push {:?} at {:?} into {:?}",
                token.to_string(),
                token_span,
                span
            );
            str.push_str(&spaces_between(span, &token_span));
            str.push_str(&token.to_string());
            *span = span.join(token_span.into());
        }
        Some(ProcCode::Variable(_, _)) | None => {
            stream.push(ProcCode::String(token.to_string(), token_span));
        }
    }
}

enum ProcCode {
    String(String, Span),
    Variable(Ident, Span),
}

impl ProcCode {
    fn into_expr(self) -> (Expr, Span) {
        match self {
            ProcCode::String(str, span) => {
                let lit_str = LitStr::new(&str, proc_macro2::Span::call_site());
                (parse_quote!(#lit_str), span)
            }
            ProcCode::Variable(val, span) => {
                let val = syn::Ident::new(&val.to_string(), val.span().into());
                let expr: Expr = parse_quote!(& #val.to_string());
                (expr, span)
            }
        }
    }
}

fn spaces_between(from: &Span, to: &Span) -> String {
    let start = from.end();
    let end = to.start();
    let mut spaces = String::new();

    if start.line == end.line {
        for col in start.column..end.column {
            spaces.push_str(" ");
        }
    } else {
        for line in start.line..end.line {
            spaces.push_str("\r\n");
        }
        for col in 0..end.column {
            spaces.push_str(" ");
        }
    }

    spaces
}

#[derive(Debug, Clone, Copy)]
struct Span {
    start: LineColumn,
    end: LineColumn,
}

impl Span {
    pub fn join(self, b: Span) -> Span {
        Span {
            start: self.start,
            end: b.end,
        }
    }

    pub fn start(&self) -> LineColumn {
        self.start
    }

    pub fn end(&self) -> LineColumn {
        self.end
    }
}

impl From<proc_macro::Span> for Span {
    fn from(span: proc_macro::Span) -> Self {
        let span: proc_macro2::Span = span.into();
        Self {
            start: span.start(),
            end: span.end(),
        }
    }
}

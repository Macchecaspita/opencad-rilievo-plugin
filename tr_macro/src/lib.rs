//! `tr_impl!(it, en, es, fr, tr, pt, pl)`: the seven forms written in the
//! source, plus the other languages of Open CAD Studio read at build time
//! from `i18n/<code>.json` (an object: English text → translated text).
//!
//! The key is the first string literal of the English form, as written
//! (escapes resolved). In the other forms that literal is swapped for the
//! translation, keeping its span, so `format!("… {x} …")` still captures `x`.
//! A missing translation falls back to the English form.

use proc_macro::{Delimiter, Group, Literal, TokenStream, TokenTree};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Codes and enum variants of the languages read from the catalogs, in
/// `crate::i18n::Lang` order after the seven written in the source.
const EXTRA: &[(&str, &str)] = &[
    ("bg", "Bg"), ("cs", "Cs"), ("de", "De"), ("el", "El"), ("fi", "Fi"), ("hi", "Hi"), ("hu", "Hu"),
    ("ja", "Ja"), ("ko", "Ko"), ("lo", "Lo"), ("nl", "Nl"), ("ru", "Ru"), ("vi", "Vi"), ("zh-CN", "ZhCn"),
    ("zh-TW", "ZhTw"), ("ar", "Ar"),
];
const WRITTEN: &[&str] = &["It", "En", "Es", "Fr", "Tr", "Pt", "Pl"];

fn catalogs() -> &'static HashMap<String, HashMap<String, String>> {
    static C: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();
    C.get_or_init(|| {
        let dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join("i18n");
        let mut all = HashMap::new();
        for (code, _) in EXTRA {
            let text = std::fs::read_to_string(dir.join(format!("{code}.json"))).unwrap_or_default();
            all.insert(code.to_string(), parse_object(&text).unwrap_or_default());
        }
        all
    })
}

#[proc_macro]
pub fn tr_impl(input: TokenStream) -> TokenStream {
    let args = split_args(input);
    if args.len() != 7 {
        return "compile_error!(\"tr!: seven forms expected (it, en, es, fr, tr, pt, pl)\")".parse().unwrap();
    }
    let key = first_literal(&args[1]).and_then(|l| unescape(&l.to_string()));
    let mut arms = String::new();
    let mut toks: Vec<TokenStream> = Vec::new();
    for (i, v) in WRITTEN.iter().enumerate() {
        arms.push_str(&format!("crate::i18n::Lang::{v} => __A{i},"));
        toks.push(args[i].clone());
    }
    for (j, (code, v)) in EXTRA.iter().enumerate() {
        let i = WRITTEN.len() + j;
        arms.push_str(&format!("crate::i18n::Lang::{v} => __A{i},"));
        let tr = key.as_ref().and_then(|k| catalogs().get(*code).and_then(|c| c.get(k)));
        toks.push(match tr {
            Some(t) => replace_first_literal(args[1].clone(), t),
            None => args[1].clone(),
        });
    }
    // match crate::i18n::current() { Lang::X => <tokens>, … }
    let skeleton: TokenStream = format!("match crate::i18n::current() {{ {arms} }}").parse().unwrap();
    substitute(skeleton, &toks)
}

/// Swap each `__A<i>` identifier for the tokens of form `i` (in a group, so
/// the form stays one expression).
fn substitute(ts: TokenStream, forms: &[TokenStream]) -> TokenStream {
    ts.into_iter()
        .map(|tt| match tt {
            TokenTree::Ident(id) if id.to_string().starts_with("__A") => {
                let i: usize = id.to_string()[3..].parse().unwrap();
                TokenTree::Group(Group::new(Delimiter::Parenthesis, forms[i].clone()))
            }
            TokenTree::Group(g) => {
                let mut ng = Group::new(g.delimiter(), substitute(g.stream(), forms));
                ng.set_span(g.span());
                TokenTree::Group(ng)
            }
            other => other,
        })
        .collect()
}

fn split_args(input: TokenStream) -> Vec<TokenStream> {
    let mut out = vec![Vec::new()];
    for tt in input {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == ',' => out.push(Vec::new()),
            _ => out.last_mut().unwrap().push(tt),
        }
    }
    out.into_iter().filter(|v| !v.is_empty()).map(|v| v.into_iter().collect()).collect()
}

fn is_str(l: &Literal) -> bool {
    let s = l.to_string();
    s.starts_with('"') || s.starts_with("r\"") || s.starts_with("r#")
}

fn first_literal(ts: &TokenStream) -> Option<Literal> {
    for tt in ts.clone() {
        match tt {
            TokenTree::Literal(l) if is_str(&l) => return Some(l),
            TokenTree::Group(g) => {
                if let Some(l) = first_literal(&g.stream()) {
                    return Some(l);
                }
            }
            _ => {}
        }
    }
    None
}

fn replace_first_literal(ts: TokenStream, text: &str) -> TokenStream {
    let mut done = false;
    fn walk(ts: TokenStream, text: &str, done: &mut bool) -> TokenStream {
        ts.into_iter()
            .map(|tt| {
                if *done {
                    return tt;
                }
                match tt {
                    TokenTree::Literal(l) if is_str(&l) => {
                        *done = true;
                        let mut n = Literal::string(text);
                        n.set_span(l.span());
                        TokenTree::Literal(n)
                    }
                    TokenTree::Group(g) => {
                        let mut ng = Group::new(g.delimiter(), walk(g.stream(), text, done));
                        ng.set_span(g.span());
                        TokenTree::Group(ng)
                    }
                    other => other,
                }
            })
            .collect()
    }
    walk(ts, text, &mut done)
}

/// The value of a string literal as written in the source.
fn unescape(src: &str) -> Option<String> {
    if let Some(rest) = src.strip_prefix('r') {
        let hashes = rest.len() - rest.trim_start_matches('#').len();
        let inner = &rest[hashes + 1..rest.len() - hashes - 1];
        return Some(inner.to_string());
    }
    let body = src.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut it = body.chars().peekable();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next()? {
            '\n' => {
                while matches!(it.peek(), Some(' ' | '\t' | '\n' | '\r')) {
                    it.next();
                }
            }
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            '0' => out.push('\0'),
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            '\'' => out.push('\''),
            'x' => {
                let h: String = [it.next()?, it.next()?].iter().collect();
                out.push(u8::from_str_radix(&h, 16).ok()? as char);
            }
            'u' => {
                it.next()?; // '{'
                let mut h = String::new();
                for c in it.by_ref() {
                    if c == '}' {
                        break;
                    }
                    h.push(c);
                }
                out.push(char::from_u32(u32::from_str_radix(&h, 16).ok()?)?);
            }
            _ => return None,
        }
    }
    Some(out)
}

/// A JSON object of strings (the catalog format), nothing more.
fn parse_object(s: &str) -> Option<HashMap<String, String>> {
    let mut p = s.trim_start_matches('\u{feff}').chars().peekable();
    let mut map = HashMap::new();
    let ws = |p: &mut std::iter::Peekable<std::str::Chars>| {
        while matches!(p.peek(), Some(c) if c.is_whitespace()) {
            p.next();
        }
    };
    fn string(p: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
        if p.next()? != '"' {
            return None;
        }
        let mut out = String::new();
        loop {
            match p.next()? {
                '"' => return Some(out),
                '\\' => match p.next()? {
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    'b' => out.push('\u{8}'),
                    'f' => out.push('\u{c}'),
                    'u' => {
                        let h: String = (0..4).filter_map(|_| p.next()).collect();
                        let mut cp = u32::from_str_radix(&h, 16).ok()?;
                        if (0xD800..0xDC00).contains(&cp) {
                            p.next();
                            p.next();
                            let h2: String = (0..4).filter_map(|_| p.next()).collect();
                            let lo = u32::from_str_radix(&h2, 16).ok()?;
                            cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                        }
                        out.push(char::from_u32(cp)?);
                    }
                    c => out.push(c),
                },
                c => out.push(c),
            }
        }
    }
    ws(&mut p);
    if p.next()? != '{' {
        return None;
    }
    loop {
        ws(&mut p);
        match p.peek()? {
            '}' => return Some(map),
            ',' => {
                p.next();
                continue;
            }
            _ => {}
        }
        let k = string(&mut p)?;
        ws(&mut p);
        if p.next()? != ':' {
            return None;
        }
        ws(&mut p);
        let v = string(&mut p)?;
        map.insert(k, v);
    }
}

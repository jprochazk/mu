use diag::Source;
use indoc::indoc;
use peg::error::{ExpectedSet, ParseError};
use span::Span;

use super::*;

fn relevant_error(set: &ExpectedSet) -> Option<&str> {
  set.tokens().find(|&token| token == "invalid indentation")
}

fn report(source: &str, err: ParseError<Span>) -> String {
  let message = if let Some(e) = relevant_error(&err.expected) {
    e.to_string()
  } else {
    format!("{}", err.expected)
  };

  let report = diag::Report::error()
    .source(Source::string(source))
    .message(message)
    .span(err.location)
    .build()
    .unwrap();
  let mut buf = String::new();
  report.emit(&mut buf).unwrap();
  buf
}

fn print_tokens(lex: &Lexer) {
  for token in lex.debug_tokens() {
    println!("{token:?}");
  }
  // let tokens = lex.debug_tokens().collect::<Vec<_>>();
  // insta::assert_debug_snapshot!(tokens);
}

macro_rules! check_module {
  ($input:literal) => {check_module!(__inner $input, false)};
  (? $input:literal) => {check_module!(__inner $input, true)};
  (__inner $input:literal , $print_tokens:expr) => {{
    let input = indoc!($input);
    let lex = Lexer::lex(input).unwrap();
    if $print_tokens { print_tokens(&lex); }
    match parse(&lex) {
      Ok(module) => insta::assert_debug_snapshot!(module),
      Err(e) => {
        eprintln!("{}", report(input, e));
        panic!("Failed to parse source, see errors above.")
      }
    };
  }};
}

macro_rules! check_expr {
  ($input:literal) => {check_expr!(__inner $input, false)};
  (? $input:literal) => {check_expr!(__inner $input, true)};
  (__inner $input:literal , $print_tokens:expr) => {{
    let input = indoc!($input);
    let lex = Lexer::lex(input).unwrap();
    if $print_tokens { print_tokens(&lex); }
    match grammar::expr(&lex, &StateRef::new(&lex)) {
      Ok(module) => insta::assert_debug_snapshot!(module),
      Err(e) => {
        eprintln!("{}", report(input, e));
        panic!("Failed to parse source, see errors above.")
      }
    };
  }};
}

macro_rules! check_error {
  ($input:literal) => {check_error!(__inner $input, false)};
  (? $input:literal) => {check_error!(__inner $input, true)};
  (__inner $input:literal , $print_tokens:expr) => {{
    let input = indoc!($input);
    let lex = Lexer::lex(input).unwrap();
    if $print_tokens { print_tokens(&lex); }
    match parse(&lex) {
      Ok(_) => panic!("module parsed successfully"),
      Err(e) => insta::assert_snapshot!(report(input, e)),
    };
  }};
}

#[test]
fn test_import_path() {
  check_module! {
    r#"
      use a
      use a.b
      use a.b.c
      use a.{b, c}
      use a.{b.{c}, d.{e}}
      use {a.{b}, c.{d}}
      use {a, b, c,}
    "#
  };

  check_module! {
    r#"
      use a as x
      use a.b as x
      use a.b.c as x
      use a.{b as x, c as y}
      use a.{b.{c as x}, d.{e as y}}
      use {a.{b as x}, c.{d as y}}
      use {a as x, b as y, c as z,}
    "#
  };

  check_error! {
    r#"
      use a
        use b
    "#
  };
}

#[test]
fn binary_expr() {
  check_expr!(r#"a + b"#);
  check_expr!(r#"a - b"#);
  check_expr!(r#"a / b"#);
  check_expr!(r#"a ** b"#);
  check_expr!(r#"a * b"#);
  check_expr!(r#"a % b"#);
  check_expr!(r#"a == b"#);
  check_expr!(r#"a != b"#);
  check_expr!(r#"a > b"#);
  check_expr!(r#"a >= b"#);
  check_expr!(r#"a < b"#);
  check_expr!(r#"a <= b"#);
  check_expr!(r#"a && b"#);
  check_expr!(r#"a || b"#);
  check_expr!(r#"a ?? b"#);

  check_module! {
    r#"
      a + b
      c + d
    "#
  };

  check_error! {
    r#"
      a +
        b
    "#
  }

  check_error! {
    r#"
      a
      + b
    "#
  }
}

#[test]
fn unary_expr() {
  // check_expr!(r#"+a"#);
  check_expr!(r#"-a"#);
  check_expr!(r#"!a"#);
}

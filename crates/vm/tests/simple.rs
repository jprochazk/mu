#[path = "common/common.rs"]
#[macro_use]
mod common;

check!(add, r#"2 + 2"#);
check!(sub, r#"2 - 2"#);
check!(mul, r#"2 * 2"#);
check!(div, r#"2 / 2"#);
check!(pow, r#"2 ** 2"#);
check!(rem, r#"2 % 2"#);
check!(cmp_eq, r#"2 == 2"#);
check!(cmp_neq, r#"2 != 2"#);
check!(cmp_gt, r#"2 > 2"#);
check!(cmp_ge, r#"2 >= 2"#);
check!(cmp_lt, r#"2 < 2"#);
check!(cmp_le, r#"2 <= 2"#);
check!(true_and_false, r#"true && false"#);
check!(true_or_false, r#"true || false"#);
check!(opt_none, r#"none ?? 2"#);
check!(opt_val, r#"2 ?? none"#);
check!(plus_n, r#"+2"#);
check!(minus_n, r#"-2"#);
check!(not_true, r#"!true"#);
check!(not_false, r#"!false"#);

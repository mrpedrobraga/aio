#[test]
fn test_parse_int() {
    use crate::{parse::integer_literal, syntax::IntegerLiteral};
    use chumsky::Parser;

    let src = "10_000";

    assert_eq!(
        dbg!(integer_literal().parse(src).output()),
        Some(&IntegerLiteral {
            value: 10000,
            sign: true,
            bits: None
        })
    )
}

#[test]
fn test_general() {
    use crate::parse::expression;
    use chumsky::Parser;

    use ariadne::{Color, Label, Report, ReportKind, Source};

    let src = "a";
    let (res, errs) = expression().parse(src).into_output_errors();

    dbg!(res);

    errs.into_iter().for_each(|e| {
        Report::build(ReportKind::Error, e.span().into_range())
            .with_message(e.to_string())
            .with_label(
                Label::new(e.span().into_range())
                    .with_message(e.into_reason())
                    .with_color(Color::Red),
            )
            .finish()
            .eprint(Source::from(src))
            .unwrap();
    });
}

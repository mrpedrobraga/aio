use {
    crate::syntax::*,
    chumsky::{
        IterParser, Parser, extra,
        pratt::infix,
        prelude::{any, choice, just, via_parser},
        span::SimpleSpan,
        text,
    },
};

mod tests;

pub fn expression<'a>()
-> impl Parser<'a, &'a str, Spanned<Expression>, extra::Err<chumsky::error::Rich<'a, char>>> {
    let v_l_expression = l_expression()
        .map(Expression::LExpression)
        .spanned()
        .map(Spanned::from)
        .labelled("expression");

    binary_operation().or(v_l_expression)
}

pub fn l_expression<'a>()
-> impl Parser<'a, &'a str, LExpression, extra::Err<chumsky::error::Rich<'a, char>>> + Clone {
    ident().map(LExpression::SymbolRef)
}

pub fn binary_operation<'a>()
-> impl Parser<'a, &'a str, Spanned<Expression>, extra::Err<chumsky::error::Rich<'a, char>>> + Clone
{
    let plus_op = just("+")
        .padded()
        .to(BuiltinOperator::Plus)
        .spanned()
        .map(Spanned::from);

    let terminator = choice((plus_op,)).ignored();
    let atom_recovery = any()
        .and_is(terminator.not())
        .repeated()
        .at_least(1)
        .to(Expression::LExpression(LExpression::Error));

    let atom = l_expression()
        .map(Expression::LExpression)
        .labelled("expression")
        .recover_with(via_parser(atom_recovery))
        .spanned()
        .map(Spanned::from);

    atom.pratt((infix(
        chumsky::pratt::left(1),
        plus_op,
        |left: Spanned<Expression>,
         op: Spanned<BuiltinOperator>,
         right: Spanned<Expression>,
         extra| {
            let span: SimpleSpan = extra.span();

            Spanned {
                node: Expression::BinaryOp(Box::new(BinaryOperation { op, left, right })),
                span: span.into(),
            }
        },
    ),))
}

pub fn ident<'a>()
-> impl Parser<'a, &'a str, Identifier, extra::Err<chumsky::error::Rich<'a, char>>> + Clone {
    text::ident().map(|s| Identifier(std::sync::Arc::from(s)))
}

pub fn integer_literal<'a>()
-> impl Parser<'a, &'a str, IntegerLiteral, extra::Err<chumsky::error::Rich<'a, char>>> + Clone {
    let sign = just('-').or(just('+')).or_not();

    let digits = any()
        .filter(|c: &char| c.is_ascii_digit() || *c == '_')
        .repeated()
        .at_least(1)
        .collect::<String>();

    sign.then(digits).map(|(sign, s)| {
        let stripped = s.replace('_', "");
        IntegerLiteral {
            value: stripped.parse::<u64>().unwrap(),
            sign: sign.is_none_or(|e| e != '-'),
            bits: None,
        }
    })
}

pub mod implementations {
    use crate::syntax::{Span, Spanned};
    use chumsky::span::SimpleSpan;

    impl<T> From<chumsky::prelude::Spanned<T, SimpleSpan>> for Spanned<T> {
        fn from(value: chumsky::prelude::Spanned<T, SimpleSpan>) -> Self {
            Spanned {
                node: value.inner,
                span: Span {
                    start: value.span.start,
                    end: value.span.end,
                },
            }
        }
    }

    impl From<SimpleSpan> for Span {
        fn from(value: SimpleSpan) -> Self {
            Span {
                start: value.start,
                end: value.end,
            }
        }
    }
}

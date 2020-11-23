use std::error::Error;

use boolean_expression::Expr;

struct SPDXExpression {
    expression: Expr<String>,
}

impl SPDXExpression {
    pub fn try_from_string(expression_string: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let expression = parser::parse_spdx(&expression_string);
        match expression {
            Ok(expression) => Ok(SPDXExpression {
                expression: expression.1,
            }),
            Err(error) => Err(error.into()),
        }
    }
}

#[cfg(test)]
mod test_spdx_expression {
    use super::*;

    #[test]
    fn simple() {
       let expression = SPDXExpression::try_from_string("MIT").unwrap();
       assert_eq!(expression.expression, Expr::Terminal("MIT".to_string()))
    }
}

mod parser {
    use boolean_expression::Expr;

    use nom::{
        branch::alt, bytes::complete::tag, bytes::complete::take_while, sequence::separated_pair,
        IResult,
    };

    /// Test if character is legal in SPDX license identifier.
    fn is_spdx_char(c: char) -> bool {
        c.is_alphanumeric() || c == '.' || c == '-' || c == '+'
    }

    /// Parse SPDX license expression.
    fn parse_simple_expression(i: &str) -> IResult<&str, Expr<String>> {
        let (i, expression) = take_while(is_spdx_char)(i)?;
        let expression = Expr::Terminal(expression.into());
        Ok((i, expression))
    }

    /// Parse SPDX license expression.
    fn parse_expression(i: &str) -> IResult<&str, Expr<String>> {
        let (i, expression) = alt((parse_with, parse_simple_expression))(i)?;
        Ok((i, expression))
    }
    /// Parse SPDX idstring
    fn parse_idstring(i: &str) -> IResult<&str, String> {
        let (i, expression) = take_while(is_spdx_char)(i)?;
        Ok((i, expression.into()))
    }
    /// Parse OR expression.
    fn parse_or(i: &str) -> IResult<&str, Expr<String>> {
        let (i, (left, right)) = separated_pair(
            alt((parse_and, parse_expression)),
            tag(" OR "),
            alt((parse_or, parse_and, parse_expression)),
        )(i)?;
        let expression = Expr::or(left, right);
        Ok((i, expression))
    }

    /// Parse AND expression.
    fn parse_and(i: &str) -> IResult<&str, Expr<String>> {
        let (i, (left, right)) = separated_pair(
            parse_expression,
            tag(" AND "),
            alt((parse_and, parse_expression)),
        )(i)?;
        let expression = Expr::and(left, right);
        Ok((i, expression))
    }

    /// Parse WITH expression.
    fn parse_with(i: &str) -> IResult<&str, Expr<String>> {
        let (i, (left, right)) = separated_pair(parse_idstring, tag(" WITH "), parse_idstring)(i)?;
        let expression = Expr::Terminal(format!("{} WITH {}", left, right));
        Ok((i, expression))
    }
    /// Combine the parsers.
    pub(crate) fn parse_spdx(i: &str) -> IResult<&str, Expr<String>> {
        alt((parse_or, parse_and, parse_expression))(i)
    }

    #[cfg(test)]
    mod test_spdx_parser {
        use std::collections::HashMap;

        use super::*;

        #[test]
        fn test_parse_expression() {
            let (_, output) = parse_expression("MIT").unwrap();
            let expected = Expr::Terminal("MIT".to_string());
            assert_eq!(output, expected);

            let (_, output) = parse_expression("GPL-3.0-or-later").unwrap();
            let expected = Expr::Terminal("GPL-3.0-or-later".to_string());
            assert_eq!(output, expected);
        }

        #[test]
        fn test_simple() {
            let expected = Expr::Terminal("MIT".to_string());
            let (_, output) = parse_spdx("MIT").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_with() {
            let expected = Expr::Terminal("GPL-2.0-only WITH Classpath-exception-2.0".to_string());
            let (_, output) = parse_spdx("GPL-2.0-only WITH Classpath-exception-2.0").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_or() {
            let left = Expr::Terminal("MIT".to_string());
            let right = Expr::Terminal("GPL-2.0-only".to_string());
            let expected = Expr::or(left, right);
            let (_, output) = parse_spdx("MIT OR GPL-2.0-only").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_and() {
            let left = Expr::Terminal("MIT".to_string());
            let right = Expr::Terminal("GPL-2.0-only".to_string());
            let expected = Expr::and(left, right);
            let (_, output) = parse_spdx("MIT AND GPL-2.0-only").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_and_with() {
            let left = Expr::Terminal("GPL-2.0-only WITH Classpath-exception-2.0".to_string());
            let right = Expr::Terminal("MIT".to_string());
            let expected = Expr::and(left, right);
            let (_, output) =
                parse_spdx("GPL-2.0-only WITH Classpath-exception-2.0 AND MIT").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_and_with_three() {
            let left = Expr::Terminal("MIT".to_string());
            let right_left = Expr::Terminal("GPL-2.0-only".to_string());
            let right_right = Expr::Terminal("BSD-3-Clause".to_string());
            let right = Expr::and(right_left, right_right);
            let expected = Expr::and(left, right);
            let (_, output) = parse_spdx("MIT AND GPL-2.0-only AND BSD-3-Clause").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_or_with_three() {
            let left = Expr::Terminal("MIT".to_string());
            let right_left = Expr::Terminal("GPL-2.0-only".to_string());
            let right_right = Expr::Terminal("BSD-3-Clause".to_string());
            let right = Expr::or(right_left, right_right);
            let expected = Expr::or(left, right);
            let (_, output) = parse_spdx("MIT OR GPL-2.0-only OR BSD-3-Clause").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_or_with_three_and() {
            let left = Expr::Terminal("MIT".to_string());
            let right_left = Expr::Terminal("GPL-2.0-only".to_string());
            let right_right = Expr::Terminal("BSD-3-Clause".to_string());
            let right = Expr::and(right_left, right_right);
            let expected = Expr::or(left, right);
            let (_, output) = parse_spdx("MIT OR GPL-2.0-only AND BSD-3-Clause").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_and_or_and() {
            let left_left = Expr::Terminal("MIT".to_string());
            let left_right = Expr::Terminal("ISC".to_string());
            let right_left = Expr::Terminal("GPL-2.0-only".to_string());
            let right_right = Expr::Terminal("BSD-3-Clause".to_string());
            let left = Expr::and(left_left, left_right);
            let right = Expr::and(right_left, right_right);
            let expected = Expr::or(left, right);
            let (_, output) = parse_spdx("MIT AND ISC OR GPL-2.0-only AND BSD-3-Clause").unwrap();
            assert_eq!(output, expected);
        }

        #[test]
        fn test_evaluate() {
            let (_, output) = parse_spdx("MIT AND ISC OR GPL-2.0-only AND BSD-3-Clause").unwrap();
            let mut policy = HashMap::new();
            policy.insert("MIT".into(), true);
            assert_eq!(output.evaluate(&policy), false);
            policy.insert("ISC".into(), true);
            assert_eq!(output.evaluate(&policy), true);
            let mut policy = HashMap::new();
            policy.insert("GPL-2.0-only".into(), true);
            assert_eq!(output.evaluate(&policy), false);
            policy.insert("BSD-3-Clause".into(), true);
            assert_eq!(output.evaluate(&policy), true);
        }
    }
}

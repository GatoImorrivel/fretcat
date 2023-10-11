use crate::{CustomParseError, Parse};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MathFunction<V> {
    Calc(Calc<V>),
    Min(Vec<Calc<V>>),
    Max(Vec<Calc<V>>),
    Clamp(Calc<V>, Calc<V>, Calc<V>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Calc<V> {
    Value(Box<V>),
    Number(f32),
    Sum(Box<Calc<V>>, Box<Calc<V>>),
    Product(f32, Box<Calc<V>>),
    Function(Box<MathFunction<V>>),
}

impl<
        'i,
        V: Parse<'i>
            + std::ops::Mul<f32, Output = V>
            + std::ops::Add<V, Output = V>
            + std::cmp::PartialOrd<V>
            + std::convert::Into<Calc<V>>
            + std::convert::From<Calc<V>>
            + std::fmt::Debug,
    > Parse<'i> for Calc<V>
{
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let f = input.expect_function()?;
        match_ignore_ascii_case! { f,
            "calc" => {
                let calc = input.parse_nested_block(Calc::parse_sum)?;
                match calc {
                    Calc::Value(_) | Calc::Number(_) => Ok(calc),
                    _ => Ok(Calc::Function(Box::new(MathFunction::Calc(calc))))
                }
            },
            "min" => {
                let mut args = input.parse_nested_block(|input| input.parse_comma_separated(Calc::parse_sum))?;
                let mut reduced = Calc::reduce_args(&mut args, std::cmp::Ordering::Less);
                if reduced.len() == 1 {
                    return Ok(reduced.remove(0))
                }
                 Ok(Calc::Function(Box::new(MathFunction::Min(reduced))))
            },
            "max" => {
                let mut args = input.parse_nested_block(|input| input.parse_comma_separated(Calc::parse_sum))?;
                let mut reduced = Calc::reduce_args(&mut args, std::cmp::Ordering::Greater);
                if reduced.len() == 1 {
                return Ok(reduced.remove(0))
                }
                Ok(Calc::Function(Box::new(MathFunction::Max(reduced))))
            },
            "clamp" => {
                let (mut min, mut center, mut max) = input.parse_nested_block(|input| {
                let min = Some(Calc::parse_sum(input)?);
                input.expect_comma()?;
                let center: Calc<V> = Calc::parse_sum(input)?;
                input.expect_comma()?;
                let max = Some(Calc::parse_sum(input)?);
                Ok((min, center, max))
                })?;

                // According to the spec, the minimum should "win" over the maximum if they are in the wrong order.
                let cmp = if let (Some(Calc::Value(max_val)), Calc::Value(center_val)) = (&max, &center) {
                    center_val.partial_cmp(max_val)
                } else {
                    None
                };

                // If center is known to be greater than the maximum, replace it with maximum and remove the max argument.
                // Otherwise, if center is known to be less than the maximum, remove the max argument.
                match cmp {
                    Some(std::cmp::Ordering::Greater) => {
                        center = std::mem::take(&mut max).unwrap();
                    }
                    Some(_) => {
                        max = None;
                    }
                    None => {}
                }

                let cmp = if let (Some(Calc::Value(min_val)), Calc::Value(center_val)) = (&min, &center) {
                    center_val.partial_cmp(min_val)
                } else {
                    None
                };

                // If center is known to be less than the minimum, replace it with minimum and remove the min argument.
                // Otherwise, if center is known to be greater than the minimum, remove the min argument.
                match cmp {
                    Some(std::cmp::Ordering::Less) => {
                        center = std::mem::take(&mut min).unwrap();
                    }
                    Some(_) => {
                        min = None;
                    }
                    None => {}
                }

                // Generate clamp(), min(), max(), or value depending on which arguments are left.
                match (min, max) {
                    (None, None) => Ok(center),
                    (Some(min), None) => Ok(Calc::Function(Box::new(MathFunction::Max(vec![min, center])))),
                    (None, Some(max)) => Ok(Calc::Function(Box::new(MathFunction::Min(vec![center, max])))),
                    (Some(min), Some(max)) => Ok(Calc::Function(Box::new(MathFunction::Clamp(min, center, max))))
                }
            },
            _ => Err(location.new_unexpected_token_error(Token::Ident(f.clone()))),
        }
    }
}

impl<
        'i,
        V: Parse<'i>
            + std::ops::Mul<f32, Output = V>
            + std::ops::Add<V, Output = V>
            + std::cmp::PartialOrd<V>
            + std::convert::Into<Calc<V>>
            + std::convert::From<Calc<V>>
            + std::fmt::Debug,
    > Calc<V>
{
    fn parse_sum<'t>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut cur: Calc<V> = Calc::parse_product(input)?;
        loop {
            let start = input.state();
            match input.next_including_whitespace() {
                Ok(&Token::WhiteSpace(_)) => {
                    if input.is_exhausted() {
                        break; // allow trailing whitespace
                    }
                    match *input.next()? {
                        Token::Delim('+') => {
                            let next = Calc::parse_product(input)?;
                            cur = cur + next;
                        }
                        Token::Delim('-') => {
                            let mut rhs = Calc::parse_product(input)?;
                            rhs = rhs * -1.0;
                            cur = cur + rhs;
                        }
                        ref t => {
                            let t = t.clone();
                            return Err(input.new_unexpected_token_error(t));
                        }
                    }
                }
                _ => {
                    input.reset(&start);
                    break;
                }
            }
        }
        Ok(cur)
    }

    fn parse_product<'t>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut node = Calc::parse_value(input)?;
        loop {
            let start = input.state();
            match input.next() {
                Ok(&Token::Delim('*')) => {
                    // At least one of the operands must be a number.
                    let rhs = Self::parse_value(input)?;
                    if let Calc::Number(val) = rhs {
                        node = node * val;
                    } else if let Calc::Number(val) = node {
                        node = rhs;
                        node = node * val;
                    } else {
                        return Err(input.new_unexpected_token_error(Token::Delim('*')));
                    }
                }
                Ok(&Token::Delim('/')) => {
                    let rhs = Self::parse_value(input)?;
                    if let Calc::Number(val) = rhs {
                        if val != 0.0 {
                            node = node * (1.0 / val);
                            continue;
                        }
                    }
                    return Err(input.new_custom_error(CustomParseError::InvalidValue));
                }
                _ => {
                    input.reset(&start);
                    break;
                }
            }
        }
        Ok(node)
    }

    fn parse_value<'t>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        // Parse nested calc() and other math functions.
        if let Ok(calc) = input.try_parse(Self::parse) {
            match calc {
                Calc::Function(f) => {
                    return Ok(match *f {
                        MathFunction::Calc(c) => c,
                        _ => Calc::Function(f),
                    })
                }
                c => return Ok(c),
            }
        }

        if input.try_parse(|input| input.expect_parenthesis_block()).is_ok() {
            return input.parse_nested_block(Calc::parse_sum);
        }

        if let Ok(num) = input.try_parse(|input| input.expect_number()) {
            return Ok(Calc::Number(num));
        }

        if let Ok(value) = input.try_parse(V::parse) {
            return Ok(Calc::Value(Box::new(value)));
        }

        Err(input.new_error_for_next_token())
    }

    fn reduce_args(args: &mut Vec<Calc<V>>, cmp: std::cmp::Ordering) -> Vec<Calc<V>> {
        // Reduces the arguments of a min() or max() expression, combining compatible values.
        // e.g. min(1px, 1em, 2px, 3in) => min(1px, 1em)
        let mut reduced: Vec<Calc<V>> = vec![];
        for arg in args.drain(..) {
            let mut found = None;
            if let Calc::Value(val) = &arg {
                for b in reduced.iter_mut() {
                    if let Calc::Value(v) = b {
                        match val.partial_cmp(v) {
                            Some(ord) if ord == cmp => {
                                found = Some(Some(b));
                                break;
                            }
                            Some(_) => {
                                found = Some(None);
                                break;
                            }
                            None => {}
                        }
                    }
                }
            }
            if let Some(r) = found {
                if let Some(r) = r {
                    *r = arg
                }
            } else {
                reduced.push(arg)
            }
        }
        reduced
    }
}

impl<V: std::ops::Mul<f32, Output = V>> std::ops::Mul<f32> for Calc<V> {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        if other == 1.0 {
            return self;
        }

        match self {
            Calc::Value(v) => Calc::Value(Box::new(*v * other)),
            Calc::Number(n) => Calc::Number(n * other),
            Calc::Sum(a, b) => Calc::Sum(Box::new(*a * other), Box::new(*b * other)),
            Calc::Product(num, calc) => {
                let num = num * other;
                if num == 1.0 {
                    return *calc;
                }
                Calc::Product(num, calc)
            }
            Calc::Function(f) => match *f {
                MathFunction::Calc(c) => Calc::Function(Box::new(MathFunction::Calc(c * other))),
                _ => Calc::Product(other, Box::new(Calc::Function(f))),
            },
        }
    }
}

impl<
        V: std::ops::Add<V, Output = V>
            + std::convert::Into<Calc<V>>
            + std::convert::From<Calc<V>>
            + std::fmt::Debug,
    > std::ops::Add<Calc<V>> for Calc<V>
{
    type Output = Self;

    fn add(self, other: Calc<V>) -> Calc<V> {
        match (self, other) {
            (Calc::Value(a), Calc::Value(b)) => (*a + *b).into(),
            (Calc::Number(a), Calc::Number(b)) => Calc::Number(a + b),
            (Calc::Value(a), b) => (*a + V::from(b)).into(),
            (a, Calc::Value(b)) => (V::from(a) + *b).into(),
            (a, b) => (V::from(a) + V::from(b)).into(),
        }
    }
}

impl<V: std::cmp::PartialEq<f32>> std::cmp::PartialEq<f32> for Calc<V> {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Calc::Value(a) => **a == *other,
            Calc::Number(a) => *a == *other,
            _ => false,
        }
    }
}

impl<V: std::cmp::PartialOrd<f32>> std::cmp::PartialOrd<f32> for Calc<V> {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            Calc::Value(a) => a.partial_cmp(other),
            Calc::Number(a) => a.partial_cmp(other),
            _ => None,
        }
    }
}

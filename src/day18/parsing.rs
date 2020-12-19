use std::{
    fmt::{Display, Formatter},
    mem,
};

use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(u64),
    OpenParen,
    CloseParen,
    PlusSign,
    MulSign,
}

impl Token {
    /// Lex a string into tokens
    fn lex(i: &str) -> Result<Vec<Self>> {
        // Conveniently, all numbers are one char long max
        i.chars()
            .filter_map(|c| match c {
                '(' => Some(Ok(Token::OpenParen)),
                ')' => Some(Ok(Token::CloseParen)),
                '+' => Some(Ok(Token::PlusSign)),
                '*' => Some(Ok(Token::MulSign)),
                ' ' => None,
                // unwrap is ok because we just checked if it's a digit
                c if c.is_ascii_digit() => Some(Ok(Token::Number(c.to_digit(10).unwrap() as u64))),
                oh_no => Some(Err(anyhow!("bad character `{}`", oh_no))),
            })
            .collect()
    }

    /// Is this an operator?
    /// Return Some if it is.
    fn operator(self) -> Option<Operator> {
        match self {
            Token::PlusSign => Some(Operator::Add),
            Token::MulSign => Some(Operator::Multiply),
            _ => None,
        }
    }

    /// Is this a parenthese?
    /// Return `Some(true)` if it opens, `Some(false)` if it closes,
    /// and None otherwise
    fn paren(self) -> Option<bool> {
        match self {
            Token::OpenParen => Some(true),
            Token::CloseParen => Some(false),
            _ => None,
        }
    }
}

/// A term in an equation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// No value.
    /// If this is returned this is an error.
    Nothing,

    /// Just a literal value
    Literal(u64),
    /// A left-hand side and a right-hand side.
    Compound(Box<Term>, Operator, Box<Term>),
    /// Signal that we're ending a parentheses block
    EndParen,
}

impl Term {
    /// Parse a stream of tokens into a cons list of terms, with no precedence
    fn parse_communist(tokens: Vec<Token>) -> Result<Self> {
        /*
            2 + 3 => Term(2, +, 3)
            2 * 3 + 4 => Term(Term(2, *, 3), 4)
            (1 + 2) + (7 * 8) => Term(Term(1, +, 2), +, Term(7 * 8))

            ---
            2 * 3 + 4 * (5 + 6)
            > current = 2

            * 3 + 4 * (5 + 6)
            > current = 2 * _

            3 + 4 * (5 + 6)
            > current = 2 * 3

            + 4 * (5 + 6)
            > current = (2 * 3) + _

            4 * (5 + 6)
            > current = (2 * 3) + 4

            * (5 + 6)
            > current = (2 * 3) + (4 * _)

            (5 + 6)
            > push a new _ onto the stack
            > stack1 = _
            > current = (2 * 3) + (4 * _)

            5 + 6)
            > stack1 = 5

            + 6)
            > stack1 = 5 + _

            6)
            > stack1 = 5 + 6

            )
            > pop off the stack
            > current = (2 * 3) + (4 * (5 + 6))
            ---
        */

        let mut stack = vec![Term::Nothing];

        for token in tokens {
            let current = stack.last_mut().unwrap();

            if let Token::Number(num) = token {
                match current {
                    Term::Nothing => *current = Term::Literal(num),
                    Term::Compound(_lhs, _oper, ref mut rhs @ box Term::Nothing) => {
                        // fill the blank right side
                        *rhs = Box::new(Term::Literal(num))
                    }
                    Term::Compound(..) | Term::EndParen => {
                        bail!("tried to put a compound before a literal")
                    }
                    Term::Literal(..) => bail!("tried to put two literals together"),
                }
            } else if let Some(oper) = token.operator() {
                // make sure we're not trying to rub two operators together
                if let Term::Compound(_lhs, _oper, box Term::Nothing) = current {
                    bail!("tried to put two operators together")
                }
                let prev_current = current.clone();
                // huh `box` was stabilized? til
                *current = Term::Compound(Box::new(prev_current), oper, Box::new(Term::Nothing));
            } else if let Some(opens) = token.paren() {
                if opens {
                    // push a new blank operator onto the stack
                    stack.push(Term::Nothing);
                } else {
                    // pop our finished subexpr off the stack
                    let finished = stack.pop().unwrap();
                    // and assign it to the new top
                    let current = stack
                        .last_mut()
                        .ok_or_else(|| anyhow!("too many close parentheses"))?;
                    match current {
                        Term::Nothing => *current = finished,
                        Term::Compound(_lhs, _oper, ref mut rhs @ box Term::Nothing) => {
                            *rhs = Box::new(finished)
                        }
                        Term::Compound(..) | Term::EndParen => {
                            bail!("tried to put two compounds together")
                        }
                        Term::Literal(..) => bail!("tried to put a literal before a compound"),
                    }
                }
            }
        }

        let out = stack
            .pop()
            .ok_or_else(|| anyhow!("didn't have anything to pop off at the end"))?;
        if !stack.is_empty() {
            bail!("not enough close parentheses");
        }
        Ok(out)
    }

    /// Parse a stream of tokens into a cons list of terms, with addition reigning over multiplication
    fn parse_precedentful(tokens: Vec<Token>) -> Result<Self> {
        /*
            2 * 3 + 4 + 5 * 6
            > 2

            * 3 + 4 + 5 * 6
            > 2 * _

            3 + 4 + 5 * 6
            > (2 * 3)

            + 4 + 5 * 6
            > push
            > 1: (3 + _)
            > 0: (2 * _)

            4 + 5 * 6
            > 1: (3 + 4)
            > 0: (2 * _)

            + 5 * 6
            > push
            > 2: (4 + _)
            > 1: (3 + _)
            > 0: (2 * _)

            5 * 6
            > 2: (4 + 5)
            > 1: (3 + _)
            > 0: (2 * _)

            * 6
            > * this is not a + so we pop one down
            > 1: (3 + (4 + 5))
            > 0: (2 * _)
            > 1 is a +, so we pop one down
            > 0: (2 * (3 + (4 + 5)))
            > and assign * finally
            > 0: ( (2 * (3 + (4 + 5))) * _ )

            > 6
            0: ( (2 * (3 + (4 + 5))) * 6 )


            ---

            2 + 3
            > 2

            > + 3
        */

        let mut stack = vec![Term::Nothing];

        for token in tokens {
            let current = stack.last_mut().unwrap();

            if let Token::Number(num) = token {
                match current {
                    Term::Nothing | Term::EndParen => *current = Term::Literal(num),
                    Term::Compound(_lhs, _oper, ref mut rhs @ box Term::Nothing) => {
                        // fill the blank right side
                        *rhs = Box::new(Term::Literal(num));
                    }
                    Term::Compound(..) => bail!("tried to put a compound before a literal"),
                    Term::Literal(..) => bail!("tried to put two literals together"),
                }
            } else if let Some(oper) = token.operator() {
                // make sure we're not trying to rub two operators together
                if let Term::Compound(_lhs, _oper, box Term::Nothing) = current {
                    bail!("tried to put two operators together")
                }
                if oper == Operator::Multiply {
                    // we need to merge the inner thing into ourselves
                    while let Some(Term::EndParen) = stack.last() {
                        stack.pop();
                    }
                    let current = stack.last_mut().unwrap();
                    let prev_current = current.clone();
                    *current = Term::Compound(
                        Box::new(prev_current),
                        Operator::Multiply,
                        Box::new(Term::Nothing),
                    );
                } else {
                    // we need to push!
                    if let Term::Compound(_, _, ref mut rhs) = current {
                        // strip it out
                        let stolen = mem::replace(rhs, Box::new(Term::Nothing));
                        stack.push(Term::Compound(
                            stolen,
                            Operator::Add,
                            Box::new(Term::Nothing),
                        ));
                    } else if let Term::EndParen = current {
                        // ok, we'll put the next thing into our rhs
                        while let Some(Term::EndParen) = stack.last() {
                            stack.pop();
                        }
                        let current = stack.last_mut().unwrap();
                        let new_lhs = current.clone();
                        *current = Term::Compound(
                            Box::new(new_lhs),
                            Operator::Add,
                            Box::new(Term::Nothing),
                        );
                    } else {
                        // Do the normal thing
                        let prev_current = current.clone();
                        *current =
                            Term::Compound(Box::new(prev_current), oper, Box::new(Term::Nothing));
                    }
                }
            } else if let Some(opens) = token.paren() {
                if opens {
                    // push a new blank operator onto the stack
                    stack.push(Term::Nothing);
                } else {
                    // pop our finished subexpr off the stack
                    let finished = stack.pop().unwrap();
                    // and assign it to the new top
                    match stack.last_mut() {
                        Some(current) => match current {
                            Term::Nothing | Term::EndParen => *current = finished,
                            Term::Compound(_lhs, _oper, ref mut rhs @ box Term::Nothing) => {
                                *rhs = Box::new(finished)
                            }
                            Term::Compound(..) => bail!("tried to put two compounds together"),
                            Term::Literal(..) => bail!("tried to put a literal before a compound"),
                        },
                        None => {
                            // maybe the whole thing was surrounded in parens?
                            // in any case just put it back
                            stack.push(finished);
                        }
                    }
                    // Add a new layer to protect addition from stealing things
                    stack.push(Term::EndParen);
                }
            }
        }

        // We may have many things on the stack; merge them to the right
        while let Some(top) = stack.pop() {
            let next = match stack.last_mut() {
                Some(it) => it,
                None => {
                    // we are too far
                    stack.push(top);
                    break;
                }
            };
            // println!(
            //     "top: {} | next: {}",
            //     top.pretty_print(),
            //     next.pretty_print()
            // );
            if let Term::Compound(_, _, ref mut rhs @ box Term::Nothing) = next {
                *rhs = Box::new(top);
            } else if *next == Term::Nothing || *next == Term::EndParen {
                // Just merge it on down
                *next = top;
            } else if top == Term::Nothing || top == Term::EndParen {
                // do nothing, let it pop off
            } else {
                bail!("something went wrong while merging at the end");
            }
        }
        let out = stack
            .pop()
            .ok_or_else(|| anyhow!("didn't have anything at the end"))?;
        if !stack.is_empty() {
            bail!("not enough close parens");
        }
        Ok(out)
    }

    /// Lex and parse a string into a cons list of Terms with no precedence.
    pub fn lex_parse_communist(input: &str) -> Result<Self> {
        let toks = Token::lex(input).context("while lexing")?;
        Self::parse_communist(toks)
    }

    pub fn lex_parse_precedentful(input: &str) -> Result<Self> {
        let toks = Token::lex(input).context("while lexing")?;
        Self::parse_precedentful(toks)
    }

    /// Evaluate this.
    pub fn evaluate(&self) -> Result<u64> {
        Ok(match self {
            Term::Nothing => bail!("tried to evaluate Nothing"),
            Term::EndParen => bail!("tried to evaluate EndParen"),
            Term::Literal(it) => *it,
            Term::Compound(lhs, oper, rhs) => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                oper.operate(lhs, rhs)
            }
        })
    }

    /// Pretty-print this with parenthese depth included
    pub fn pretty_print(&self) -> String {
        fn inner(term: &Term, depth: usize) -> String {
            const BRACKETS: &[[char; 2]] = &[['(', ')'], ['[', ']'], ['{', '}'], ['<', '>']];
            match term {
                Term::Nothing => "_".to_owned(),
                Term::EndParen => "@".to_owned(), // uh-oh
                Term::Literal(num) => num.to_string(),
                Term::Compound(lhs, oper, rhs) => {
                    if depth == 0 {
                        format!(
                            "{} {} {}",
                            inner(lhs, depth + 1),
                            oper.repr(),
                            inner(rhs, depth + 1),
                        )
                    } else {
                        let [lbracket, rbracket] = BRACKETS[(depth - 1) % BRACKETS.len()];
                        format!(
                            "{}{} {} {}{}",
                            lbracket,
                            inner(lhs, depth + 1),
                            oper.repr(),
                            inner(rhs, depth + 1),
                            rbracket,
                        )
                    }
                }
            }
        }
        inner(self, 0)
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Nothing => f.write_str("_"),
            Term::EndParen => f.write_str("@"),
            Term::Literal(num) => write!(f, "{}", num),
            Term::Compound(lhs, oper, rhs) => write!(f, "({} {} {})", lhs, oper.repr(), rhs),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Multiply,
}

impl Operator {
    /// Operate on two numbers
    pub fn operate(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
        }
    }

    /// Get the repr of this operator
    pub fn repr(self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Multiply => '*',
        }
    }
}

#[test]
fn test_lexing() -> Result<()> {
    let input = "2 * 3 + 4 * (5 + 6)";
    let tokens = Token::lex(input)?;
    println!("{:?}", tokens);

    Ok(())
}

#[test]
fn test_parsing() -> Result<()> {
    let input = "2 * 3 + 4 * (5 + 6)";
    let term = Term::lex_parse_communist(input)?;
    println!("{}", term);

    Ok(())
}

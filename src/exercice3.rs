#[derive(Debug, PartialEq)]
pub enum ComputeError {
    DivisionByZero,
    EmptyStack,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
pub enum Token {
    Number(i32),
    Op(Operator),
}

/// Évalue une expression en Notation Polonaise Inverse.
/// Si l'expression est correcte `compute` renvoie le résultat entier de l'expression de type `Ok(i32)`
/// Si l'expression lève une erreur `compute` renvoie une erreur de type `Err(ComputeError)`
///
/// # Exemple
/// ```
/// use td_exercices_choisis::exercice3::*;
/// let r = compute(&[Token::Number(4), Token::Number(3), Token::Op(Operator::Plus)]);
/// assert_eq!(r, Ok(7));
/// ```
pub fn compute(input: &[Token]) -> Result<i32, ComputeError> {
    panic!("Not implemented!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let r = compute(&[]);
        assert_eq!(r, Err(ComputeError::EmptyStack));
    }
    #[test]
    fn just_a_number() {
        let r = compute(&[Token::Number(5)]);
        assert_eq!(r, Ok(5));
    }

    #[test]
    fn plus() {
        let r = compute(&[
            Token::Number(12),
            Token::Number(-5),
            Token::Op(Operator::Plus),
        ]);
        assert_eq!(r, Ok(7));
    }

    #[test]
    fn minus() {
        let r = compute(&[
            Token::Number(3),
            Token::Number(7),
            Token::Op(Operator::Minus),
        ]);
        assert_eq!(r, Ok(-4));
    }

    #[test]
    fn times() {
        let r = compute(&[
            Token::Number(5),
            Token::Number(7),
            Token::Op(Operator::Times),
        ]);
        assert_eq!(r, Ok(35));
    }

    #[test]
    fn divide_ok() {
        let r = compute(&[
            Token::Number(15),
            Token::Number(2),
            Token::Op(Operator::Divide),
        ]);
        assert_eq!(r, Ok(7));
    }

    #[test]
    fn divide_err() {
        let r = compute(&[
            Token::Number(15),
            Token::Number(0),
            Token::Op(Operator::Divide),
        ]);
        assert_eq!(r, Err(ComputeError::DivisionByZero));
    }

    #[test]
    fn complex_expression() {
        let r = compute(&[
            Token::Number(1),
            Token::Number(2),
            Token::Number(3),
            Token::Number(4),
            Token::Op(Operator::Minus),
            Token::Op(Operator::Times),
            Token::Number(3),
            Token::Op(Operator::Times),
            Token::Op(Operator::Plus),
        ]);
        assert_eq!(r, Ok(-5));
    }

    #[test]
    fn stack_underflow() {
        let r = compute(&[Token::Number(4), Token::Op(Operator::Minus)]);
        assert_eq!(r, Err(ComputeError::EmptyStack));
    }
}

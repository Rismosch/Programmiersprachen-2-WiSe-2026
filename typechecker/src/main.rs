//==============================================================================
// definitions
//==============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Unit,
    Function(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Term {
    Variable(String),
    Abstraction {
        variable: String,
        variable_type: Type,
        term: Box<Term>,
    },
    Application {
        lhs: Box<Term>,
        rhs: Box<Term>,
    },
    Sequence {
        lhs: Box<Term>,
        rhs: Box<Term>,
    },
}

//==============================================================================
// trait impls
//==============================================================================
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit => write!(f, "unit"),
            Self::Function(lhs, rhs) => {
                write!(f, "({} -> {})", lhs, rhs)
            }
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(variable) => write!(f, "{}", variable),
            Self::Abstraction {
                variable,
                variable_type,
                term,
            } => write!(f, "(λ{} : {}.{})", variable, variable_type, term),
            Self::Application { lhs, rhs } => write!(f, "({} {})", lhs, rhs),
            Self::Sequence { lhs, rhs } => write!(f, "({}; {})", lhs, rhs),
        }
    }
}

//==============================================================================
// errors
//==============================================================================
#[derive(Debug)]
pub struct TypeDeriveError(String);

impl std::fmt::Display for TypeDeriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to derive type: {}", self.0)
    }
}

impl std::error::Error for TypeDeriveError {}

impl TypeDeriveError {
    fn new<T>(message: impl AsRef<str>) -> Result<T, Self> {
        Err(Self(message.as_ref().to_string()))
    }
}

//==============================================================================
// typechecker
//==============================================================================
impl Term {
    fn derive_type(
        &self,
        mut type_context: std::collections::HashMap<String, Type>,
    ) -> Result<Type, TypeDeriveError> {
        match self {
            Term::Variable(variable) => match type_context.get(variable) {
                Some(variable_type) => Ok(variable_type.clone()),
                None => TypeDeriveError::new(format!(
                    "expected variable `{}` to be found in context, but was missing",
                    variable,
                )),
            },
            Term::Abstraction {
                variable,
                variable_type,
                term,
            } => {
                let existing_entry = type_context.insert(variable.clone(), variable_type.clone());
                if let Some(existing_type) = existing_entry {
                    return TypeDeriveError::new(format!(
                        "expected variable `{}` to not be found in context, but was already present with type `{}`",
                        variable, existing_type,
                    ));
                }

                let type_in = Box::new(variable_type.clone());
                let type_out = Box::new(term.derive_type(type_context)?);
                Ok(Type::Function(type_in, type_out))
            }
            Term::Application { lhs, rhs } => {
                let type_lhs = lhs.derive_type(type_context.clone())?;
                let type_rhs = rhs.derive_type(type_context.clone())?;

                let Type::Function(type_in, type_out) = type_lhs else {
                    return TypeDeriveError::new(format!(
                        "expected term `{}` to be a function type but was `{}`",
                        lhs, type_lhs,
                    ));
                };

                if *type_in != type_rhs {
                    return TypeDeriveError::new(format!(
                        "expected term `{}` to be of type `{}` but was `{}`",
                        rhs, type_in, type_rhs,
                    ));
                }

                Ok(*type_out)
            }
            Term::Sequence { lhs, rhs } => {
                let type_lhs = lhs.derive_type(type_context.clone())?;
                let type_rhs = rhs.derive_type(type_context.clone())?;

                if type_lhs != Type::Unit {
                    return TypeDeriveError::new(format!(
                        "expected term `{}` to be of type `{}` but was `{}`",
                        lhs,
                        Type::Unit,
                        type_lhs,
                    ));
                }

                Ok(type_rhs)
            }
        }
    }
}

//==============================================================================
// main
//==============================================================================
fn main() {
    // well formed terms
    eprintln!();
    eprintln!("some well formed terms:");
    print_and_derive(
        [("x".to_string(), Type::Unit)].into(),
        Term::Variable("x".to_string()),
    );
    print_and_derive(
        [].into(),
        Term::Abstraction {
            variable: "x".to_string(),
            variable_type: Type::Unit,
            term: Box::new(Term::Variable("x".to_string())),
        },
    );
    print_and_derive(
        [("y".to_string(), Type::Unit)].into(),
        Term::Application {
            lhs: Box::new(Term::Abstraction {
                variable: "x".to_string(),
                variable_type: Type::Unit,
                term: Box::new(Term::Variable("x".to_string())),
            }),
            rhs: Box::new(Term::Variable("y".to_string())),
        },
    );
    print_and_derive(
        [
            (
                "x".to_string(),
                Type::Function(Box::new(Type::Unit), Box::new(Type::Unit)),
            ),
            ("y".to_string(), Type::Unit),
        ]
        .into(),
        Term::Application {
            lhs: Box::new(Term::Variable("x".to_string())),
            rhs: Box::new(Term::Variable("y".to_string())),
        },
    );

    // some malformed terms
    eprintln!();
    eprintln!("some malformed terms:");
    print_and_derive(
        [].into(),
        Term::Abstraction {
            variable: "x".to_string(),
            variable_type: Type::Unit,
            term: Box::new(Term::Variable("y".to_string())),
        },
    );
    print_and_derive(
        [("x".to_string(), Type::Unit)].into(),
        Term::Abstraction {
            variable: "x".to_string(),
            variable_type: Type::Unit,
            term: Box::new(Term::Variable("y".to_string())),
        },
    );
    print_and_derive(
        [("x".to_string(), Type::Unit), ("y".to_string(), Type::Unit)].into(),
        Term::Application {
            lhs: Box::new(Term::Variable("x".to_string())),
            rhs: Box::new(Term::Variable("y".to_string())),
        },
    );
    print_and_derive(
        [
            (
                "x".to_string(),
                Type::Function(Box::new(Type::Unit), Box::new(Type::Unit)),
            ),
            (
                "y".to_string(),
                Type::Function(Box::new(Type::Unit), Box::new(Type::Unit)),
            ),
        ]
        .into(),
        Term::Application {
            lhs: Box::new(Term::Variable("x".to_string())),
            rhs: Box::new(Term::Variable("y".to_string())),
        },
    );

    // sequence extension
    eprintln!();
    eprintln!("sequence extension:");
    print_and_derive(
        [("x".to_string(), Type::Unit), ("y".to_string(), Type::Unit)].into(),
        Term::Sequence {
            lhs: Box::new(Term::Variable("x".to_string())),
            rhs: Box::new(Term::Variable("y".to_string())),
        },
    );
    print_and_derive(
        [
            (
                "x".to_string(),
                Type::Function(Box::new(Type::Unit), Box::new(Type::Unit)),
            ),
            ("y".to_string(), Type::Unit),
        ]
        .into(),
        Term::Sequence {
            lhs: Box::new(Term::Variable("x".to_string())),
            rhs: Box::new(Term::Variable("y".to_string())),
        },
    );
    print_and_derive(
        [("y".to_string(), Type::Unit)].into(),
        Term::Sequence {
            lhs: Box::new(Term::Application {
                lhs: Box::new(Term::Abstraction {
                    variable: "x".to_string(),
                    variable_type: Type::Unit,
                    term: Box::new(Term::Variable("x".to_string())),
                }),
                rhs: Box::new(Term::Variable("y".to_string())),
            }),
            rhs: Box::new(Term::Abstraction {
                variable: "x".to_string(),
                variable_type: Type::Unit,
                term: Box::new(Term::Variable("x".to_string())),
            }),
        },
    );
}

fn print_and_derive(type_context: std::collections::HashMap<String, Type>, term: Term) {
    let type_context_string = type_context
        .iter()
        .map(|(variable, variable_type)| format!("{} : {}", variable, variable_type))
        .collect::<Vec<_>>()
        .join(", ");

    let space = if type_context_string.is_empty() {
        ""
    } else {
        " "
    };

    let type_string = match term.derive_type(type_context) {
        Ok(term_type) => format!("{}", term_type),
        Err(error) => format!("[ERROR {}]", error),
    };

    eprintln!(
        "{}{}⊢ {} : {}",
        type_context_string, space, term, type_string
    )
}

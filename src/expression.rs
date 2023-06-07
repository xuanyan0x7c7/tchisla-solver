use std::fmt;
use std::rc::Rc;

pub enum Expression {
    Number(i64),
    Negate(Rc<Expression>),
    Add(Rc<Expression>, Rc<Expression>),
    Subtract(Rc<Expression>, Rc<Expression>),
    Multiply(Rc<Expression>, Rc<Expression>),
    Divide(Rc<Expression>, Rc<Expression>),
    Power(Rc<Expression>, Rc<Expression>),
    Sqrt(Rc<Expression>, usize),
    Factorial(Rc<Expression>),
}

impl Expression {
    #[inline]
    pub fn to_number(&self) -> Option<i64> {
        match self {
            Expression::Number(x) => Some(*x),
            _ => None,
        }
    }

    #[inline]
    pub fn is_number(&self) -> bool {
        matches!(self, Expression::Number(_))
    }

    #[inline]
    pub fn to_negate(&self) -> Option<&Rc<Expression>> {
        match self {
            Expression::Negate(x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn is_negate(&self) -> bool {
        matches!(self, Expression::Negate(_))
    }

    #[inline]
    pub fn to_add(&self) -> Option<(&Rc<Expression>, &Rc<Expression>)> {
        match self {
            Expression::Add(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn is_add(&self) -> bool {
        matches!(self, Expression::Add(_, _))
    }

    #[inline]
    pub fn to_subtract(&self) -> Option<(&Rc<Expression>, &Rc<Expression>)> {
        match self {
            Expression::Subtract(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn is_subtract(&self) -> bool {
        matches!(self, Expression::Subtract(_, _))
    }

    #[inline]
    pub fn to_multiply(&self) -> Option<(&Rc<Expression>, &Rc<Expression>)> {
        match self {
            Expression::Multiply(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn is_multiply(&self) -> bool {
        matches!(self, Expression::Multiply(_, _))
    }

    #[inline]
    pub fn to_divide(&self) -> Option<(&Rc<Expression>, &Rc<Expression>)> {
        match self {
            Expression::Divide(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn is_divide(&self) -> bool {
        matches!(self, Expression::Divide(_, _))
    }

    #[inline]
    pub fn to_power(&self) -> Option<(&Rc<Expression>, &Rc<Expression>)> {
        match self {
            Expression::Power(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn is_power(&self) -> bool {
        matches!(self, Expression::Power(_, _))
    }

    #[inline]
    pub fn to_sqrt(&self) -> Option<(&Rc<Expression>, &usize)> {
        match self {
            Expression::Sqrt(x, order) => Some((x, order)),
            _ => None,
        }
    }

    #[inline]
    pub fn is_sqrt(&self) -> bool {
        matches!(self, Expression::Sqrt(_, _))
    }

    #[inline]
    pub fn to_factorial(&self) -> Option<&Rc<Expression>> {
        match self {
            Expression::Factorial(x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn is_factorial(&self) -> bool {
        matches!(self, Expression::Factorial(_))
    }

    #[inline]
    fn precedence(&self) -> i32 {
        match self {
            Expression::Number(_) => 6,
            Expression::Negate(_) => 1,
            Expression::Add(_, _) => 2,
            Expression::Subtract(_, _) => 2,
            Expression::Multiply(_, _) => 3,
            Expression::Divide(_, _) => 3,
            Expression::Power(_, _) => 4,
            Expression::Sqrt(_, _) => 5,
            Expression::Factorial(_) => 6,
        }
    }
}

fn fmt_binary(
    f: &mut fmt::Formatter,
    x: &Rc<Expression>,
    y: &Rc<Expression>,
    operator: &str,
    precedence: i32,
    abelian: bool,
    rtl: bool,
) -> fmt::Result {
    let lhs = if x.precedence() < precedence || (x.precedence() == precedence && rtl && !abelian) {
        format!("({x})")
    } else {
        format!("{x}")
    };
    let rhs = if y.precedence() < precedence || (y.precedence() == precedence && !rtl && !abelian) {
        format!("({y})")
    } else {
        format!("{y}")
    };
    write!(f, "{lhs}{operator}{rhs}")
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Number(x) => write!(f, "{x}"),
            Expression::Negate(x) => {
                if x.is_add() || x.is_subtract() {
                    write!(f, "-({x})")
                } else {
                    write!(f, "-{x}")
                }
            }
            Expression::Add(x, y) => fmt_binary(f, x, y, "+", self.precedence(), true, false),
            Expression::Subtract(x, y) => fmt_binary(f, x, y, "-", self.precedence(), false, false),
            Expression::Multiply(x, y) => fmt_binary(f, x, y, "*", self.precedence(), true, false),
            Expression::Divide(x, y) => fmt_binary(f, x, y, "/", self.precedence(), false, false),
            Expression::Power(x, y) => fmt_binary(f, x, y, "^", self.precedence(), false, true),
            Expression::Sqrt(x, order) => {
                write!(f, "{}{x}{}", "sqrt(".repeat(*order), ")".repeat(*order))
            }
            Expression::Factorial(x) => {
                if x.is_number() {
                    write!(f, "{x}!")
                } else {
                    write!(f, "({x})!")
                }
            }
        }
    }
}

fn add_latex_parens(x: String) -> String {
    "\\left(".to_string() + &x + "\\right)"
}

fn fmt_latex_binary(
    x: &Rc<Expression>,
    y: &Rc<Expression>,
    operator: &str,
    precedence: i32,
    abelian: bool,
    rtl: bool,
) -> String {
    let lhs = if x.precedence() < precedence || (x.precedence() == precedence && rtl && !abelian) {
        add_latex_parens(x.to_latex_string())
    } else {
        x.to_latex_string()
    };
    let rhs = if y.precedence() < precedence || (y.precedence() == precedence && !rtl && !abelian) {
        add_latex_parens(y.to_latex_string())
    } else {
        y.to_latex_string()
    };
    lhs + operator + &rhs
}

impl Expression {
    pub fn to_latex_string(&self) -> String {
        match self {
            Expression::Number(x) => x.to_string(),
            Expression::Negate(x) => {
                if x.is_add() || x.is_subtract() {
                    "-".to_string() + &add_latex_parens(x.to_latex_string())
                } else {
                    "-".to_string() + &x.to_latex_string()
                }
            }
            Expression::Add(x, y) => fmt_latex_binary(x, y, "+", self.precedence(), true, false),
            Expression::Subtract(x, y) => {
                fmt_latex_binary(x, y, "-", self.precedence(), false, false)
            }
            Expression::Multiply(x, y) => {
                fmt_latex_binary(x, y, "\\times", self.precedence(), true, false)
            }
            Expression::Divide(x, y) => format!(
                "\\frac{{{}}}{{{}}}",
                x.to_latex_string(),
                y.to_latex_string()
            ),
            Expression::Power(x, y) => format!(
                "{}^{{{}}}",
                if x.is_number() {
                    x.to_latex_string()
                } else {
                    add_latex_parens(x.to_latex_string())
                },
                y.to_latex_string()
            ),
            Expression::Sqrt(x, order) => {
                "\\sqrt{".repeat(*order)
                    + x.to_latex_string().as_str()
                    + "}".repeat(*order).as_str()
            }
            Expression::Factorial(x) => {
                if x.is_number() {
                    x.to_latex_string() + "!"
                } else {
                    add_latex_parens(x.to_latex_string()) + "!"
                }
            }
        }
    }

    pub fn from_number(x: i64) -> Rc<Expression> {
        Rc::new(Expression::Number(x))
    }

    pub fn from_negate(x: Rc<Expression>) -> Rc<Expression> {
        if let Some((y, z)) = x.to_subtract() {
            Rc::new(Expression::Subtract(z.clone(), y.clone()))
        } else {
            Rc::new(Expression::Negate(x))
        }
    }

    pub fn from_add(x: Rc<Expression>, y: Rc<Expression>) -> Rc<Expression> {
        let x0 = x.to_subtract();
        let y0 = y.to_subtract();
        if let (Some(x), Some(y)) = (x0, y0) {
            Rc::new(Expression::Subtract(
                Rc::new(Expression::Add(x.0.clone(), y.0.clone())),
                Rc::new(Expression::Add(x.1.clone(), y.1.clone())),
            ))
        } else if let Some(x) = x0 {
            Rc::new(Expression::Subtract(
                Rc::new(Expression::Add(x.0.clone(), y)),
                x.1.clone(),
            ))
        } else if let Some(y) = y0 {
            Rc::new(Expression::Subtract(
                Rc::new(Expression::Add(x, y.0.clone())),
                y.1.clone(),
            ))
        } else if let Some((y1, y2)) = y.to_add() {
            Rc::new(Expression::Add(
                Expression::from_add(x, y1.clone()),
                y2.clone(),
            ))
        } else {
            Rc::new(Expression::Add(x, y))
        }
    }

    pub fn from_subtract(x: Rc<Expression>, y: Rc<Expression>) -> Rc<Expression> {
        if let Some((y1, y2)) = y.to_subtract() {
            Expression::from_add(x, Rc::new(Expression::Subtract(y2.clone(), y1.clone())))
        } else if let Some((x1, x2)) = x.to_subtract() {
            Rc::new(Expression::Subtract(
                x1.clone(),
                Rc::new(Expression::Add(x2.clone(), y)),
            ))
        } else {
            Rc::new(Expression::Subtract(x, y))
        }
    }

    pub fn from_multiply(x: Rc<Expression>, y: Rc<Expression>) -> Rc<Expression> {
        let x0 = x.to_divide();
        let y0 = y.to_divide();
        if let (Some(x), Some(y)) = (x0, y0) {
            Rc::new(Expression::Divide(
                Rc::new(Expression::Multiply(x.0.clone(), y.0.clone())),
                Rc::new(Expression::Multiply(x.1.clone(), y.1.clone())),
            ))
        } else if let Some(x) = x0 {
            Rc::new(Expression::Divide(
                Rc::new(Expression::Multiply(x.0.clone(), y)),
                x.1.clone(),
            ))
        } else if let Some(y) = y0 {
            Rc::new(Expression::Divide(
                Rc::new(Expression::Multiply(x, y.0.clone())),
                y.1.clone(),
            ))
        } else if let Some((y1, y2)) = y.to_multiply() {
            Rc::new(Expression::Multiply(
                Expression::from_multiply(x, y1.clone()),
                y2.clone(),
            ))
        } else {
            Rc::new(Expression::Multiply(x, y))
        }
    }

    pub fn from_divide(x: Rc<Expression>, y: Rc<Expression>) -> Rc<Expression> {
        if let Some((y1, y2)) = y.to_divide() {
            Expression::from_multiply(x, Rc::new(Expression::Divide(y2.clone(), y1.clone())))
        } else if let Some((x1, x2)) = x.to_divide() {
            Rc::new(Expression::Divide(
                x1.clone(),
                Rc::new(Expression::Multiply(x2.clone(), y)),
            ))
        } else {
            Rc::new(Expression::Divide(x, y))
        }
    }

    pub fn from_power(x: Rc<Expression>, y: Rc<Expression>) -> Rc<Expression> {
        if let Some((x1, x2)) = x.to_power() {
            Rc::new(Expression::Power(
                x1.clone(),
                Expression::from_multiply(x2.clone(), y),
            ))
        } else if let Some((x0, order)) = x.to_sqrt() {
            Rc::new(Expression::Sqrt(
                Expression::from_power(x0.clone(), y),
                *order,
            ))
        } else {
            Rc::new(Expression::Power(x, y))
        }
    }

    pub fn from_sqrt(x: Rc<Expression>, order: usize) -> Rc<Expression> {
        if order == 0 {
            x
        } else if let Some((y, z)) = x.to_sqrt() {
            Rc::new(Expression::Sqrt(y.clone(), z + order))
        } else if let Some((y, z)) = x.to_multiply() {
            Expression::from_multiply(
                Expression::from_sqrt(y.clone(), order),
                Expression::from_sqrt(z.clone(), order),
            )
        } else if let Some((y, z)) = x.to_divide() {
            Expression::from_divide(
                Expression::from_sqrt(y.clone(), order),
                Expression::from_sqrt(z.clone(), order),
            )
        } else {
            Rc::new(Expression::Sqrt(x, order))
        }
    }

    pub fn from_factorial(x: Rc<Expression>) -> Rc<Expression> {
        Rc::new(Expression::Factorial(x))
    }
}

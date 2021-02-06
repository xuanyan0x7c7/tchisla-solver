use crate::number::Number;
use std::fmt;
use std::rc::Rc;

pub enum Expression<T: Number> {
    Number(T),
    Negate(Rc<Expression<T>>),
    Add(Rc<Expression<T>>, Rc<Expression<T>>),
    Subtract(Rc<Expression<T>>, Rc<Expression<T>>),
    Multiply(Rc<Expression<T>>, Rc<Expression<T>>),
    Divide(Rc<Expression<T>>, Rc<Expression<T>>),
    Power(Rc<Expression<T>>, Rc<Expression<T>>),
    Sqrt(Rc<Expression<T>>, usize),
    Factorial(Rc<Expression<T>>),
}

impl<T: Number> Expression<T> {
    #[inline]
    pub fn get_number(&self) -> Option<&T> {
        match self {
            Expression::Number(x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn get_negate(&self) -> Option<&Rc<Expression<T>>> {
        match self {
            Expression::Negate(x) => Some(x),
            _ => None,
        }
    }

    #[inline]
    pub fn get_add(&self) -> Option<(&Rc<Expression<T>>, &Rc<Expression<T>>)> {
        match self {
            Expression::Add(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn get_subtract(&self) -> Option<(&Rc<Expression<T>>, &Rc<Expression<T>>)> {
        match self {
            Expression::Subtract(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn get_multiply(&self) -> Option<(&Rc<Expression<T>>, &Rc<Expression<T>>)> {
        match self {
            Expression::Multiply(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn get_divide(&self) -> Option<(&Rc<Expression<T>>, &Rc<Expression<T>>)> {
        match self {
            Expression::Divide(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn get_power(&self) -> Option<(&Rc<Expression<T>>, &Rc<Expression<T>>)> {
        match self {
            Expression::Power(x, y) => Some((x, y)),
            _ => None,
        }
    }

    #[inline]
    pub fn get_sqrt(&self) -> Option<(&Rc<Expression<T>>, &usize)> {
        match self {
            Expression::Sqrt(x, order) => Some((x, order)),
            _ => None,
        }
    }

    #[inline]
    pub fn get_factorial(&self) -> Option<&Rc<Expression<T>>> {
        match self {
            Expression::Factorial(x) => Some(x),
            _ => None,
        }
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

fn fmt_binary<T: Number>(
    f: &mut fmt::Formatter,
    x: &Rc<Expression<T>>,
    y: &Rc<Expression<T>>,
    operator: &str,
    precedence: i32,
    abelian: bool,
    rtl: bool,
) -> fmt::Result {
    let lhs = if x.precedence() < precedence || (x.precedence() == precedence && rtl && !abelian) {
        format!("({})", x)
    } else {
        format!("{}", x)
    };
    let rhs = if y.precedence() < precedence || (y.precedence() == precedence && !rtl && !abelian) {
        format!("({})", y)
    } else {
        format!("{}", y)
    };
    write!(f, "{}{}{}", lhs, operator, rhs)
}

impl<T: Number> fmt::Display for Expression<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Number(x) => write!(f, "{}", x),
            Expression::Negate(x) => match x.get_add().or(x.get_subtract()) {
                Some(_) => write!(f, "-({})", x),
                None => write!(f, "-{}", x),
            },
            Expression::Add(x, y) => fmt_binary(f, x, y, "+", self.precedence(), true, false),
            Expression::Subtract(x, y) => fmt_binary(f, x, y, "-", self.precedence(), false, false),
            Expression::Multiply(x, y) => fmt_binary(f, x, y, "*", self.precedence(), true, false),
            Expression::Divide(x, y) => fmt_binary(f, x, y, "/", self.precedence(), false, false),
            Expression::Power(x, y) => fmt_binary(f, x, y, "^", self.precedence(), false, true),
            Expression::Sqrt(x, order) => {
                write!(f, "{}{}{}", "sqrt(".repeat(*order), x, ")".repeat(*order))
            }
            Expression::Factorial(x) => match x.get_number() {
                Some(_) => write!(f, "{}!", x),
                None => write!(f, "({})!", x),
            },
        }
    }
}

fn add_parens(x: String) -> String {
    "\\left(".to_string() + &x + "\\right)"
}

fn fmt_latex_binary<T: Number>(
    x: &Rc<Expression<T>>,
    y: &Rc<Expression<T>>,
    operator: &str,
    precedence: i32,
    abelian: bool,
    rtl: bool,
) -> String {
    let lhs = if x.precedence() < precedence || (x.precedence() == precedence && rtl && !abelian) {
        add_parens(x.to_latex_string())
    } else {
        x.to_latex_string()
    };
    let rhs = if y.precedence() < precedence || (y.precedence() == precedence && !rtl && !abelian) {
        add_parens(y.to_latex_string())
    } else {
        y.to_latex_string()
    };
    lhs + operator + &rhs
}

impl<T: Number> Expression<T> {
    pub fn to_latex_string(&self) -> String {
        match self {
            Expression::Number(x) => x.to_string(),
            Expression::Negate(x) => match x.get_add().or(x.get_subtract()) {
                Some(_) => "-".to_string() + &add_parens(x.to_latex_string()),
                None => "-".to_string() + &x.to_latex_string(),
            },
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
                if x.get_number().is_some() {
                    x.to_latex_string()
                } else {
                    add_parens(x.to_latex_string())
                },
                y.to_latex_string()
            ),
            Expression::Sqrt(x, order) => {
                "\\sqrt{".repeat(*order)
                    + x.to_latex_string().as_str()
                    + "}".repeat(*order).as_str()
            }
            Expression::Factorial(x) => match x.get_number() {
                Some(_) => x.to_latex_string() + "!",
                None => add_parens(x.to_latex_string()) + "!",
            },
        }
    }

    pub fn from_number(x: T) -> Rc<Expression<T>> {
        Rc::new(Expression::Number(x))
    }

    pub fn from_negate(x: Rc<Expression<T>>) -> Rc<Expression<T>> {
        if let Some((y, z)) = x.get_subtract() {
            Rc::new(Expression::Subtract(z.clone(), y.clone()))
        } else {
            Rc::new(Expression::Negate(x))
        }
    }

    pub fn from_add(x: Rc<Expression<T>>, y: Rc<Expression<T>>) -> Rc<Expression<T>> {
        let x0 = x.get_subtract();
        let y0 = y.get_subtract();
        if x0.is_some() && y0.is_some() {
            Rc::new(Expression::Subtract(
                Rc::new(Expression::Add(
                    x0.unwrap().0.clone(),
                    y0.unwrap().0.clone(),
                )),
                Rc::new(Expression::Add(
                    x0.unwrap().1.clone(),
                    y0.unwrap().1.clone(),
                )),
            ))
        } else if x0.is_some() {
            Rc::new(Expression::Subtract(
                Rc::new(Expression::Add(x0.unwrap().0.clone(), y)),
                x0.unwrap().1.clone(),
            ))
        } else if y0.is_some() {
            Rc::new(Expression::Subtract(
                Rc::new(Expression::Add(x, y0.unwrap().0.clone())),
                y0.unwrap().1.clone(),
            ))
        } else if let Some((y1, y2)) = y.get_add() {
            Rc::new(Expression::Add(
                Expression::from_add(x, y1.clone()),
                y2.clone(),
            ))
        } else {
            Rc::new(Expression::Add(x, y))
        }
    }

    pub fn from_subtract(x: Rc<Expression<T>>, y: Rc<Expression<T>>) -> Rc<Expression<T>> {
        if let Some((y1, y2)) = y.get_subtract() {
            Expression::from_add(x, Rc::new(Expression::Subtract(y2.clone(), y1.clone())))
        } else if let Some((x1, x2)) = x.get_subtract() {
            Rc::new(Expression::Subtract(
                x1.clone(),
                Rc::new(Expression::Add(x2.clone(), y)),
            ))
        } else {
            Rc::new(Expression::Subtract(x, y))
        }
    }

    pub fn from_multiply(x: Rc<Expression<T>>, y: Rc<Expression<T>>) -> Rc<Expression<T>> {
        if x.get_sqrt().is_some() && y.get_sqrt().is_some() {
            let (x_base, x_order) = x.get_sqrt().unwrap();
            let (y_base, y_order) = y.get_sqrt().unwrap();
            let min_order = usize::min(*x_order, *y_order);
            return Expression::from_sqrt(
                Expression::from_multiply(
                    Expression::from_sqrt(x_base.clone(), x_order - min_order),
                    Expression::from_sqrt(y_base.clone(), y_order - min_order),
                ),
                min_order,
            );
        }
        let x0 = x.get_divide();
        let y0 = y.get_divide();
        if x0.is_some() && y0.is_some() {
            Rc::new(Expression::Divide(
                Rc::new(Expression::Multiply(
                    x0.unwrap().0.clone(),
                    y0.unwrap().0.clone(),
                )),
                Rc::new(Expression::Multiply(
                    x0.unwrap().1.clone(),
                    y0.unwrap().1.clone(),
                )),
            ))
        } else if x0.is_some() {
            Rc::new(Expression::Divide(
                Rc::new(Expression::Multiply(x0.unwrap().0.clone(), y)),
                x0.unwrap().1.clone(),
            ))
        } else if y0.is_some() {
            Rc::new(Expression::Divide(
                Rc::new(Expression::Multiply(x, y0.unwrap().0.clone())),
                y0.unwrap().1.clone(),
            ))
        } else if let Some((y1, y2)) = y.get_multiply() {
            Rc::new(Expression::Multiply(
                Expression::from_multiply(x, y1.clone()),
                y2.clone(),
            ))
        } else {
            Rc::new(Expression::Multiply(x, y))
        }
    }

    pub fn from_divide(x: Rc<Expression<T>>, y: Rc<Expression<T>>) -> Rc<Expression<T>> {
        if let Some((y1, y2)) = y.get_divide() {
            Expression::from_multiply(x, Rc::new(Expression::Divide(y2.clone(), y1.clone())))
        } else if let Some((x1, x2)) = x.get_divide() {
            Rc::new(Expression::Divide(
                x1.clone(),
                Rc::new(Expression::Multiply(x2.clone(), y)),
            ))
        } else {
            Rc::new(Expression::Divide(x, y))
        }
    }

    pub fn from_power(x: Rc<Expression<T>>, y: Rc<Expression<T>>) -> Rc<Expression<T>> {
        if let Some((x1, x2)) = x.get_power() {
            Rc::new(Expression::Power(
                x1.clone(),
                Expression::from_multiply(x2.clone(), y),
            ))
        } else if let Some((x0, order)) = x.get_sqrt() {
            Rc::new(Expression::Sqrt(
                Expression::from_power(x0.clone(), y),
                *order,
            ))
        } else {
            Rc::new(Expression::Power(x, y))
        }
    }

    pub fn from_sqrt(x: Rc<Expression<T>>, order: usize) -> Rc<Expression<T>> {
        if order == 0 {
            x
        } else if let Some((y, z)) = x.get_sqrt() {
            Rc::new(Expression::Sqrt(y.clone(), z + order))
        } else if let Some((y, z)) = x.get_divide() {
            Rc::new(Expression::Divide(
                Rc::new(Expression::Sqrt(y.clone(), order)),
                Rc::new(Expression::Sqrt(z.clone(), order)),
            ))
        } else {
            Rc::new(Expression::Sqrt(x, order))
        }
    }

    pub fn from_factorial(x: Rc<Expression<T>>) -> Rc<Expression<T>> {
        Rc::new(Expression::Factorial(x))
    }
}

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug,Copy, Clone)]
pub enum Operator<T> {
    Push(T), Dup, Drop, Add, Sub, Mul, Div, Swap, Over
}

pub enum Error{
    DivisionByZero,
    StackUnderflow,
}

pub trait Zero{
    fn is_zero(&self) -> bool;
}

impl Zero for i32{
    fn is_zero(&self) -> bool {
        self == &0
    }
}

pub trait Stack<T>{
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, val: T);
}

impl<X> Operator<X>
    where X: Copy + Add<Output = X> + Sub<Output = X> + Mul<Output = X> + Div<Output = X> + Zero,
{
    fn pop1<T: Stack<X>>(stack: &mut T) -> Result<X,Error>{
        stack.pop().ok_or(Error::StackUnderflow)
    }
    fn pop2<T: Stack<X>>(stack: &mut T) -> Result<(X, X),Error>{
        stack.pop()
            .and_then(|val2|stack.pop().map(|val1|(val1, val2))).ok_or(Error::StackUnderflow)
    }

    pub fn apply<T: Stack<X>>(&self, stack: &mut T) -> Result<(), Error> {
        match self {
            Operator::Push(v) => {
                stack.push(*v);
                Ok(())
            },
            Operator::Dup => {
                let val = Self::pop1(stack)?;
                stack.push(val);
                stack.push(val);
                Ok(())
            }
            Operator::Drop => Self::pop1(stack).map(|_|()),
            Operator::Add => {
                let (a, b) = Self::pop2(stack)?;
                stack.push(a + b);
                Ok(())
            },
            Operator::Sub => {
                let (a, b) = Self::pop2(stack)?;
                stack.push(a - b);
                Ok(())
            }
            Operator::Mul => {
                let (a, b) = Self::pop2(stack)?;
                stack.push(a * b);
                Ok(())
            }
            Operator::Div => {
                let (a, b) = Self::pop2(stack)?;
                if b.is_zero() {
                    return Err(Error::DivisionByZero);
                }
                stack.push(a / b);
                Ok(())
            }
            Operator::Swap => {
                let (a, b) = Self::pop2(stack)?;
                stack.push(b);
                stack.push(a);
                Ok(())
            }
            Operator::Over => {
                let (a, b) = Self::pop2(stack)?;
                stack.push(a);
                stack.push(b);
                stack.push(a);
                Ok(())
            }
        }
    }
}




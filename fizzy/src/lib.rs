// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::ops::{Rem, Deref};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    pred: Box<dyn 'static + Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(_matcher: F, subs: S) -> Matcher<T>
        where
            S: ToString,
            F: 'static + Fn(T) -> bool,

    {
        Matcher {
            pred: Box::new(_matcher),
            subs: subs.to_string(),
        }
    }

    pub fn substitute(&self, val: T) -> Option<&String> {
        if (self.pred.deref())(val) {
            Some(&self.subs)
        } else {
            None
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

struct FizzIter<T, I> {
    inner: I,
    fizzy: Fizzy<T>,
}

impl<T, I> Iterator for FizzIter<T, I>
    where
        T: ToString + Copy,
        I: Iterator<Item=T>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.inner.next() {
            let val = self.fizzy.matchers.iter().fold(None, |acc, matcher| {
                if let Some(subs) = matcher.substitute(next) {
                    Some(acc.map_or(subs.to_owned(), |acc| acc + subs.as_str()))
                } else {
                    acc
                }
            });
            match val {
                None => Some(next.to_string()),
                Some(val) => Some(val)
            }
        } else { None }
    }
}

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { matchers: Vec::new() }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item=String>
        where
            T: ToString + Copy,
            I: Iterator<Item=T>,

    {
        FizzIter {
            inner: iter,
            fizzy: self,
        }
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
    where T: Rem<Output=T> + PartialEq + From<u8>
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n| n % T::from(5) == T::from(0), "buzz"))
}

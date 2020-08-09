use std::fmt;

/// Note: only the Display impl is special; the Debug impl is just a `#[derive]`.
/// Use <https://crates.io/crates/fmt_adapter> if this is an issue.
///
/// ```
/// use fmt_utils::std::Separated;
///
/// assert_eq!(Separated { sep: ',', iter: &[] as &[char] }.to_string(), "");
/// assert_eq!(Separated { sep: ',', iter: &['a'] }.to_string(), "a");
/// assert_eq!(Separated { sep: ',', iter: &['a', 'b'] }.to_string(), "a,b");
/// assert_eq!(Separated { sep: ',', iter: &['a', 'b', 'c'] }.to_string(), "a,b,c");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Separated<Sep, Iter>
where
    Sep: fmt::Display,
    Iter: Copy + IntoIterator,
    Iter::Item: fmt::Display,
{
    pub sep: Sep,
    pub iter: Iter,
}

impl<Sep, Iter> fmt::Display for Separated<Sep, Iter>
where
    Sep: fmt::Display,
    Iter: Copy + IntoIterator,
    Iter::Item: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = self.iter.into_iter();
        if let Some(x) = it.next() {
            write!(f, "{}", x)?;
            for y in it {
                write!(f, "{}{}", self.sep, y)?;
            }
        }
        Ok(())
    }
}

/// Note: only the Display impl is special; the Debug impl is just a `#[derive]`.
/// Use <https://crates.io/crates/fmt_adapter> if this is an issue.
///
/// ```
/// use fmt_utils::std::Repeated;
///
/// assert_eq!(Repeated { value: "", count: 0 }.to_string(), "");
/// assert_eq!(Repeated { value: 'a', count: 0 }.to_string(), "");
/// assert_eq!(Repeated { value: "ab", count: 0 }.to_string(), "");
/// assert_eq!(Repeated { value: "", count: 1 }.to_string(), "");
/// assert_eq!(Repeated { value: 'a', count: 1 }.to_string(), "a");
/// assert_eq!(Repeated { value: "ab", count: 1 }.to_string(), "ab");
/// assert_eq!(Repeated { value: "", count: 2 }.to_string(), "");
/// assert_eq!(Repeated { value: 'a', count: 2 }.to_string(), "aa");
/// assert_eq!(Repeated { value: "ab", count: 2 }.to_string(), "abab");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Repeated<T>
where
    T: fmt::Display,
{
    pub value: T,
    pub count: usize,
}

impl<T> fmt::Display for Repeated<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.count {
            write!(f, "{}", self.value)?
        }
        Ok(())
    }
}

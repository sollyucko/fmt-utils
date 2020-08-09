/// Do not use this if you already have a buffer you can use.
/// See also: [`fmt_to_cleared_string`]
pub fn fmt_to_new_string<V, S>(value: V, strategy: &S) -> String
where
    V: fast_fmt::Fmt<S>,
{
    let mut buf = String::new();
    value.fmt(&mut buf, strategy).unwrap(); // should be infallible
    buf
}

pub fn fmt_to_cleared_string<'a, V, S>(mut buf: &'a mut String, value: V, strategy: &S) -> &'a str
where
    V: fast_fmt::Fmt<S>,
{
    buf.clear();
    value.fmt(&mut buf, strategy).unwrap(); // should be infallible
    buf
}

/// ```
/// use fmt_utils::fast_fmt::{Separated, fmt_to_cleared_string};
///
/// let mut buf = String::new();
///
/// assert_eq!(fmt_to_cleared_string(&mut buf, Separated { sep: ',', iter: &[] as &[char] }, &fast_fmt::Display), "");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Separated { sep: ',', iter: &['a'] }, &fast_fmt::Display), "a");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Separated { sep: ',', iter: &['a', 'b'] }, &fast_fmt::Display), "a,b");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Separated { sep: ',', iter: &['a', 'b', 'c'] }, &fast_fmt::Display), "a,b,c");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Separated<Sep, Iter>
where
    Iter: Copy + IntoIterator,
{
    pub sep: Sep,
    pub iter: Iter,
}

impl<Sep, Iter, S> fast_fmt::Fmt<S> for Separated<Sep, Iter>
where
    Iter: Copy + IntoIterator,
    Iter::Item: fast_fmt::Fmt<S>,
    Sep: fast_fmt::Fmt<S>,
{
    fn fmt<W: fast_fmt::Write>(&self, writer: &mut W, strategy: &S) -> Result<(), W::Error> {
        let mut it = self.iter.into_iter();
        if let Some(x) = it.next() {
            x.fmt(writer, strategy)?;
            for y in it {
                self.sep.fmt(writer, strategy)?;
                y.fmt(writer, strategy)?;
            }
        }
        Ok(())
    }

    fn size_hint(&self, strategy: &S) -> usize {
        let mut it = self.iter.into_iter();
        if let Some(x) = it.next() {
            let mut result = x.size_hint(strategy);
            let mut num_seps = 0;
            for y in it {
                result += y.size_hint(strategy);
                num_seps += 1;
            }
            if num_seps > 0 {
                result += self.sep.size_hint(strategy) * num_seps;
            }
            result
        } else {
            0
        }
    }
}

/// ```
/// use fmt_utils::fast_fmt::{Repeated, fmt_to_cleared_string};
///
/// let mut buf = String::new();
///
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: "", count: 0 }, &fast_fmt::Display), "");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: 'a', count: 0 }, &fast_fmt::Display), "");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: "ab", count: 0 }, &fast_fmt::Display), "");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: "", count: 1 }, &fast_fmt::Display), "");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: 'a', count: 1 }, &fast_fmt::Display), "a");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: "ab", count: 1 }, &fast_fmt::Display), "ab");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: "", count: 2 }, &fast_fmt::Display), "");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: 'a', count: 2 }, &fast_fmt::Display), "aa");
/// assert_eq!(fmt_to_cleared_string(&mut buf, Repeated { value: "ab", count: 2 }, &fast_fmt::Display), "abab");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Repeated<T> {
    pub value: T,
    pub count: usize,
}

impl<T, S> fast_fmt::Fmt<S> for Repeated<T>
where
    T: fast_fmt::Fmt<S>,
{
    fn fmt<W: fast_fmt::Write>(&self, writer: &mut W, strategy: &S) -> Result<(), W::Error> {
        for _ in 0..self.count {
            self.value.fmt(writer, strategy)?;
        }
        Ok(())
    }

    fn size_hint(&self, strategy: &S) -> usize {
        if self.count == 0 {
            0
        } else {
            self.value.size_hint(strategy) * self.count
        }
    }
}

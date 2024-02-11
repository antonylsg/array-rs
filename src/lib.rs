use std::mem;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, Clone, Copy)]
pub struct NotEnoughItems;

pub trait TryFromIterator<A> {
    type Error: std::fmt::Debug;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = A>,
        Self: Sized;
}

fn try_from_iter<I, T, const N: usize>(iter: I) -> Option<[T; N]>
where
    I: IntoIterator<Item = T>,
{
    let mut iter = iter.into_iter();
    let mut buffer = mem::MaybeUninit::<[T; N]>::uninit();
    let ptr = &mut buffer as *mut mem::MaybeUninit<[T; N]> as *mut T;

    for i in 0..N {
        if let Some(next) = iter.next() {
            unsafe { ptr.add(i).write(next) };
        } else {
            return None;
        }
    }

    Some(unsafe { buffer.assume_init() })
}

impl<T, const N: usize> TryFromIterator<T> for [T; N] {
    type Error = NotEnoughItems;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        try_from_iter(iter).ok_or(NotEnoughItems)
    }
}

impl<T> TryFromIterator<T> for Vec<T> {
    type Error = std::convert::Infallible;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        Ok(iter.into_iter().collect())
    }
}

pub trait Array<Idx = usize>:
    TryFromIterator<Self::Item>
    + IntoIterator
    + AsRef<[Self::Item]>
    + AsMut<[Self::Item]>
    + Index<Idx, Output = Self::Item>
    + IndexMut<Idx>
    + Clone
{
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<Idx, T, U> Array<Idx> for T
where
    T: TryFromIterator<U>
        + IntoIterator<Item = U>
        + AsRef<[Self::Item]>
        + AsMut<[Self::Item]>
        + Index<Idx, Output = U>
        + IndexMut<Idx>
        + Clone,
{
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    use super::TryFromIterator;

    #[test]
    fn try_from_iter() {
        let array = <[_; 5]>::try_from_iter([0, 1, 2, 3, 4]).unwrap();
        assert_eq!(&array, &[0, 1, 2, 3, 4]);
    }

    fn as_slice<A: Array>(array: &A) -> &[A::Item] {
        array.as_ref()
    }

    #[test]
    fn as_slice_vector() {
        let vector = vec![0, 1, 2];

        as_slice(&vector);
    }

    #[test]
    fn as_slice_static_array() {
        let static_array = [0; 3];

        as_slice(&static_array);
    }
}

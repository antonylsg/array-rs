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

fn try_from_iter<I, T, const N: usize>(iter: I) -> Result<[T; N], NotEnoughItems>
where
    I: IntoIterator<Item = T>,
{
    let mut iter = iter.into_iter();
    let mut buffer = mem::MaybeUninit::<[T; N]>::uninit();
    let ptr: *mut T = unsafe { mem::transmute(&mut buffer) };

    for i in 0..N {
        if let Some(next) = iter.next() {
            unsafe { ptr.add(i).write(next) };
        } else {
            return Err(NotEnoughItems);
        }
    }

    Ok(unsafe { buffer.assume_init() })
}

impl<T, const N: usize> TryFromIterator<T> for [T; N] {
    type Error = NotEnoughItems;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        try_from_iter(iter)
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
}

impl<Idx, T, U> Array<Idx> for T where
    T: TryFromIterator<U>
        + IntoIterator<Item = U>
        + AsRef<[Self::Item]>
        + AsMut<[Self::Item]>
        + Index<Idx, Output = U>
        + IndexMut<Idx>
        + Clone
{
}

#[cfg(test)]
mod tests {
    use super::Array;

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

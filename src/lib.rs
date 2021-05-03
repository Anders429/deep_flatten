use std::iter::Flatten;

pub struct DeepFlatten<D, I, T>
where
    I: DeepFlattenIterator<D, T>,
{
    iter: I::Iter,
}

impl<D, I, T> Iterator for DeepFlatten<D, I, T>
where
    I: DeepFlattenIterator<D, T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub trait DeepFlattenExt: Iterator + Sized {
    fn deep_flatten<D, T>(self) -> DeepFlatten<D, Self, T>
    where
        Self: DeepFlattenIterator<D, T>,
    {
        DeepFlatten {
            iter: DeepFlattenIterator::flatten(self),
        }
    }
}

impl<I> DeepFlattenExt for I where I: Iterator {}

pub trait DeepFlattenIterator<D, T> {
    type Iter: Iterator<Item = T>;

    fn flatten(self) -> Self::Iter;
}

impl<I> DeepFlattenIterator<(), I::Item> for I
where
    I: Iterator,
{
    type Iter = Self;

    fn flatten(self) -> Self::Iter {
        self
    }
}

impl<D, I, T> DeepFlattenIterator<(D,), T> for I
where
    I: Iterator,
    Flatten<I>: DeepFlattenIterator<D, T>,
    <I as Iterator>::Item: IntoIterator,
{
    type Iter = <Flatten<I> as DeepFlattenIterator<D, T>>::Iter;

    fn flatten(self) -> Self::Iter {
        DeepFlattenIterator::flatten(self.flatten())
    }
}

#[cfg(test)]
mod tests {
    use crate::DeepFlattenExt;

    #[test]
    fn deep_flatten() {
        assert_eq!(
            vec![vec![1], vec![2, 3], vec![], vec![4, 5, 6]]
                .into_iter()
                .deep_flatten()
                .collect::<Vec<usize>>(),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}

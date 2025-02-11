//! methods for N dimensional vector

pub trait VecN<T, const N: usize> {
    fn add(&self, other: &[T; N]) -> Self;
    fn add_in_place(&mut self, other: &[T; N]);
    fn sub(&self, other: &[T; N]) -> Self;
    fn scale(&self, scalar: T) -> Self;
    fn scale_in_place(&mut self, scale: T);
    fn norm(&self) -> T;
}

impl<T, const N: usize> VecN<T, N> for [T; N]
where
    T: num_traits::Float,
{
    fn add(&self, other: &[T; N]) -> Self {
        std::array::from_fn(|i| self[i] + other[i])
    }
    fn add_in_place(&mut self, other: &[T; N]) {
        *self = self.add(other);
    }
    fn sub(&self, other: &[T; N]) -> Self {
        std::array::from_fn(|i| self[i] - other[i])
    }
    fn norm(&self) -> T {
        // self.iter().map(|&v| v * v).sum::<T>().sqrt() // remove because it requires std:iter::Sum
        self.iter()
            .fold(T::zero(), |acc, &elem| acc + elem * elem)
            .sqrt()
    }
    fn scale(&self, scalar: T) -> Self {
        std::array::from_fn(|i| self[i] * scalar)
    }
    fn scale_in_place(&mut self, scale: T) {
        *self = self.scale(scale);
    }
}

#[test]
fn test_add() {
    // Test with different array sizes
    assert_eq!([1., 2.].add(&[2., 3.]), [3., 5.]);
    assert_eq!([1., 2.].sub(&[2., 3.]), [-1., -1.]);
    assert_eq!([4., 3.].norm(), 5.);
    // assert_eq!([1, 2, 3].add(&[2, 3, 4]), [3, 5, 7]);
    // assert_eq!([1, 2, 3, 4].add(&[2, 3, 4, 5]), [3, 5, 7, 9]);
}

pub fn add_three<T, const N: usize>(a: &[T; N], b: &[T; N], c: &[T; N]) -> [T; N]
where
    T: num_traits::Float,
{
    std::array::from_fn(|i| a[i] + b[i] + c[i])
}

pub fn add_four<T, const N: usize>(a: &[T; N], b: &[T; N], c: &[T; N], d: &[T; N]) -> [T; N]
where
    T: num_traits::Float,
{
    std::array::from_fn(|i| a[i] + b[i] + c[i] + d[i])
}

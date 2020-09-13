use crate::field::Field;
use std::ops::{Add, Sub, Mul};
use std::iter::repeat;
use std::fmt::{Debug, Formatter, Error};

pub struct Polynomial<T: Field>(pub Vec<T::Element>);

impl<T: Field> Clone for Polynomial<T> {
	fn clone(&self) -> Self {
		Polynomial(self.0.clone())
	}
}

impl<T: Field<Element = U>, U: Debug + Copy> Debug for Polynomial<T> {
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str("Polynomial(")?;
		formatter.write_fmt(format_args!("{:?}", self.0))?;
		formatter.write_str(")")?;

		Ok(())
	}
}

impl<T: Field> Polynomial<T> {
	pub fn eval(&self, elem: T::Element) -> T::Element {
		let mut out = T::ZERO;
		let mut accum = T::ONE;

		for coeff in self.0.iter() {
			out = T::add(out, T::mul(*coeff, accum));
			accum = T::mul(accum, elem);
		}

		out
	}

	/// This will panic if: `by_polynomial` has a greater degree than `self`,
	/// the leading term of `by_polynomial` does not satisfy [Field::is_one](../field/trait.Field.html#tymethod.is_one),
	/// or `by_polynomial` is of degree zero.
	pub fn become_divisible(&self, by_polynomial: &Polynomial<T>) -> Self {
		assert!(T::is_one(*by_polynomial.0.last().unwrap()));

		let mut residue = self.clone();

		let degree_difference = self.degree().checked_sub(by_polynomial.degree()).unwrap();

		for i in 0 ..= degree_difference {
			let shifting_by = degree_difference - i;
			let subtractor = by_polynomial
				.shift_by(shifting_by)
				.multiply_by_scalar(residue.0[shifting_by + by_polynomial.degree() - 1]);

			residue = &residue - &subtractor;
		}

		self - &residue
	}

	// this sadly cannot be written as a Mul implementation because the implementations might conflict in an edge case
	pub fn multiply_by_scalar(&self, elem: T::Element) -> Self {
		let out = self.0.iter().map(|coeff| {
			T::mul(*coeff, elem)
		}).collect();

		Polynomial(out)
	}

	pub fn degree(&self) -> usize {
		self.0.len()
	}

	pub fn shift_by(&self, amount: usize) -> Self {
		let out = repeat(T::ZERO).take(amount).chain(self.0.iter().copied());

		Polynomial(out.collect())
	}
}

impl<T: Field> Add for &Polynomial<T> {
	type Output = Polynomial<T>;

	fn add(self, other: Self) -> Self::Output {
		let len = usize::max(self.0.len(), other.0.len());
		let mut out = vec![T::ZERO; len];

		for i in 0 .. self.0.len() {
			out[i] = T::add(out[i], self.0[i]);
		}

		for i in 0 .. other.0.len() {
			out[i] = T::add(out[i], other.0[i]);
		}

		Polynomial(out)
	}
}

impl<T: Field> Sub for &Polynomial<T> {
	type Output = Polynomial<T>;

	fn sub(self, other: Self) -> Self::Output {
		let len = usize::max(self.0.len(), other.0.len());
		let mut out = vec![T::ZERO; len];

		for i in 0 .. self.0.len() {
			out[i] = T::add(out[i], self.0[i]);
		}

		for i in 0 .. other.0.len() {
			out[i] = T::add(out[i], T::neg(other.0[i]));
		}

		Polynomial(out)
	}
}

impl<T: Field> Mul for &Polynomial<T> {
	type Output = Polynomial<T>;

	fn mul(self, other: Self) -> Self::Output {
		let len = (self.0.len().checked_add(other.0.len())).unwrap() - 1;
		let mut out = vec![T::ZERO; len];

		for i in 0 .. self.0.len() {
			for j in 0 .. other.0.len() {
				out[i + j] = T::add(out[i + j], T::mul(self.0[i], other.0[j]));
			}
		}

		Polynomial(out)
	}
}

#[test]
fn test_become_divisible() {
	use crate::gf256::GF256;

	let dividend = Polynomial::<GF256>(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

	let roots = [2, 3, 5, 7];

	let divisors: Vec<_> = roots.iter().map(|&num| {
		Polynomial(vec![GF256::neg(num), 1])
	}).collect();
	
	let mut divisor = Polynomial::<GF256>(vec![1]);

	for component in divisors.iter() {
		divisor = &divisor * component;
	}

	let out = dividend.become_divisible(&divisor);

	for &root in roots.iter() {
		assert!(out.eval(root) == 0);
	}
}

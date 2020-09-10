use crate::field::Field;
use std::ops::{Add, Sub, Mul};

#[derive(Clone)]
pub struct Polynomial<T: Field>(Vec<T::Element>);

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

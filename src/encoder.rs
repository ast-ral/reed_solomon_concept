use crate::field::Field;
use crate::polynomial::Polynomial;

pub fn encode<T: Field>(elements: Vec<T::Element>, error_codewords: usize) -> Vec<T::Element> {
	assert!(elements.len() + error_codewords <= T::SEQUENCE_LENGTH);

	let mut alpha_n = T::ONE;

	let divisors = (0 .. error_codewords).map(|_| {
		let out = Polynomial(vec![T::neg(alpha_n), T::ONE]);

		alpha_n = T::mul(alpha_n, T::ALPHA);

		out
	});

	let mut divisor: Polynomial<T> = Polynomial(vec![T::ONE]);

	for component in divisors {
		divisor = &divisor * &component;
	}

	Polynomial(elements).shift_by(error_codewords).become_divisible(&divisor).0
}

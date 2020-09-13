use crate::field::Field;
use crate::polynomial::Polynomial;

pub fn encode<T: Field>(elements: Vec<T::Element>, error_codewords: usize) -> Vec<T::Element> {
	assert!(elements.len() + error_codewords < T::SEQUENCE_LENGTH);

	let divisors = T::sequence().take(error_codewords).map(|elem| {
		Polynomial(vec![T::neg(elem), T::ONE])
	});

	let mut divisor: Polynomial<T> = Polynomial(vec![T::ONE]);

	for component in divisors {
		divisor = &divisor * &component;
	}

	Polynomial(elements).shift_by(error_codewords).become_divisible(&divisor).0
}

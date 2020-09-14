use crate::field::Field;
use crate::matrix::Matrix;
use crate::polynomial::Polynomial;

pub fn decode<T: Field>(elements: Vec<T::Element>, error_codewords: usize) -> Vec<usize> {
	assert!(elements.len() <= T::SEQUENCE_LENGTH);
	assert!(error_codewords < elements.len());

	let mut alpha_n = T::ONE;

	let syndromes: Vec<_> = (0 .. error_codewords).map(|_| {
		let mut out = T::ZERO;
		let x = alpha_n;
		alpha_n = T::mul(alpha_n, T::ALPHA);
		let mut x_n = T::ONE; // represents x^n

		for &coeff in elements.iter() {
			out = T::add(out, T::mul(coeff, x_n));
			x_n = T::mul(x, x_n)
		}

		out
	}).collect();

	if syndromes.iter().copied().all(T::is_zero) {
		return Vec::new();
		//return (&elements[error_codewords ..]).to_vec();
	}

	let can_locate = error_codewords / 2;

	let syndrome_matrix = Matrix::new(
		can_locate + 1,
		can_locate,
		|y, x| {
			syndromes[x + y]
		},
	);

	let coefficients = reduce_matrix::<T>(syndrome_matrix);
	let error_locator = Polynomial::<T>(coefficients);

	let inv_alpha = T::inv(T::ALPHA);
	let mut inv_alpha_n = T::ONE;

	let mut errors = Vec::with_capacity(error_locator.degree() - 1);

	for i in 0 .. elements.len() {
		let eval_on = inv_alpha_n;
		inv_alpha_n = T::mul(inv_alpha_n, inv_alpha);
		if T::is_zero(error_locator.eval(eval_on)) {
			errors.push(i);
		}
	}

	errors
}

/// Takes in a matrix, rrefs it and returns coefficients of the error polynomial.
fn reduce_matrix<T: Field>(mut matrix: Matrix<T::Element>) -> Vec<T::Element> {
	let mut errors = 0;

	let rows = matrix.rows();
	let cols = matrix.cols();

	// downward propagation
	for i in 0 .. cols {
		// verify the leading element is nonzero
		if T::is_zero(*matrix.get(i, i)) {
			let mut found = false;
			for j in i + 1 .. cols {
				if !T::is_zero(*matrix.get(j, i)) {
					for k in 0 .. rows {
						*matrix.get(i, k) = T::add(*matrix.get(i, k), *matrix.get(j, k));
					}
					found = true;
					break
				}
			}

			if !found {
				break
			}
		}

		let leading = *matrix.get(i, i);
		let inverse = T::inv(leading);

		for j in i + 1 .. cols {
			let factor = T::neg(T::mul(*matrix.get(j, i), inverse));

			for k in i .. rows {
				*matrix.get(j, k) = T::add(*matrix.get(j, k), T::mul(factor, *matrix.get(i, k)));
			}
		}

		errors += 1;
	}

	let mut out = Vec::with_capacity(errors + 1);
	out.push(T::ONE);

	// upward propagation
	for i in (0 .. errors).rev() {
		let val = T::mul(*matrix.get(i, errors), T::inv(*matrix.get(i, i)));
		//*matrix.get(i, errors) = val;
		out.push(T::neg(val));

		for j in (0 .. i).rev() {
			let subtracting = T::mul(val, *matrix.get(j, i));
			*matrix.get(j, errors) = T::add(*matrix.get(j, errors), T::neg(subtracting));
		}
	}

	out
}

#[test]
fn test_error_location() {
	use crate::gf256::GF256;
	use crate::encoder::encode;

	let msg = vec![0, 1, 2, 3, 4];
	let error_codewords = GF256::SEQUENCE_LENGTH - msg.len();

	let mut encoded = encode::<GF256>(msg.clone(), error_codewords);

	let errors: Vec<_> = (0 .. encoded.len()).step_by(3).collect();

	for i in errors.iter().copied() {
		encoded[i] ^= 7;
	}

	assert!(decode::<GF256>(encoded, error_codewords) == errors);
}

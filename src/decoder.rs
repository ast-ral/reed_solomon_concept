use crate::field::Field;
use crate::matrix::Matrix;
use crate::polynomial::Polynomial;

pub fn decode<T: Field>(elements: &mut [T::Element], error_codewords: usize) -> &mut [T::Element] {
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
		return &mut elements[error_codewords ..]
	}

	let can_locate = error_codewords / 2;

	let syndrome_matrix = Matrix::new(
		can_locate,
		can_locate + 1,
		|y, x| {
			syndromes[x + y]
		},
	);

	let mut coefficients = vec![T::ONE];

	reduce_matrix::<T, _>(syndrome_matrix, |elem| {
		coefficients.push(T::neg(elem))
	});

	let error_locator = Polynomial::<T>(coefficients);

	let inv_alpha = T::inv(T::ALPHA);
	let mut inv_alpha_n = T::ONE;

	let mut locations = Vec::with_capacity(error_locator.degree() - 1);
	let mut locators = Vec::with_capacity(error_locator.degree() - 1);

	for i in 0 .. elements.len() {
		let eval_on = inv_alpha_n;
		inv_alpha_n = T::mul(inv_alpha_n, inv_alpha);
		if T::is_zero(error_locator.eval(eval_on)) {
			locations.push(i);
			locators.push(T::inv(eval_on));
		}
	}

	let row_num = locations.len();
	let row_len = locations.len() + 1;
	let backing_len = row_num * row_len;

	let mut backing = vec![T::ZERO; backing_len];

	for (col, locator) in locators.into_iter().enumerate() {
		let mut accum = T::ONE;

		for i in 0 .. row_num {
			backing[i * row_len + col] = accum;
			accum = T::mul(accum, locator);
		}
	}

	for row in 0 .. row_num {
		backing[row * row_len + row_len - 1] = syndromes[row];
	}

	let correction_matrix = Matrix::from_raw(row_num, row_len, backing);

	reduce_matrix::<T, _>(correction_matrix, |elem| {
		let index = locations.pop().unwrap();
		elements[index] = T::add(elements[index], T::neg(elem));
	});

	assert!(locations.len() == 0);

	&mut elements[error_codewords ..]
}

/// Takes in a matrix, rrefs it and calls `push` for each variable that it solves for, in reverse order.
fn reduce_matrix<T: Field, F: FnMut(T::Element)>(mut matrix: Matrix<T::Element>, mut push: F) {
	// represents count of solvable variables
	let mut solvable = 0;

	let rows = matrix.rows();
	let cols = matrix.cols();

	// downward propagation
	for i in 0 .. rows {
		// verify the leading element is nonzero
		if T::is_zero(*matrix.get(i, i)) {
			let mut found = false;
			for j in i + 1 .. rows {
				if !T::is_zero(*matrix.get(j, i)) {
					for k in 0 .. cols {
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

		for j in i + 1 .. rows {
			let factor = T::neg(T::mul(*matrix.get(j, i), inverse));

			for k in i .. cols {
				*matrix.get(j, k) = T::add(*matrix.get(j, k), T::mul(factor, *matrix.get(i, k)));
			}
		}

		solvable += 1;
	}

	// upward propagation
	for i in (0 .. solvable).rev() {
		let val = T::mul(*matrix.get(i, solvable), T::inv(*matrix.get(i, i)));
		//*matrix.get(i, solvable) = val;
		push(val);

		for j in (0 .. i).rev() {
			let subtracting = T::mul(val, *matrix.get(j, i));
			*matrix.get(j, solvable) = T::add(*matrix.get(j, solvable), T::neg(subtracting));
		}
	}
}

#[test]
fn test_error_correction() {
	use crate::gf256::GF256;
	use crate::encoder::encode;

	let msg: Vec<_> = (0 .. 25).collect();
	let error_codewords = GF256::SEQUENCE_LENGTH - msg.len();

	let mut encoded = encode::<GF256>(msg.clone(), error_codewords);

	let original_encoded = encoded.clone();

	let error_locations: Vec<_> = (0 .. encoded.len()).step_by(3).collect();

	for i in error_locations.into_iter() {
		encoded[i] ^= 7;
	}

	assert!(decode::<GF256>(&mut encoded, error_codewords) == msg);
	assert!(encoded == original_encoded);
}

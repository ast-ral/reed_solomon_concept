use crate::field::Field;

pub struct GF256;

const REDUCING: u16 = 0b100011101;

const fn mul(left: u8, right: u8) -> u8 {
	let mut product: u16 = 0;

	let mut i = 0;

	while i < 8 {
		if left & 1 << i != 0 {
			product ^= (right as u16) << i;
		}

		i += 1;
	}

	let mut i = 8;

	while i != 0 {
		i -= 1;

		if product & 1 << (i + 8) != 0 {
			product ^= REDUCING << i;
		}
	}

	product as u8
}

const fn mul_table() -> [[u8; 256]; 256] {
	let mut out = [[0; 256]; 256];

	let mut a: u16 = 0;
	while a < 256 {
		let mut b: u16 = 0;
		while b < 256 {
			out[a as usize][b as usize] = mul(a as u8, b as u8);
			b += 1;
		}
		a += 1;
	}

	out
}

const fn inv_table() -> [u8; 256] {
	let mut out = [0; 256];

	let mut i: u16 = 0;

	// in this field, i^254 = i^(-1)
	while i < 256 {
		let n = i as u8;
		let n2 = mul(n, n);
		let n4 = mul(n2, n2);
		let n8 = mul(n4, n4);
		let n16 = mul(n8, n8);
		let n32 = mul(n16, n16);
		let n64 = mul(n32, n32);
		let n128 = mul(n64, n64);
		let n192 = mul(n128, n64);
		let n224 = mul(n192, n32);
		let n240 = mul(n224, n16);
		let n248 = mul(n240, n8);
		let n252 = mul(n248, n4);
		let n254 = mul(n252, n2);

		out[i as usize] = n254;
		i += 1;
	}

	out
}

const MUL_TABLE: [[u8; 256]; 256] = mul_table();

const INV_TABLE: [u8; 256] = inv_table();

impl Field for GF256 {
	const ORDER: u64 = 256;

	type Element = u8;

	const ZERO: u8 = 0;
	const ONE: u8 = 1;

	const ALPHA: u8 = 2;

	fn add(left: &u8, right: &u8) -> u8 {
		left ^ right
	}

	fn mul(left: &u8, right: &u8) -> u8 {
		MUL_TABLE[*left as usize][*right as usize]
	}

	fn neg(elem: &u8) -> u8 {
		*elem
	}

	fn inv(elem: &u8) -> u8 {
		assert!(*elem != 0);
		INV_TABLE[*elem as usize]
	}
}

#[test]
fn test_inverses() {
	for i in 1 .. 256 {
		let elem = i as u8;
		let inv = GF256::inv(&elem);

		assert!(GF256::mul(&elem, &inv) == 1);
	}
}

use crate::galois::GaloisField;

pub struct GF256;

const REDUCING: u16 = 0b100011101;

const fn mul(left: u8, right: u8) -> u8 {
	let mut product: u16 = 0;

	let mut i = 0;

	while i < 8 {
		if left & i << i != 0 {
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

const MUL_TABLE: [[u8; 256]; 256] = mul_table();

const fn exp_and_log_tables() -> ([u8; 255], [u8; 256]) {
	let mut exp_table = [0; 255];
	let mut log_table = [0; 256];

	let mut accum = 1;

	let mut i = 0;

	while i < 255 {
		exp_table[i as usize] = accum;
		log_table[accum as usize] = i;

		accum = MUL_TABLE[2][accum as usize];

		i += 1;
	}

	(exp_table, log_table)
}

const EXP_AND_LOG_TABLES: ([u8; 255], [u8; 256]) = exp_and_log_tables();
const EXP_TABLE: [u8; 255] = EXP_AND_LOG_TABLES.0;
const LOG_TABLE: [u8; 256] = EXP_AND_LOG_TABLES.1;

const fn inv_table() -> [u8; 256] {
	let mut out = [0; 256];

	let mut i = 0;

	while i < 256 {
		let log = LOG_TABLE[i as usize];
		let log_inv = ((log as u16 * 254) % 255) as u8;
		let inv = EXP_TABLE[log_inv as usize];

		out[log as usize] = inv;

		i += 1;
	}

	out
}

const INV_TABLE: [u8; 256] = inv_table();

impl GaloisField for GF256 {
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

	fn exp(num: u128) -> u8 {
		EXP_TABLE[(num % 255) as usize]
	}

	fn log(elem: &u8) -> u128 {
		assert!(*elem != 0);
		LOG_TABLE[*elem as usize] as u128
	}
}

pub struct Matrix<T> {
	rows: usize,
	cols: usize,
	backing: Vec<T>,
}

impl<T> Matrix<T> {
	pub fn get(&mut self, y: usize, x: usize) -> &mut T {
		assert!(y < self.rows);
		assert!(x < self.cols);

		&mut self.backing[y * self.cols + x]
	}

	pub fn new<F: FnMut(usize, usize) -> T>(rows: usize, cols: usize, mut generator: F) -> Self {
		let mut backing = Vec::with_capacity(rows * cols);

		let mut i = 0;

		backing.resize_with(rows * cols, || {
			let x = i % cols;
			let y = i / cols;

			i += 1;

			generator(y, x)
		});

		Matrix {rows, cols, backing}
	}

	pub fn rows(&self) -> usize {
		self.rows
	}

	pub fn cols(&self) -> usize {
		self.cols
	}

	pub fn from_raw(rows: usize, cols: usize, backing: Vec<T>) -> Self {
		assert!(backing.len() == rows * cols);

		Matrix {rows, cols, backing}
	}
}

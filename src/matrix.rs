pub struct Matrix<T> {
	rows: usize,
	cols: usize,
	backing: Vec<T>,
}

impl<T> Matrix<T> {
	pub fn get(&mut self, y: usize, x: usize) -> &mut T {
		assert!(x < self.rows);
		assert!(y < self.cols);

		&mut self.backing[y * self.rows + x]
	}

	pub fn new<F: FnMut(usize, usize) -> T>(rows: usize, cols: usize, mut generator: F) -> Self {
		let mut backing = Vec::with_capacity(rows * cols);

		let mut i = 0;

		backing.resize_with(rows * cols, || {
			let x = i % rows;
			let y = i / rows;

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
}

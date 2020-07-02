use tetra::graphics::Color;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
	negative: bool,
	shifted: bool
}

impl Position {
	fn negate(&mut self) {
		self.negative = !self.negative;
	}
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct TetrimoPosition {
	x: Position,
	y: Position
}

impl TetrimoPosition {
	pub fn rotate_left(&mut self) {
		let temp_y = y.clone();
		self.y = self.x;
		self.x = self.y;
		self.x.negate();
		self.y.negate();
	}

	pub fn rotate_right(&mut self) {
		let temp_y = y.clone();
		self.y = self.x;
		self.x = self.y;
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TetrimoShape {
	positions: [TetrimoPosition; 4],
	wide_x: bool,
	tall_y: bool
}

impl TetrimoShape {

	const fn from_vec_matrix(matrix: Vec<Vec<bool>>) -> TetrimoShape {
		let tall_y = matrix.len() == 4;
		let wide_x = matrix[0].len() == 4;
		let mut postitions = [TetrimoPosition::default(); 4];

		let mut index = 0;

		'build_shape: for i in 0..matrix.len() {
			for k in 0..matrix[0].len() {
				if matrix[i][k] {

					// y position
					let negative = i < 2;
					let shifted = if tall_y {
							i == 0 || i == 3
						} else {i != 1};
					let y = Position {negative, shifted}

					// x position
					let negative = k < 2;
					let shifted = if tall_y {
							k == 0 || k == 3
						} else {k != 1};
					let x = Position {negative, shifted}

					// add tetrimo piece
					let postition = TetrimoPosition{x, y};
					postitions[index] = position;
					index += 1;
				}
			}
		}

		TetrimoShape{postitions, wide_x, tall_y}
	}

	pub fn rotate_right(&mut self) {
		let index = 0;
		while index < self.positions.len() {
			self.positions[0].rotate_right();
			index += 1;
		}
	}

	pub fn rotate_left(&mut self) {
		let index = 0;
		while index < self.positions.len() {
			self.positions[0].rotate_left();
			index += 1;
		}
	}

	pub const I_SHAPE: TetrimoShape = from_vec_matrix(vec![
		vec![false, false, false, false],
		vec![true, true, true, true],
		vec![false, false, false, false],
		vec![false, false, false, false]
	]);

	pub const J_SHAPE: TetrimoShape = from_vec_matrix(vec![
		vec![true, false, false],
		vec![true, true, true],
		vec![false, false, false]
	]);

	pub const L_SHAPE: TetrimoShape = from_vec_matrix(vec![
		vec![false, false, true],
		vec![true, true, true],
		vec![false, false, false]
	]);

	pub const O_SHAPE: TetrimoShape = from_vec_matrix(vec![
		vec![false, false, false, false],
		vec![false, true, true, false],
		vec![false, true, true, false],
		vec![false, false, false, false]
	]);

	pub const Z_SHAPE: TetrimoShape = from_vec_matrix(vec![
		vec![true, true, false],
		vec![false, true, true],
		vec![false, false, false]
	]);

	pub const T_SHAPE: TetrimoShape = from_vec_matrix(vec![
		vec![false, true, false],
		vec![true, true, true],
		vec![false, false, false]
	]);

	pub const S_SHAPE: TetrimoShape = from_vec_matrix(vec![
		vec![false, true, true],
		vec![true, true, false],
		vec![false, false, false]
	]);
}

#[derive(Clone, Debug, PartialEq)]
struct TetrimoColor {
	color: Color
}

impl TetrimoColor {

	const fn rgb(r: f32, g: f32, b: f32) -> Self {
		TetrimoColor {color: Color::rgb(r, g, b)}
	}

	const CYAN: TetrimoColor = Self::rgb(0.0, 1.0, 1.0);
	const BLUE: TetrimoColor = Self::rgb(0.0, 0.0, 1.0);
	const ORANGE: TetrimoColor = Self::rgb(1.0, 0.65, 0.0);
	const YELLOW: TetrimoColor = Self::rgb(1.0, 1.0, 0.0);
	const GREEN: TetrimoColor = Self::rgb(0.0, 1.0, 0.0);
	const PURPLE: TetrimoColor = Self::rgb(1.0, 0.0, 1.0);
	const RED: TetrimoColor = Self::rgb(1.0, 0.0, 0.0);
}

#[derive(Clone, Debug, PartialEq)]
pub struct TetrimoType {
	shape: TetrimoShape,
	color: TetrimoColor
}

impl TetrimoType {
	const fn new(shape: TetrimoShape, color: TetrimoColor) -> TetrimoType {
		TetrimoType {shape, color}
	}

	pub fn rotate_right(&mut self) {
		self.shape.rotate_right();
	}

	pub fn rotate_left(&mut self) {
		self.shape.rotate_left();
	}

	pub const I_PIECE: TetrimoType = Self::new(TetrimoShape::I_SHAPE, TetrimoColor::CYAN);
	pub const J_PIECE: TetrimoType = Self::new(TetrimoShape::J_SHAPE, TetrimoColor::BLUE);
	pub const L_PIECE: TetrimoType = Self::new(TetrimoShape::L_SHAPE, TetrimoColor::ORANGE);
	pub const O_PIECE: TetrimoType = Self::new(TetrimoShape::O_SHAPE, TetrimoColor::YELLOW);
	pub const Z_PIECE: TetrimoType = Self::new(TetrimoShape::Z_SHAPE, TetrimoColor::GREEN);
	pub const T_PIECE: TetrimoType = Self::new(TetrimoShape::T_SHAPE, TetrimoColor::PURPLE);
	pub const S_PIECE: TetrimoType = Self::new(TetrimoShape::S_SHAPE, TetrimoColor::RED);
}

/*
bitflags!{
	struct Tetrimo: u16 {
		const X_ONE = 0b00000000000000001;
		const Y_ONE = 0b00000000000000010;
		const NEG_1 = 0b00000000000000100;

		const NEG_2 = 0b00000000000001000;
		const X_TWO = 0b00000000000010000;
		const Y_TWO = 0b00000000000100000;

		const NEG_3 = 0b00000000001000000;
		const X_TRI = 0b00000000010000000;
		const Y_TRI = 0b00000000100000000;

		const NEG_4 = 0b00000001000000000;
		const X_FOR = 0b00000010000000000;
		const Y_FOR = 0b00000100000000000;

		const WIDEX = 0b00001000000000000;
		const TALLY = 0b00010000000000000;
	}
}*/
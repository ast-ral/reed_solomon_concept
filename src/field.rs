/// This trait represents a [Galois Field](https://en.wikipedia.org/wiki/Finite_field).
/// The max supported sequence length is whatever can fit in a `usize`.
pub trait Field {
	// The order of the field.
	const SEQUENCE_LENGTH: usize;

	type Element: Copy;

	const ZERO: Self::Element;
	const ONE: Self::Element;

	/// A primitive element (multiplicative generator) for the field.
	const ALPHA: Self::Element;

	fn add(left: Self::Element, right: Self::Element) -> Self::Element;
	fn mul(left: Self::Element, right: Self::Element) -> Self::Element;

	/// Produces the additive inverse or negation of `elem`.
	fn neg(elem: Self::Element) -> Self::Element;

	/// Produces the multiplicative inverse of `elem`.
	fn inv(elem: Self::Element) -> Self::Element;

	fn is_zero(elem: Self::Element) -> bool;
	fn is_one(elem: Self::Element) -> bool;
}

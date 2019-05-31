use cgmath::Vector3;

use nom::{
	do_parse,
	named,
	number::complete::{
		be_f32,
		be_f64,
		le_f32,
		le_f64
	}
};

/// Bounds with minimum and maximum point limits and radius in 3D space
#[derive(Clone,Debug,Default,PartialEq)]
pub struct Bounds<T> {
	pub min: Vector3<T>,
	pub max: Vector3<T>,
	pub radius: Option<T>,
}

/// Type alias for float-based [`Bounds`]
pub type Bounds32 = Bounds<f32>;

/// Type alias for double-based [`Bounds`]
pub type Bounds64 = Bounds<f64>;

/// Parses a [`Bounds32`] in little endian with radius
named!(pub le_bounds32_r<Bounds32>,
	do_parse!(
		min: le_v3f >>
		max: le_v3f >>
		r: le_f32 >>
		(Bounds32 {
			min: min,
			max: max,
			radius: Some(r),
		})
	)
);

/// Parses a [`Bounds64`] in little endian with radius
named!(pub le_bounds64_r<Bounds64>,
	do_parse!(
		min: le_v3d >>
		max: le_v3d >>
		r: le_f64 >>
		(Bounds64 {
			min: min,
			max: max,
			radius: Some(r),
		})
	)
);

/// Parses a [`Bounds32`] in big endian with radius
named!(pub be_bounds32_r<Bounds32>,
	do_parse!(
		min: be_v3f >>
		max: be_v3f >>
		r: be_f32 >>
		(Bounds32 {
			min: min,
			max: max,
			radius: Some(r),
		})
	)
);

/// Parses a [`Bounds64`] in big endian with radius
named!(pub be_bounds64_r<Bounds64>,
	do_parse!(
		min: be_v3d >>
		max: be_v3d >>
		r: be_f64 >>
		(Bounds64 {
			min: min,
			max: max,
			radius: Some(r),
		})
	)
);

/// Parses a [`Bounds32`] in little endian with no radius
named!(pub le_bounds32<Bounds32>,
	do_parse!(
		min: le_v3f >>
		max: le_v3f >>
		(Bounds32 {
			min: min,
			max: max,
			radius: None,
		})
	)
);

/// Parses a [`Bounds64`] in little endian with no radius
named!(pub le_bounds64<Bounds64>,
	do_parse!(
		min: le_v3d >>
		max: le_v3d >>
		(Bounds64 {
			min: min,
			max: max,
			radius: None,
		})
	)
);

/// Parses a [`Bounds32`] in big endian with no radius
named!(pub be_bounds32<Bounds32>,
	do_parse!(
		min: be_v3f >>
		max: be_v3f >>
		(Bounds32 {
			min: min,
			max: max,
			radius: None,
		})
	)
);

/// Parses a [`Bounds64`] in big endian with no radius
named!(pub be_bounds64<Bounds64>,
	do_parse!(
		min: be_v3d >>
		max: be_v3d >>
		(Bounds64 {
			min: min,
			max: max,
			radius: None,
		})
	)
);

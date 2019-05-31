use cgmath::{
	Matrix4,
	Quaternion,
	Vector2,
	Vector3,
	Vector4
};

use nom::{
	character::complete::digit1,
	flat_map,
	named,
	number::complete::{
		be_f32,
		be_f64,
		le_f32,
		le_f64
	},
	opt,
	pair,
	parse_to,
	recognize,
	tag,
	take_until
};

/// Parses a three-dimensional double precision vector in big endian order from binary input
named!(pub be_v3d<Vector3<f64> >,
	do_parse!(
		x: be_f64 >>
		y: be_f64 >>
		z: be_f64 >>
		(Vector3::new(x, y, z))
	)
);

/// Parses a three-dimensional single precision vector in big endian order from binary input
named!(pub be_v3f<Vector3<f32> >,
	do_parse!(
		x: be_f32 >>
		y: be_f32 >>
		z: be_f32 >>
		(Vector3::new(x, y, z))
	)
);

/// Parses a 4x3 single precision matrix in little endian order from binary input
named!(pub le_mat4x3<Matrix4<f32> >,
	do_parse!(
		data: count!(le_f32, 12) >>
		(Matrix4::new(data.0, data.1, data.2, 1.0,
		data.3, data.4, data.5, 1.0,
		data.6, data.7, data.8, 1.0,
		data.9, data.10, data.11, 1.0))
	)
)

/// Parses a three-dimensional double precision vector in little endian order from binary input
named!(pub le_v3d<Vector3<f64> >,
	do_parse!(
		x: le_f64 >>
		y: le_f64 >>
		z: le_f64 >>
		(Vector3::new(x, y, z))
	)
);

/// Parses a three-dimensional single precision vector in little endian order from binary input
named!(pub le_v3f<Vector3<f32> >,
	do_parse!(
		x: le_f32 >>
		y: le_f32 >>
		z: le_f32 >>
		(Vector3::new(x, y, z))
	)
);

/// Parses a signed 32-bit integer from text input
named!(pub int<i32>, recognize!(pair!(opt!(tag!("-")), uint)));

/// Parses an unsigned 32-bit integer from text input
named!(pub uint<u32>, uint32);

/// Parses an unsigned byte from text input
named!(pub uint8<u8>, flat_map!(digit1, parse_to!(u8)));

/// Parses an unsigned 16-bit integer from text input
named!(pub uint16<u16>, flat_map!(digit1, parse_to!(u16)));

/// Parses an unsigned 32-bit integer from text input
named!(pub uint32<u32>, flat_map!(digit1, parse_to!(u32)));

/// Parses a null-terminated string from binary input
named!(pub zstr<&str>, take_until!("\0"));

use cgmath::{
	Matrix4,
	Quaternion,
	Vector2,
	Vector3,
	Vector4
};

use nom::{
	digit,
	flat_map,
	be_f32,
	le_f32,
	named,
	parse_to,
	take_until
};

named!(pub be_v3d<Vector3<f64>,
	do_parse!(
		x: be_f64 >>
		y: be_f64 >>
		z: be_f64 >>
		(Vector3::new(x, y, z))
	)
);

named!(pub be_v3f<Vector3<f32>,
	do_parse!(
		x: be_f32 >>
		y: be_f32 >>
		z: be_f32 >>
		(Vector3::new(x, y, z))
	)
);

named!(pub le_v3d<Vector3<f64>,
	do_parse!(
		x: le_f64 >>
		y: le_f64 >>
		z: le_f64 >>
		(Vector3::new(x, y, z))
	)
);

named!(pub le_v3f<Vector3<f32>,
	do_parse!(
		x: le_f32 >>
		y: le_f32 >>
		z: le_f32 >>
		(Vector3::new(x, y, z))
	)
);

/// Parses an unsigned 32-bit integer from text input
named!(pub uint<u32>, uint32);

/// Parses an unsigned byte from text input
named!(pub uint8<u8>, flat_map!(digit, parse_to!(u8)));

/// Parses an unsigned 16-bit integer from text input
named!(pub uint16<u16>, flat_map!(digit, parse_to!(u16)));

/// Parses an unsigned 32-bit integer from text input
named!(pub uint32<u32>, flat_map!(digit, parse_to!(u32)));

/// Parses a null-terminated string from binary input
named!(pub zstr<&str>, take_until!("\0"));

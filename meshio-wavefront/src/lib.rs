use cgmath::Vector3;

use nom::{
	character::complete::not_line_ending,
	do_parse,
	map_res,
	named,
	ws
};

use std::str;

named!(pub(crate) vector3<Vector3<f32> >,
	ws!(do_parse!(
		x: float >>
		y: float >>
		z: float >>
		(Vector3::new(x, y, z))
	))
);

named!(pub(crate) id<&str>, map_res!(not_line_ending, str::from_utf8));

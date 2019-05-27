extern crate nom;

use nom::{
	do_parse,
	map_res,
	named,
	not_line_ending,
	ws
};

use std::str;

use obj::*;

mod obj;

named!(pub(crate) vector3<Vector3f>,
	ws!(do_parse!(
		x: float >>
		y: float >>
		z: float >>
		(Vector3f {
			x: x,
			y: y,
			z: z,
		})
	))
);

named!(pub(crate) id<&str>, map_res!(not_line_ending, str::from_utf8));

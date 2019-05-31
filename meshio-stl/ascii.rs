use cgmath::Vector3;

use nom::{
	character::{
		complete::float,
		is_alphanumeric
	},
	do_parse,
	many1,
	map_res,
	named,
	opt,
	tag,
	take_while,
	ws
};

use std::str;

fn is_id_char(c: char) -> bool {
	is_alphanumeric(c) || c == '_'
}

named!(id, take_while!(is_id_char));

named!(vector3<Vector3<f32> >,
	ws!(do_parse!(
		x: float >>
		y: float >>
		z: float >>
		(Vector3::new(x, y, z))
	))
);

named!(soli

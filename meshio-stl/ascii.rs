extern crate nom;

extern crate meshio;

use nom::{
	do_parse,
	is_alphanumeric,
	many1,
	map_res,
	named,
	opt,
	tag,
	take_while,
	ws
};

use meshio::{
	Vector3f
};

use std::str;

fn is_id_char(c: char) -> bool {
	is_alphanumeric(c) || c == '_'
}

named!(id, take_while!(is_id_char));

named!(vector3<Vector3f>,
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

named!(soli

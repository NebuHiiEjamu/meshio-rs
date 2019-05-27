extern crate nom;

use nom::{
	do_parse,
	named,
	take_until_and_consume,
	ws
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Pose {
	pub name: String,
	pub rotation: Vector3,
	pub coord: Vector3,
	pub scale: Vector3,
}

named!(identifier<&[u8]>, take_until_and_consume!(":"));

named!(pub(crate) pose<Pose>,
	ws!(do_parse!(
		name: identifier >>
		rotation: vector3_ascii >>
		coord: vector3_ascii >>
		scale: vector3_ascii >>
		(Pose {
			name: String::from_utf8_lossy(name),
			rotation: rotation,
			coord: coord,
			scale: scale,
		})
	))
);

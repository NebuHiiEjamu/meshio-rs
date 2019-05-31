use cgmath::Vector3;

use nom::{
	do_parse,
	named,
	take_until,
	ws
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Pose {
	pub name: String,
	pub rotation: Vector3<f32>,
	pub coord: Vector3<f32>,
	pub scale: Vector3<f32>,
}

named!(identifier<&[u8]>, take_until!(":"));

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

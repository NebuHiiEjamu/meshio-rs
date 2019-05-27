extern crate nom;
extern crate byteorder;

extern crate meshio;

use nom::{
	cond,
	count,
	do_parse,
	le_f32,
	le_i16,
	le_u8,
	le_u16,
	le_u32,
	named,
	tag,
	take
};

use std::str;

use meshio::{
	ColorB,
	Vector2f,
	Vector3f,
	Vector3u,
	Vector4f,
	Vector4h
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum OptionType {
	POSE = 1,
	FLAGS = 2,
	NONE = 255,
	WASTE,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum ItemData {
	None(Vec<u32>),
	Flags(Vec<u32>),
	Pose(Pose),
	Waste(Vec<u32>),
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Item {
	opt_kind: OptionType,
	data: ItemData,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Header {
	version_major: u16,
	version_minor: u16,
	settings_size: u32,
	machine_name: String,
	user_name: String,
	files: String,
	pose: Pose,
	hash: u32,
	items: Vec<Item>,
}

impl Header {
	fn is_tangent(&self) -> bool {
		self.version_major <= 1 && self.version_minor <= 12
	}
}

named!(string<&str>,
	do_parse!(
		len1: le_u8 >>
		len2: cond!(len1 >= 128, le_u8) >>
		data: if len2.is_some() {
			take!((len1 % 128) + (len2 * 128))
		} else {
			take!(len1 % 128)
		}
		(str::from_utf8_unchecked(data))
	)
);

named!(header<Header>,
	do_parse!(
		tag!(&[160, 238, 4, 0]) >>
		ver_maj: le_u16 >>
		ver_min: le_u16 >>
	)
);

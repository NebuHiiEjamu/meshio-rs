extern crate nom;

extern crate meshio;

use nom::{
	alt,
	do_parse,
	float,
	many0,
	opt,
	switch,
	tag_no_case,
	value,
	ws
};

use meshio::{
	uint8,
	Vector2u,
	Vector3f,
	Vector3u
};

use super::{
	id,
	vector3
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Channel {
	RED,
	GREEN,
	BLUE,
	MATTE,
	LUMINANCE,
	Z_DEPTH
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Reflection {
	SPHERE,
	CUBE_TOP,
	CUBE_BOTTOM,
	CUBE_LEFT,
	CUBE_RIGHT,
	CUBE_BACK,
	CUBE_FRONT,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Map {
	path: String,
	origin: Option<Vector3u>,
	blendu: Option<bool>,
	blendv: Option<bool>,
	boost: Option<f32>,
	mm_base: Option<f32>,
	mm_gain: Option<f32>,
	scale: Option<Vector3u>,
	turbulence: Option<Vector3u>,
	resolution: Option<Vector2u>,
	clamp: Option<bool>,
	bump_multiplier: Option<f32>,
	channel: Option<Channel>,
}

named!(mtllib<&str>,
	ws!(do_parse!(
		tag_no_case!("newmtl") >>
		name: id >>
		(name)
	))
);

named!(ka<Vector3f>,
	ws!(do_parse!(
		tag_no_case!("ka") >>
		val: vector3 >>
		(val)
	))
);

named!(kd<Vector3f>,
	ws!(do_parse!(
		tag_no_case!("kd") >>
		val: vector3 >>
		(val)
	))
);

named!(ks<Vector3f>,
	ws!(do_parse!(
		tag_no_case!("ks") >>
		val: vector3 >>
		(val)
	))
);

named!(ns<f32>,
	ws!(do_parse!(
		tag_no_case!("ns") >>
		val: float >>
		(val)
	))
);

named!(d<f32>,
	ws!(do_parse!(
		tag_no_case!('d') >>
		val: float >>
		(val)
	))
);

named!(tr<f32>,
	ws!(do_parse!(
		tag_no_case!("tr") >>
		val: float >>
		(val)
	))
);

named!(illum<u8>,
	ws!(do_parse!(
		tag_no_case!("illum") >>
		val: uint8 >>
		(val)
	))
);

named!(uvw0<Vector3u>,
	ws!(do_parse!(
		x: uint >>
		y: opt!(uint) >>
		z: opt!(uint) >>
		(Vector3u {
			x: x,
			y: if y.is_none() { 0 } else { y },
			z: if z.is_none() { 0 } else { z },
		})
	))
);

named!(uvw1<Vector3u>,
	ws!(do_parse!(
		x: uint >>
		y: opt!(uint) >>
		z: opt!(uint) >>
		(Vector3u {
			x: x,
			y: if y.is_none() { 1 } else { y },
			z: if z.is_none() { 1 } else { z },
		})
	))
);

named!(boolean<bool>, alt!(
	tag_no_case!("off") => { |_| false } |
	tag_no_case!("on") => { |_| true }
));

named!(map<Map>,
	ws!(do_parse!(
		ops: many0!(alt!(
			do_parse!(
				tag_no_case!("-o") >>
				val: uvw0 >>
				(val)
			) |
			do_parse!(
				tag_no_case!("-s") >>
				val: uvw1 >>
				(val)
			) |
			do_parse!(
				tag_no_case!("-t") >>
				val: uvw0 >>
				(val)
			) |
			do_parse!(
				tag_no_case!("-blendu") >>
				val: boolean >>
				(val)
			) |
			do_parse!(
				tag_no_case!("-blendv") >>
				val: boolean >>
				(val)
			) |
			do_parse!(
				tag_no_case!("-boost") >>
				val: float >>
				(val)
			) |
			do_parse!(
				tag_no_case!("-mm") >>
				b: float >>
				g: float >>
				(b, g)
			) |
			do_parse!(
				tag_no_case!("-texres") >>
				x: uint >>
				tag_no_case!('x') >>
				y: uint >>
				(Vector2u {
					x: x,
					y: y,
				})
			) |
			do_parse!(
				tag_no_case!("-clamp") >>
				val: boolean >>
				(val)
			) |
			do_parse!(
				tag_no_case!("-bm") >>
				val: float >>
				(val)
			) |
			do_parse!(
				tag_no_case!("-imfchan") >>
				val: switch!(
					tag_no_case!('r') => value!(Channel::RED) |
					tag_no_case!('g') => value!(Channel::GREEN) |
					tag_no_case!('b') => value!(Channel::BLUE) |
					tag_no_case!('l') => value!(Channel::LUMINANCE) |
					tag_no_case!('m') => value!(Channel::MATTE) |
					tag_no_case!('z') => value!(Channel::Z_DEPTH) |
				) >>
				(val)
			)
		) >>
		path: id >>
		
	))
);

named!(map_ka<Map>,
	ws!(do_parse!(
		tag_no_case!("map_ka") >>
		da
	))
);

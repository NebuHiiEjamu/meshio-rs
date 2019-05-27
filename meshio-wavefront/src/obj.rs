extern crate nom;

extern crate meshio;

use nom::{
	alt,
	do_parse,
	float,
	many0,
	many1,
	named,
	opt,
	tag,
	tag_no_case,
	ws
};

use meshio::{
	uint,
	Vector3,
	Vector3f,
	Vector4f
};

use super::{
	id,
	vector3
};

type Face = Vector3<[u32; 3]>;

struct Group {
	name: Option<String>,
	material: Option<String>,
	vertices: Vec<Vector4f>,
	normals: Vec<Vector3f>,
	uvs: Vec<Vector3f>,
	psvs: Vec<Vector3f>,
	faces: Vec<Face>,
	lines: Vec<Vec<u32>>,
	smoothing: u32,
};

struct Object {
	name: String,
	groups: Vec<Group>,
}

impl Object {
	fn new_with_anon_group() -> Object {
		
	}
}

named!(v<Vector4f>,
	ws!(do_parse!(
		tag_no_case!('v') >>
		x: float >>
		y: float >>
		z: float >>
		w: opt!(float) >>
		(Vector4f {
			x: x,
			y: y,
			z: z,
			w: if w.is_none() { 1.0 } else { w.unwrap() },
		})
	))
);

named!(vt<Vector3f>,
	ws!(do_parse!(
		tag_no_case!("vt") >>
		u: float >>
		v: float >>
		w: opt!(float) >>
		(Vector3f {
			x: u,
			y: v,
			z: if w.is_none() { 0.0 } else { w.unwrap() },
		})
	))
);

named!(vn<Vector3f>,
	ws!(do_parse!(
		tag_no_case!("vn") >>
		v: vector3 >>
		(v)
	))
);

named!(vp<Vector3f>,
	ws!(do_parse!(
		tag_no_case!("vp") >>
		u: float >>
		v: opt!(float) >>
		w: opt!(float) >>
		(Vector3f {
			x: x,
			y: if v.is_none() { 1.0 } else { v.unwrap() },
			z: if w.is_none() { 0.0 } else { w.unwrap() },
		})
	))
);

named!(face_sub<[u32; 3]>,
	ws!(do_parse!(
		x: uint >>
		y: opt!(do_parse!(
			tag!('/') >>
			f: opt!(uint) >>
			(f)
		)) >>
		z: opt!(do_parse!(
			tag!('/') >>
			f: opt!(uint) >>
			(f)
		)) >>
		([x,
			if y.is_none() { 0 } else { y.unwrap() },
			if z.is_none() { 0 } else { z.unwrap() }
		])
	))
);

named!(f<Face>,
	ws!(do_parse!(
		tag_no_case!('f') >>
		x: face_sub >>
		y: face_sub >>
		z: face_sub >>
		(Face {
			x: x,
			y: y,
			z: z,
		})
	))
);

named!(l<Vec<u32> >,
	ws!(do_parse!(
		tag_no_case!('l') >>
		data: many1!(uint) >>
		(data)
	))
);

named!(mtllib<&str>,
	ws!(do_parse!(
		tag_no_case!("mtllib") >>
		name: id >>
		(name)
	))
);

named!(usemtl<&str>,
	ws!(do_parse!(
		tag_no_case!("usemtl") >>
		name: id >>
		(name)
	))
);

named!(g<Group>,
	ws!(do_parse!(
		tag_no_case!('g') >>
		name: id >>
		data: many1!(alt!(v | vn | vp | vt | f | l | s | usemtl)) >>
		(Group {
			name: Some(name.to_owned()),
		})
	))
);

named!(o<Object>,
	ws!(do_parse!(
		tag_no_case!('o') >>
		name: id >>
		data: alt!(many1!(g) |
			many1!(alt!(v | vn | vp | vt | f | l | s | g | usemtl))
		) >>
		(name, data)
	))
);

named!(s,
	ws!(do_parse!(
		tag_no_case!('s') >>
		val: alt!(tag_no_case!("off") | uint) >>
		(val)
	))
);

use nom::{
	character::complete::line_ending,
	do_parse,
	named,
	opt,
	pair,
	recognize,
	tag,
	take_until,
	ws
};

use meshio::{
	int,
	uint
};

named!(identifier<&[u8]>, take_until!(recognize!(line_ending)));

named!(bone<Bone>,
	do_parse!(
		name: identifier >>
		parent: int >>
		coords: vector3_ascii >>
		(Bone {
			name: String::from_utf8_lossy(name),
			parent: parent as i16,
			coords: coords,
		})
	)
);

named!(color<Color>,
	do_parse!(
		r: uint >>
		g: uint >>
		b: uint >>
		a: uint >>
		(Color {
			r: r as u8,
			g: g as u8,
			b: b as u8,
			a: a as u8,
		})
	)
);

named!(vector2<Vector2>,
	ws!(do_parse!(
		x: float >>
		y: float >>
		(Vector2 {
			x: x,
			y: y,
		})
	))
);

named!(vertex<Vertex>,
	do_parse!(
		coord: vector3_ascii >>
		normal: vector3_ascii >>
		color: color >>
		uv: vector2 >>
	)
);

named!(long_vector3<LongVector3>,
	do_parse!(
		x: uint >>
		y: uint >>
		z: uint >>
		(LongVector3 {
			
		})
	)
);

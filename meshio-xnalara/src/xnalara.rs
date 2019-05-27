extern crate nom;

extern crate meshio;

use nom::{
	do_parse,
	float,
	named,
	ws
};

use meshio::{
	ColorB,
	Vector2f,
	Vector3f,
	Vector3u,
	Vector4f,
	Vector4h
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Bone {
	pub name: String,
	pub parent: i16,
	pub coords: Vector3f,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Texture {
	pub path: String,
	pub uv_layer_id: u32,
}
#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Weight {
	pub bone: Vector4h,
	pub weight: Vector4f,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Vertex {
	pub coord: Vector3f,
	pub normal: Vector3f,
	pub color: ColorB,
	pub uvs: Vec<Vector2f>,
	pub weights: Vec<Weight>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Mesh {
	pub name: String,
	pub uv_layer_count: u32,
	pub vertices: Vec<Vertex>,
	pub faces: Vec<Vector3u>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Model {
	header: Header,
	pub bones: Vec<Bone>,
	pub meshes: Vec<Mesh>,
}

named!(pub(crate) vector3_ascii<Vector3f>,
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

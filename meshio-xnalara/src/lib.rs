use cgmath::{
	Vector2,
	Vector3,
	Vector4
};

use nom::{
	character::complete::float,
	do_parse,
	named,
	ws
};

use meshio::ColorB;

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Bone {
	pub name: String,
	pub parent: i16,
	pub coords: Vector3<f32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Texture {
	pub path: String,
	pub uv_layer_id: u32,
}
#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Weight {
	pub bone: Vector4<u16>,
	pub weight: Vector4<f32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Vertex {
	pub coord: Vector3<f32>,
	pub normal: Vector3<f32>,
	pub color: ColorB,
	pub uvs: Vec<Vector2<f32>>,
	pub weights: Vec<Weight>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Mesh {
	pub name: String,
	pub uv_layer_count: u32,
	pub vertices: Vec<Vertex>,
	pub faces: Vec<Vector3<u32>>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Model {
	header: Header,
	pub bones: Vec<Bone>,
	pub meshes: Vec<Mesh>,
}

named!(pub(crate) vector3_ascii<Vector3<f32> >,
	ws!(do_parse!(
		x: float >>
		y: float >>
		z: float >>
		(Vector3::new(x, y, z))
	))
);

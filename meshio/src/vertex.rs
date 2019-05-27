use cgmath::{
	Vector2,
	Vector3,
	Vector4
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct Vertex<P, C> {
	pub position: Vector3<P>,
	pub normals: [Vector3<P>, 2],
	pub uv: Vector2<P>,
	pub color: Color<C>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub enum Face {
	Triangle(Vector3<u32>),
	Quad(Vector4<u32>),
	Ngon(Vec<u32>),
}

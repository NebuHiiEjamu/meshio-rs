use cgmath::Vector3;

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) Facet {
	pub vertices: [Vector3<f32>; 3],
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Solid {
	name: Option<String>,
}

extern crate meshio;

use meshio::{
	Vector3f
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) Facet {
	pub vertices: [Vector3f; 3],
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Solid {
	name: Option<String>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct Node<P, C> {
	pub name: LocalizedStringMap,
	pub vertices: Vec<Vertex<P, C>>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct Object<P, C> {
	pub name: LocalizedStringMap,
	pub nodes: Vec<Node<P, C>>,
}

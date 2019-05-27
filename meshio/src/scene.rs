#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct Scene<P, C> {
	pub name: LocalizedStringMap,
	pub objects: Vec<Object>,
}

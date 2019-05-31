use nom::{
	do_parse,
	named,
	number::complete::{
		le_f32,
		le_u32
	},
	tag
};

use std::fmt::{
	Display,
	Formatter,
	Result,
	write
};

use sha1::Sha1;
use regex::Regex;

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Attribute {
	semantic: String,
	semantic_index: u32,
	offset: u32,
	size: u32,
	format: String,
	items: Vec<u32>,
	data: Vec<u8>,
}

impl Display for Attribute {
	fn fmt(&self, &mut f: Formatter) -> Result {
		write!(f, "[{}:{}:{}:{}:{}]", self.semantic, self.semantic_index,
			self.offset, self.size, self.format)
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Model {
	block_size: u32,
	hash: Sha1;
	attributes: Vec<Attribute>,
	textures: Vec<String>,
	shaders: Vec<String>,
	faces: Vec<[f32; 3]>,
}

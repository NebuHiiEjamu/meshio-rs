extern crate nom;

use nom::{
	count_fixed,
	do_parse,
	le_f32,
	le_u16,
	named,
	tag
};

pub mod pmd {
	#[derive(Clone,Debug,Default,PartialEq,Eq)]
	pub struct Header {
		version: f32,
		name_local: [u16; 10],
		name_global: [u16; 10],
		comment_local: [u16; 128],
		comment_global: [u16; 128],
	}
	
	named!(pub header<Header>,
		do_parse!(
			tag!("Pmd") >>
			version: le_f32 >>
			name: count_fixed!([&[u16], le_u16, 10) >>
			comment: count_fixed!([&[u16], le_u16, 128) >>
			(Header {
				version: version;
				name_local: name,
				comment_local: comment,
				...Default::default()
			})
		)
	);
}

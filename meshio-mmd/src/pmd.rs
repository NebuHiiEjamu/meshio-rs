use nom::{
	count,
	do_parse,
	le_f32,
	le_u16,
	named,
	tag
};

use meshio::{
	LocalizedStringMap
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Header {
	version: f32,
	name: LocalizedStringMap
	comment: LocalizedStringMap
}

named!(header<Header>,
	do_parse!(
		tag!("Pmd") >>
		version: le_f32 >>
		name: count!([&[u16], le_u16, 10) >>
		comment: count!([&[u16], le_u16, 128) >>
		(Header {
			version: version;
			name_local: name,
			comment_local: comment,
			...Default::default()
		})
	)
);

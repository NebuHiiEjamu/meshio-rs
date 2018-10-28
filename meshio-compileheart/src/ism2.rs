extern crate nom;

use nom::{
	do_parse,
	Endianness,
	le_u32,
	named,
	peek,
	tag,
	take
};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Header {
	version: u8,
	file_size: u32,
	section_count: u32,
}

named!(header<Header>,
	do_parse!(
		tag!("ISM2") >>
		endian_check: peek!(do_parse!(
			_: take!(16) >>
			endian: le_u32 >>
			(endian)
		)) >>
		version: le_u8 >>
		version_extra: take!(3) >>
		unknown3: le_u32 >>
		unknown4: le_u32 >>
		
	)
)

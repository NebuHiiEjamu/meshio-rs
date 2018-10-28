extern crate nom;

use nom::{
	do_parse,
	named,
	tag
};

pub mod mdx {
	named!(model<Model>,
		do_parse!(
			tag!("MDLX") >>
		)
	)
}

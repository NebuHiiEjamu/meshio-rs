use nom::{
	do_parse,
	named,
	tag
};

use mdlx1::*;

named!(model<Model>,
	do_parse!(
		tag!("MDLX") >>
	)
)

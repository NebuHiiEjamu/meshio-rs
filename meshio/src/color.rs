use nom::{
	do_parse,
	named,
	number::complete::{
		be_f32,
		be_f64,
		be_u8,
		le_f32,
		le_f64,
		le_u8
	}
};

/// A color with red, green, blue, and alpha values
#[derive(Clone,Debug,Default,PartialEq)]
pub struct Color<T> {
	pub red: T,
	pub green: T,
	pub blue: T,
	pub alpha: T,
}

/// Type alias for a [`Color`] of single precision floats
pub type ColorF = Color<f32>;

/// Type alias for a [`Color`] of double precision floats
pub type ColorD = Color<f64>;

/// Type alias for a [`Color`] of bytes
pub type ColorB = Color<u8>;

/// Parses a ['ColorB'] as little endian RGBA
named!(pub le_rgba_b<ColorB>,
	do_parse!(
		r: le_u8 >>
		g: le_u8 >>
		b: le_u8 >>
		a: le_u8 >>
		(ColorB {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		})
	)
);

/// Parses a ['ColorB'] as little endian ARGB
named!(pub le_argb_b<ColorB>,
	do_parse!(
		a: le_u8 >>
		r: le_u8 >>
		g: le_u8 >>
		b: le_u8 >>
		(ColorB {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		})
	)
);

/// Parses a ['ColorB'] as little endian BGRA
named!(pub le_bgra_b<ColorB>,
	do_parse!(
		b: le_u8 >>
		g: le_u8 >>
		r: le_u8 >>
		a: le_u8 >>
		(ColorB {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		})
	)
);

/// Parses a ['ColorF'] as little endian RGBA
named!(pub le_rgba_f<ColorF>,
	do_parse!(
		r: le_f32 >>
		g: le_f32 >>
		b: le_f32 >>
		a: le_f32 >>
		(ColorF {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		})
	)
);

/// Parses a ['ColorF'] as little endian ARGB
named!(pub le_argb_f<ColorF>,
	do_parse!(
		a: le_f32 >>
		r: le_f32 >>
		g: le_f32 >>
		b: le_f32 >>
		(ColorF {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		})
	)
);

/// Parses a ['ColorF'] as little endian RGB
named!(pub le_rgb_f<ColorF>,
	do_parse!(
		r: le_f32 >>
		g: le_f32 >>
		b: le_f32 >>
		(ColorF {
			red: r,
			green: g,
			blue: b,
			alpha: 1.0,
		})
	)
);

/// Parses a ['ColorB'] as big endian ARGB
named!(pub be_argb_b<ColorB>,
	do_parse!(
		a: be_u8 >>
		r: be_u8 >>
		g: be_u8 >>
		b: be_u8 >>
		(ColorB {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		})
	)
);

/// Parses a ['ColorB'] as big endian BGRA
named!(pub be_bgra_b<ColorB>,
	do_parse!(
		b: be_u8 >>
		g: be_u8 >>
		r: be_u8 >>
		a: be_u8 >>
		(ColorB {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		})
	)
);

/// Parses a ['ColorF'] as big endian RGBA
named!(pub be_rgba_f<ColorF>,
	do_parse!(
		r: be_f32 >>
		g: be_f32 >>
		b: be_f32 >>
		a: be_f32 >>
		(ColorF {
			red: r,
			green: g,
			blue: b,
			alpha: a,
		})
	)
);

/// Parses a ['ColorF'] as big endian RGB
named!(pub be_rgb_f<ColorF>,
	do_parse!(
		r: be_f32 >>
		g: be_f32 >>
		b: be_f32 >>
		(ColorF {
			red: r,
			green: g,
			blue: b,
			alpha: 1.0,
		})
	)
);

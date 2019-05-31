use cgmath::{
	Vector2,
	Vector3
};
use mdlx1::*;

use nom::{
	alt,
	character::complete::float,
	count,
	delimited,
	do_parse,
	many0,
	many1,
	named,
	opt,
	tag,
	ws
};

use meshio::{
	uint8,
	uint16,
	uint32
};

named!(version<u32>,
	ws!(do_parse!(
		tag!("Version") >>
		format_version: delimited!(
			tag!("{"),
			do_parse!(
				tag!("FormatVersion") >> v: uint32 >>
				(v)
			),
			tag!("}")
		) >>
		(format_version.0)
	))
);

named!(vector3<Vector3<f32> >,
	ws!(delimited!(
		tag!("{"),
		do_parse!(
			x: float >> tag!(",") >> y: float >> tag!(",") >> z: float >>
			([x, y, z])
		),
		tag!("}"),
	))
);

named!(vector2<Vector2<f32> >,
	ws!(delimited!(
		tag!("{"),
		do_parse!(x: float >> tag!(",") >> y: float >>
			([x, y])
		),
		tag!("}"),
	))
);

named!(string<Vec[u8]>, delimited!(tag!("\""), many0!(anychar), tag!("\"")));

fn string80(s: &[u8]) -> String {
	let mut newstr = String::from(s);
	newstr.truncate(80);
	newstr
}

fn string260(s: &[u8]) -> String {
	let mut newstr = String::from(s);
	newstr.truncate(260);
	newstr
}

named!(min_extent<[f32; 3]>,
	ws!(do_parse!(
		tag!("MinimumExtent") >> v3: vector3 >> tag!(",")
		(v3)
	))
);

named!(max_extent<[f32; 3]>,
	ws!(do_parse!(
		tag!("MaximumExtent") >> v3: vector3 >> tag!(",")
		(v3)
	))
);

named!(bounds_radius<f32>,
	ws!(do_parse!(
		tag!("BoundsRadius") >> f: float >> tag!(",")
		(f)
	))
);

named!(model<Model>,
	ws!(do_parse!(
		tag!("Model") >>
		name: string >>
		data: delimited!(
			tag!("{"),
			do_parse!(
				tag!("BlendTime") >>
				blend_time: uint32 >>
				tag!(",") >>
				minimum_extent: min_extent >>
				maximum_extent: max_extent >>
				(blend_time, minimum_extent, maximum_extent)
			),
			tag!("}")
		) >>
		(Model {
			name: string80(&name[..]),
			extent: Extent {
				minimum: data.1,
				maximum: data.2,
			},
			blend_time: data.0,
		})
	))
)

named!(sequence<Sequence>,
	ws!(do_parse!(
		tag!("Anim") >>
		name: string >>
		data: delimited!(
			tag!("{"),
			do_parse!(
				tag!("Interval") >>
				interval: vector2 >>
				tag!(",") >>
				move_speed: opt!(do_parse!(
					tag!("MoveSpeed") >>
					f: float >>
					tag!(",") >>
					(f)
				)) >>
				rarity: opt!(do_parse!(
					tag!("Rarity") >>
					f: float >>
					tag!(",") >>
					(f)
				)) >>
				flag: opt!(tag!("NonLooping") => { |_| true }) >>
				tag!(",") >>
				minimum_extent: min_extent >>
				maximum_extent: max_extent >>
				bounds_radius: bounds_radius >>
				(interval, move_speed, rarity, flag, minimum_extent,
					maximum_extent, bounds_radius)
			),
			tag!("}")
		) >>
		(Sequence {
			name: string80(&name[..]),
			interval: data.0,
			move_speed: if let Some(ms) = data.1 {
				ms
			} else {
				0.0
			},
			rarity: if let Some(r) = data.2 {
				r
			} else {
				0.0
			},
			flags: if let Some(fl) = data.3 {
				if fl == true { SequenceFlag::NON_LOOPING }
			} else {
				SequenceFlag::LOOPING
			},
			extent: Extent {
				minimum_extent: data.4,
				maximum_extent: data.5,
				bounds_radius: data.6,
			},
		})
	))
);

named!(sequences<<Vec<Sequence> >,
	ws!(do_parse!(
		tag!("Sequences") >>
		count: uint32 >>
		anims: delimited!(tag!("{"), count!(sequence, count), tag!("}")) >>
		(anims)
	))
);

named!(texture<Texture>,
	ws!(do_parse!(
		tag!("Bitmap") >>
		data: delimited!(
			tag!("{"),
			do_parse!(
				tag!("Image") >>
				image: string >>
				tag!(",") >>
				replaceable_id: opt!(do_parse!(
					tag!("ReplaceableId") >>
					id: uint32 >>
					tag!(",") >>
					(id)
				)) >>
				(image, replaceable_id)
			),
			tag("}")
		) >>
		(Texture {
			replaceable_id: data.1,
			filename: string260(&data.0[..]),
		})
	))
);

named!(textures<<Vec<Texture> >,
	ws!(do_parse!(
		tag!("Textures") >>
		count: uint32 >>
		bitmaps: delimited!(tag!("{"), count!(texture, count), tag!("}")) >>
		(bitmaps)
	))
);

fn shading(flags_str: &[Vec<u8>]) -> Shading {
	let mut flags: Shading;
	
	for flag in &flags_str {
		flags |= match &flag[..] {
			b"Unshaded" => Shading::UNSHADED,
			b"TwoSided" => Shading::TWO_SIDED,
			_ => 0
		}
	}
	
	flags
}

named!(layer<Layer>,
	ws!(do_parse!(
		tag!("Layer") >>
		lyr: delimited!(
			tag!("{"),
			do_parse!(
				tag!("FilterMode") >>
				filter_mode: alt!(
					tag!("None") |
					tag!("Transparent") |
					tag!("Blend")
				) >>
				tag!(",") >>
				flags: many0!(alt!(
					tag!("Unshaded") |
					tag!("TwoSided")
				)) >>
				tag!("static") >>
				tag!("TextureID") >>
				texture_id: uint32 >>
				tag!(",") >>
				(filter_mode, flags, texture_id)
			),
			tag!("}")
		) >>
		(Layer {
			filter: match &lyr.0[..] {
				b"Transparent" => LayerFilter::TRANSPARENT,
				b"Blend" => LayerFilter::BLEND,
				_ => LayerFilter::NONE
			},
			shading: shading(&lyr.1[..]),
		})
	))
);

named!(materials<Vec<Material> >,
	ws!(do_parse!(
		tag!("Materials") >>
		count: uint32 >>
		mats: delimited!(
			tag!("{"),
			count!(do_parse!(
				tag!("Material") >>
				mat: delimited!(tag!("{"), many1!(layer), tag!("}")) >>
				(Material {
					layers: mat,
				})), count),
			tag!("}")
		) >>
		(mats)
	))
);

named!(geoset<Geoset>,
	ws!(do_parse!(
		tag!("Geoset") >>
		data: delimited!(
			tag!("{"),
			do_parse!(
				tag!("Vertices") >>
				vcount: uint32 >>
				verts: delimited!( tag!("{"), count!(vector3, vcount), tag!("}")) >>
				tag!("Normals") >>
				ncount: uint32 >>
				norms: delimited!(tag!("{"), count!(vector3, ncount), tag!("}")) >>
				tag!("TVertices") >>
				tvcount: uint32 >>
				tverts: delimited!(tag!("{"), count!(vector2, tvcount), tag!("}")) >>
				tag!("VertexGroup") >>
				vg: delimited!(
					tag!("{"), many1!(do_parse!(id: uint8 >> tag!(",") >> (id))), tag!("}")
				) >>
				tag!("Faces") >>
				fgcount: uint32 >>
				fcount: uint32 >>
				fg: delimited!(
					tag!("{"),
					do_parse!(
						tag!("Triangles") >>
						tris: delimited!(
							tag!("{"),
							count!(do_parse!(id: uint16 >> tag!(",") >> (id)), tag!("}")),
							tag!("}")
						) >>
						(tris)
					),
					tag!("}")
				) >>
				tag!("Groups") >>
				mgcount: uint32 >>
				midxs: uint32 >>
				mg: delimited!(
					tag!("{"),
					do_parse
					tag!("}")
				) >>
				(verts, norms, tverts, vg, fg, mg)
			)
		)
	)
));

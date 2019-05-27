use bitflags::bitflags;
use bytes::BufMut;
use std::{u8,u16};

use nom::{
	call,
	cond,
	count,
	do_parse,
	Err,
	IResult,
	le_f32,
	le_i16,
	le_i32,
	le_u8,
	le_u16,
	le_u32,
	length_bytes,
	map,
	named,
	named_args,
	Needed,
	switch,
	tag,
	take,
	value
};

use encoding::{
	ByteWriter,
	DecoderTrap,
	EncoderTrap,
	UTF_16LE_ENCODING
};

use meshio::{
	BoundsF,
	ColorF,
	Language,
	le_bbf,
	le_rgba_f,
	le_v2f,
	le_v3f,
	le_v4f,
	LocalizedStringMap,
};

use cgmath::{
	Vector2,
	Vector3,
	Vector4
};

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum Encoding {
	UTF16LE = 0,
	UTF8 = 1,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Settings {
	encoding: Encoding,
	uv: u8,
	vertex_index_size: u8,
	texture_index_size: u8,
	material_index_size: u8,
	bone_index_size: u8,
	morph_index_size: u8,
	rigid_body_index_size: u8,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Header {
	version: f32,
	settings: Settings,
	name: LocalizedStringMap,
	comment: LocalizedStringMap,
}

#[derive(Clone,Debug,PartialEq,Eq)]
enum Skinning {
	BDef1 {
		index: i32
	},
	BDef2 {
		indices: [i32; 2],
		weight: f32
	},
	BDef4 {
		indices: [i32; 4],
		weights: [f32; 4]
	},
	SDef {
		indices: [i32; 2],
		weight: f32,
		c: Vector3<f32>,
		r: [Vector3<f32>; 2]
	},
	QDef {
		indices: [i32; 4],
		weights: [f32; 4]
	}
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Vertex {
	position: Vector3<f32>,
	normal: Vector3<f32>,
	uv: Vector2<f32>,
	uva: Vec<Vector4<f32>>,
	skinning: Skinning,
	edge: f32,
}

bitflags! {
	struct MaterialFlags: u8 {
		const NO_CULL = 1;
		const GROUND_SHADOW = 2;
		const DRAW_SHADOW = 4;
		const RECEIVE_SHADOW = 8;
		const HAS_EDGE = 16;
		const VERTEX_COLOR = 32;
		const POINT_DRAWING = 64;
		const LINE_DRAWING = 128;
	}
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum BlendMode {
	DISABLED = 0,
	MULTIPLY = 1,
	ADDITIVE = 2,
	ADDITIONAL_VEC4 = 3,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum ToonReference {
	TEXTURE = 0,
	INTERNAL = 1,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Material {
	name: LocalizedStringMap,
	diffuse: ColorF,
	specular: ColorF,
	specularity: f32,
	ambient: ColorF,
	flags: MaterialFlags,
	edge_color: ColorF,
	edge_scale: f32,
	texture: i32,
	environment: i32,
	environment_blend_mode: BlendMode,
	toon_reference: ToonReference,
	toon: i32,
	comment: String,
	surface_count: u32,
}

bitflags! {
	struct BoneFlags: u16 {
		const INDEXED_TAIL_POSITION = 1;
		const ROTATABLE = 2;
		const TRANSLATABLE = 4;
		const VISIBLE = 8;
		const ENABLED = 16;
		const USE_IK = 32;
		const INHERIT_ROTATION = 256;
		const INHERIT_TRANSLATION = 512;
		const FIXED_AXIS = 1024;
		const LOCAL_COORDINATE = 2048;
		const PHYSICS_AFTER_DEFORM = 4096;
		const EXTERNAL_PARENT_DEFORM = 8192;
	}
}

#[derive(Clone,Debug,PartialEq,Eq)]
enum TailPosition {
	Vector3(Vector3<f32>),
	Index(i32),
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct InheritBone {
	index: i32,
	weight: f32,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct LocalCoordinate {
	x: Vector3<f32>,
	z: Vector3<f32>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Link {
	bone_index: i32,
	has_limits: bool,
	limits: Option<Vec<BoundsF>>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct InverseKinematic {
	target: i32,
	loop_count: u32,
	angle_limit: f32,
	links: Option<Vec<Link>>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Bone {
	name_local: String,
	name_global: String,
	position: Vector3<f32>,
	parent: i32,
	layer: i32,
	flags: BoneFlags,
	tail_position: TailPosition,
	inherit_bone: Option<InheritBone>,
	fixed_axis: Option<Vector3<f32>>,
	local_coordinate: Option<LocalCoordinate>,
	key: Option<i32>,
	ik: Option<InverseKinematic>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum Category {
	RESERVED = 0,
	EYEBROW = 1,
	EYE = 2,
	MOUTH = 3,
	OTHER = 4,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum MorphType {
	GROUP = 0,
	VERTEX = 1,
	BONE = 2,
	UV = 3,
	ADDITIONAL_UV1 = 4,
	ADDITIONAL_UV2 = 5,
	ADDITIONAL_UV3 = 6,
	ADDITIONAL_UV4 = 7,
	MATERIAL = 8,
	FLIP = 9,
	IMPULSE = 10,
}

#[derive(Clone,Debug,PartialEq,Eq)]
enum Offset {
	Group {
		index: i32,
		weight: f32,
	},
	Vertex {
		index: i32,
		translation: Vector3<f32>,
	},
	Bone {
		index: i32,
		translation: Vector3<f32>,
		rotation: Vector4<f32>,
	},
	Uv {
		index: i32,
		offsets: Vector4<f32>,
	},
	Material {
		index: i32,
		operation: u8,
		diffuse: ColorF,
		specular: ColorF,
		specularity: f32,
		ambient: ColorF,
		edge_color: ColorF,
		edge_scale: f32,
		texture_tint: ColorF,
		environment_tint: ColorF,
		toon_tint: ColorF,
	},
	Flip {
		index: i32,
		weight: f32,
	},
	Impulse {
		index: i32,
		is_local: bool,
		speed: Vector3<f32>,
		torque: Vector3<f32>,
	},
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Morph {
	name_local: String,
	name_global: String,
	category: Category,
	kind: MorphType,
	offsets: Vec<Offset>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum FrameFlag {
	NORMAL = 0,
	SPECIAL = 1,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum FrameType {
	BONE = 0,
	MORPH = 1,
}

#[derive(Clone,Debug,PartialEq,Eq)]
enum FrameElement {
	kind: FrameType,
	index: i32,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Frame {
	name_local: String,
	name_global: String,
	flag: FrameFlag,
	elements: Vec<FrameElement>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum RigidShape {
	SPHERE = 0,
	BOX = 1,
	CAPSULE = 2,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum Physics {
	FOLLOW_BONE = 0,
	GRAVITY = 1,
	GRAVITY_BONE = 2,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct RigidBody {
	name_local: String,
	name_global: String,
	bone: i32,
	group: u8,
	mask: u16,
	shape: RigidShape,
	scale: Vector3<f32>,
	position: Vector3<f32>,
	rotation: Vector3<f32>,
	mass: f32,
	attenuation: f32,
	damping: f32,
	repulsion: f32,
	friction: f32,
	mode: Physics,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum JointType {
	SPRING_6DOF = 0,
	SIX_DOF = 1,
	P2P = 2,
	CONE_TWIST = 3,
	SLIDER = 4,
	HINGE = 5,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Joint {
	name_local: String,
	name_global: String,
	kind: JointType,
	indices: [i32; 2],
	position: Vector3<f32>,
	rotation: Vector3<f32>,
	position_min: Vector3<f32>,
	position_max: Vector3<f32>,
	rotation_min: Vector3<f32>,
	rotation_max: Vector3<f32>,
	position_spring: Vector3<f32>,
	rotation_spring: Vector3<f32>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[repr(u8)]
enum SoftShape {
	TRI_MESH = 0,
	ROPE = 1,
}

bitflags! {
	struct SoftBodyFlags: u8 {
		const B_LINK = 1;
		const CLUSTER_CREATION = 2;
		const LINK_CROSSING = 4;
	}
}

#[derive(Clone,Debug,PartialEq,Eq)]
enum Aerodynamics {
	V_POINT = 0,
	V_2SIDED = 1,
	V_1SIDED = 2,
	F_2SIDED = 3,
	F_1SIDED = 4,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct AnchorRigidBody {
	rigid_body: i32,
	vertex: i32,
	near_mode: u8,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct SoftBody {
	name_local: String,
	name_global: String,
	shape: Shape,
	material: i32,
	group: u8,
	mask: u16,
	flags: SoftBodyFlags,
	b_link_create_distance: u32,
	cluster_count: u32,
	total_mass: f32,
	collision_margin: f32,
	aerodynamics: Aerodynamics,
	vcf: f32,
	dp: f32,
	dg: f32,
	lf: f32,
	pr: f32,
	vc: f32,
	df: f32,
	mt: f32,
	chr: f32,
	khr: f32,
	shr: f32,
	ahr: f32,
	srhr_cl: f32,
	skhr_cl: f32,
	sshr_cl: f32,
	sr_splt_cl: f32,
	sk_splt_cl: f32,
	ss_splt_cl: f32,
	v_it: u32,
	p_it: u32,
	d_it: u32,
	c_it: u32,
	lst: u32,
	ast: u32,
	vst: u32,
	anchor_rigid_bodies: Vec<AnchorRigidBody>,
	vertex_pins: Vec<i32>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Model {
	header: Header,
	vertices: Vec<Vertex>,
	indices: Vec<i32>,
	textures: Vec<String>,
	materials: Vec<Material>,
	morphs: Vec<Morph>,
	frames: Vec<Frame>,
	rigid_bodies: Vec<RigidBody>,
	joints: Vec<Joint>,
	soft_bodies: Vec<SoftBody>,
}

fn index(input: &[u8], size: u8) -> IResult<&[u8],i32> {
	if input.len() < size || size > 4 {
		Err(Err::Incomplete(Needed::Size(size)))
	} else {
		match size {
			1 => map!(input, le_i8, |x| x as i32),
			2 => map!(input, le_i16, |x| x as i32),
			4 => le_i32(input),
		}
	}
}

fn string<'a>(input: &[u8], encoding: Encoding) -> IResult<&[u8],&'a str> {
	match encoding {
		Encoding::UTF16LE => map!(input, length_bytes!(le_u32), |s| UTF_16LE_ENCODING.decode(s, DecoderTrap::Replace)),
		Encoding::UTF8 => map!(input, length_bytes!(le_u32), |s| std:str::from_utf8(s).unwrap_or_default()),
	}
}

named!(settings<Settings>,
	do_parse!(
		settings_count: le_u8 >>
		encoding: switch!(le_u8,
			0 => value!(Encoding::UTF16LE) |
			1 => value!(Encoding::UTF8)
		) >>
		uv: le_u8 >>
		vertex_index_size: le_u8 >>
		texture_index_size: le_u8 >>
		material_index_size: le_u8 >>
		bone_index_size: le_u8 >>
		morph_index_size: le_u8 >>
		rigid_body_index_size: le_u8 >>
		extra: take!(settings_count - 8) >>
		
		(Settings {
			encoding: encoding,
			uv: uv,
			vertex_index_size: vertex_index_size,
			texture_index_size: texture_index_size,
			material_index_size: material_index_size,
			bone_index_size: bone_index_size,
			morph_index_size: morph_index_size,
			rigid_body_index_size: rigid_body_index_size,
		})
	)
);

named!(header<Header>,
	do_parse!(
		tag!("PMX ") >>
		version: le_f32 >>
		settings: settings >>
		name_local: call!(string, settings.encoding) >>
		name_global: call!(string, settings.encoding) >>
		comment_local: call!(string, settings.encoding) >>
		comment_global: call!(string, settings.encoding) >>
		
		(Header {
			version: version,
			settings: settings,
			name_local: name_local,
			name_global: name_global,
			comment_local: comment_local,
			comment_global: comment_global,
		})
	)
);

named!(vector4<Vector4<f32> >,
	do_parse!(
		x: le_f32 >>
		y: le_f32 >>
		z: le_f32 >>
		w: le_f32 >>
		(Vector4::new(x, y, z, w))
	)
);

named_args!(bdef1(input: &[u8], index_size: u8)<Skinning>,
	do_parse!(
		index: call!(index, index_size) >>
		(Skinning::BDef1 {
			index: index,
		})
	)
);

named_args!(bdef2(input: &[u8], index_size: u8)<Skinning>,
	do_parse!(
		indices: count_fixed!(&[u32], call!(index, index_size), 2) >>
		weight: le_f32 >>
		(Skinning::BDef2 {
			indices: indices,
			weight: weight,
		})
	)
);

named_args!(bdef4(input: &[u8], index_size: u8)<Skinning>,
	do_parse!(
		indices: count_fixed!(&[u32], call!(index, index_size), 4) >>
		weights: count_fixed!(&[f32], le_f32, 4) >>
		(Skinning::BDef4 {
			indices: indices,
			weights: weights,
		})
	)
);

named_args!(sdef(input: &[u8], index_size: u8)<Skinning>,
	do_parse!(
		indices: count_fixed!(&[u32], call!(index, index_size), 2) >>
		weight: le_f32 >>
		c: le_v3f >>
		r: count_fixed!(&[Vector3<f32>], le_v3f, 2) >>
		(Skinning::SDef {
			indices: indices,
			weight: weight,
			c: c,
			r: r,
		})
	)
);

named_args!(qdef(input: &[u8], index_size: u8)<Skinning>,
	do_parse!(
		indices: count_fixed!(&[u32], call!(index, index_size), 4) >>
		weights: count_fixed!(&[f32], le_f32, 4) >>
		(Skinning::QDef {
			indices: indices,
			weights: weights,
		})
	)
);

named_args!(vertex(input: &[u8], index_size: u8)<Vertex>,
	do_parse!(
		position: le_v3f >>
		normal: le_v3f >>
		uv: le_v2f >>
		uva: count!(le_v4f, settings.uv) >>
		skinning: switch!(le_u8,
			0 => call!(bdef1, index_size) |
			1 => call!(bdef2, index_size) |
			2 => call!(bdef4, index_size) |
			3 => call!(sdef, index_size) |
			4 => call!(qdef, index_size)
		)
		edge: le_f32 >>
		(Vertex {
			position: position,
			normal: normal,
			uv: uv,
			uva: uva,
			skinning: skinning,
			edge: edge,
		})
	)
);

named_args!(material(input: &[u8], settings: &Settings)<Material>,
	do_parse!(
		name_local: call!(string, settings.encoding) >>
		name_global: call!(string, settings.encoding) >>
		diffuse: le_rgba_f >>
		specular: le_rgb_f >>
		specularity: le_f32 >>
		ambient: le_rgb_f >>
		flags: le_u8 >>
		edge_color: le_rgba_f >>
		edge_scale: le_f32 >>
		texture: call!(index, settings.texture_index_size) >>
		environment: call!(index, settings.texture_index_size) >>
		environment_blend_mode: switch!(le_u8,
			0 => value!(BlendMode::DISABLED) |
			1 => value!(BlendMode::MULTIPLY) |
			2 => value!(BlendMode::ADDITIVE) |
			3 => value!(BlendMode::ADDITIONAL_VEC4)
		) >>
		toon_reference: switch!(le_u8,
			0 => value!(ToonReference::TEXTURE) |
			1 => value!(ToonReference::INTERNAL)
		) >>
		toon_texture: cond!(toon_reference == 0, le_u8) >>
		toon_internal: cond!(toon_texture.is_none(), call!(index, index_size)) >>
		comment: call!(string, settings.encoding) >>
		surface_count: le_u32 >>
		(Material {
			name_local: name_local,
			name_global: name_global,
			diffuse: diffuse,
			specular: specular,
			specularity: specularity,
			ambient: ambient,
			flags: MaterialFlags::from_bytes_truncate(flags),
			edge_color: edge_color,
			edge_scale: edge_scale,
			texture: texture,
			environment: environment,
			environment_blend_mode: environment_blend_mode,
			toon_reference: toon_reference,
			toon: if toon_texture.is_some() {
				toon_texture.unwrap() as i32
			} else {
				toon_internal.unwrap()
			},
			comment: comment,
			surface_count: surface_count,
		})
	)
);

named!(tail_position<TailPosition>,
	do_parse!(
		v3: le_v3f >>
		(TailPosition::Vector3(v3))
	)
);

named_args!(tail_position_indexed(input: &[u8], index_size: u8)<TailPosition>,
	do_parse!(
		index: call!(index, index_size) >>
		(TailPosition::Index(index))
	)
);

named_args!(inherit_bone(input: &[u8], index_size: u8)<InheritBone>,
	do_parse!(
		index: call!(index, index_size) >>
		weight: le_f32 >>
		(InheritBone {
			index: index,
			weight: weight,
		})
	)
);

named!(local_coordinate<LocalCoordinate>,
	do_parse!(
		x: le_v3f >>
		z: le_v3f >>
		(LocalCoordinate {
			x: x,
			z: z,
		})
	)
);

named_args!(link(input: &[u8], index_size: u8)<Link>,
	do_parse!(
		bone_index: call!(index, index_size) >>
		has_limits: le_u8 >>
		limit_count: le_u32 >>
		limits: count!(le_bbf, limit_count) >>
		(Link {
			bone_index: bone_index,
			has_limits: has_limits as bool,
			limits: limits,
		})
	)
);

named_args!(ik(input: &[u8], index_size: u8)<InverseKinematic>,
	do_parse!(
		target: call!(index, index_size) >>
		loop_count: le_u32 >>
		angle_limit: le_f32 >>
		link_count: le_u32 >>
		links: count!(link, link_count) >>
		(InverseKinematic {
			target: target,
			loop_count: loop_count,
			angle_limit: angle_limit,
			links: links,
		})
	)
);

named_args!(bone(input: &[u8], settings: &Settings)<Bone>,
	do_parse!(
		name_local: call!(string, settings.encoding) >>
		name_global: call!(string, settings.encoding) >>
		position: le_v3f >>
		parent: le_i32 >>
		layer: le_i32 >>
		flags: le_u16 >>
		tail_position: cond!((flags & 1) == 0, tail_position) >>
		tail_position_indexed: cond!((flags & 1) != 0, call!(tail_position_indexed, settings.bone_index_size)) >>
		inherit_bone: cond!((flags & (256 | 512)) != 0, call!(inherit_bone, settings.bone_index_size)) >>
		fixed_axis: cond!((flags & 1024) != 0, le_v3f) >>
		local_coordinate: cond!((flags & 2048) != 0, local_coordinate) >>
		key: cond!((flags & 8192) != 0, le_i32) >>
		ik: cond!((flags & 32) != 0, call!(ik, settings.bone_index_size)) >>
		(Bone {
			name_local: name_local,
			name_global: name_global,
			position: position,
			parent: parent,
			layer: layer,
			flags: BoneFlags::from_bytes_truncate(flags),
			tail_position: if tail_position.is_some() {
				tail_position.unwrap()
			} else {
				tail_position_indexed.unwrap()
			},
			inherit_bone: inherit_bone,
			fixed_axis: fixed_axis,
			local_coordinate: local_coordinate,
			key: key,
			ik: ik,
		})
	)
);

named!(pub(crate) model<Model>,
	do_parse!(
		header: header >>
		vertex_count: le_u32 >>
		vertices: count!(call!(vertex, header.settings.bone_index_size), vertex_count) >>
		index_count: le_u32 >>
		indices: count!(call!(index, header.settings.vertex_index_size), index_count) >>
		texture_count: le_u32 >>
		textures: count!(call!(string, header.settings.encoding), texture_count) >>
		material_count: le_u32 >>
		materials: count!(call!(material, &header.settings), material_count) >>
		bone_count: le_u32 >>
		bones: count!(call!(bone, &header.settings), bone_count) >>
);

fn determine_index_size(count: u32) -> u8 {
	if count < (u8::max_value() as u32) { return 1 }
	if count < ((u16::max_value() as u32) - 1) { return 2 }
	4
}

pub fn to_pmx(scn: &Scene, enc: Encoding) -> Result<()> {
	let mut buf = vec![];
	
	buf.put(b"PMX ");
	buf.put_f32_le(2.0); // version
	buf.put_u8(8); // settings count
	buf.put_u8(enc as u8);
	// todo: index size data
	
	if enc == UTF16LE {
		
		UTF_16LE_ENCODING.encode_to(&scn.name.text, EncoderTrap::Replace);
	} else {
	}
}

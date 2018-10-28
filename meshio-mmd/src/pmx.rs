extern crate bitflags;
extern crate encoding;
extern crate nom;

use bitflags::bitflags;

use nom::{
	call,
	cond,
	count,
	count_fixed,
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
	DecoderTrap,
	UTF_16LE_ENCODING
};

pub mod pmx {
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum Encoding {
		UTF16LE,
		UTF8,
	}

	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Settings {
		pub encoding: Encoding,
		pub uv: u8,
		pub vertex_index_size: u8,
		pub texture_index_size: u8,
		pub material_index_size: u8,
		pub bone_index_size: u8,
		pub morph_index_size: u8,
		pub rigid_body_index_size: u8,
	}

	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Header {
		pub version: f32,
		pub settings: Settings,
		pub name_local: String,
		pub name_global: String,
		pub comment_local: String,
		pub comment_global: String,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum Skinning {
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
			c: [f32; 3],
			r: [[f32; 3]; 2]
		},
		QDef {
			indices: [i32; 4],
			weights: [f32; 4]
		}
	}

	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Vertex {
		pub position: [f32; 3],
		pub normal: [f32; 3],
		pub uv: [f32; 2],
		pub uva: Vec<[f32; 4]>,
		pub skinning: Skinning,
		pub edge: f32,
	}
	
	bitflags! {
		pub struct MaterialFlags: u8 {
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
	pub enum BlendMode {
		DISABLED,
		MULTIPLY,
		ADDITIVE,
		ADDITIONAL_VEC4,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum ToonReference {
		TEXTURE,
		INTERNAL,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Material {
		pub name_local: String,
		pub name_global: String,
		pub diffuse: [f32; 4],
		pub specular: [f32; 3],
		pub specularity: f32,
		pub ambient: [f32; 3],
		pub flags: MaterialFlags,
		pub edge_color: [f32; 4],
		pub edge_scale: f32,
		pub texture: i32,
		pub environment: i32,
		pub environment_blend_mode: BlendMode,
		pub toon_reference: ToonReference,
		pub toon: i32,
		pub comment: String,
		pub surface_count: u32,
	}
	
	bitflags! {
		pub struct BoneFlags: u16 {
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
	pub enum TailPosition {
		Vec3([f32; 3]),
		Index(i32),
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct InheritBone {
		pub index: i32,
		pub weight: f32,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct LocalCoordinate {
		pub x: [f32; 3],
		pub z: [f32; 3],
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Limit {
		pub min: [f32; 3],
		pub max: [f32; 3],
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Link {
		bone_index: i32,
		has_limits: bool,
		limits: Option<Vec<Limit>>,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct InverseKinematic {
		pub target: i32,
		pub loop_count: u32,
		pub angle_limit: f32,
		pub links: Option<Vec<Link>>,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Bone {
		pub name_local: String,
		pub name_global: String,
		pub position: [f32; 3],
		pub parent: i32,
		pub layer: i32,
		pub flags: BoneFlags,
		pub tail_position: TailPosition,
		pub inherit_bone: Option<InheritBone>,
		pub fixed_axis: Option<[f32; 3]>,
		pub local_coordinate: Option<LocalCoordinate>,
		pub key: Option<i32>,
		pub ik: Option<InverseKinematic>,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum Category {
		RESERVED,
		EYEBROW,
		EYE,
		MOUTH,
		OTHER,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum MorphType {
		GROUP,
		VERTEX,
		BONE,
		UV,
		ADDITIONAL_UV1,
		ADDITIONAL_UV2,
		ADDITIONAL_UV3,
		ADDITIONAL_UV4,
		MATERIAL,
		FLIP,
		IMPULSE,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum Offset {
		Group {
			index: i32,
			weight: f32,
		},
		Vertex {
			index: i32,
			translation: [f32; 3],
		},
		Bone {
			index: i32,
			translation: [f32; 3],
			rotation: [f32; 4],
		},
		Uv {
			index: i32,
			offsets: [f32; 4],
		},
		Material {
			index: i32,
			operation: u8,
			diffuse: [f32; 4],
			specular: [f32; 3],
			specularity: f32,
			ambient: [f32; 3],
			edge_color: [f32; 4],
			edge_scale: f32,
			texture_tint: [f32; 4],
			environment_tint: [f32; 4],
			toon_tint: [f32; 4],
		},
		Flip {
			index: i32,
			weight: f32,
		},
		Impulse {
			index: i32,
			is_local: bool,
			speed: [f32; 3],
			torque: [f32; 3],
		},
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Morph {
		pub name_local: String,
		pub name_global: String,
		pub category: Category,
		pub kind: MorphType,
		pub offsets: Vec<Offset>,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum FrameFlag {
		NORMAL,
		SPECIAL,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum FrameType {
		BONE,
		MORPH
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum FrameElement {
		pub kind: FrameType,
		pub index: i32,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Frame {
		pub name_local: String,
		pub name_global: String,
		pub flag: FrameFlag,
		pub elements: Vec<FrameElement>,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum RigidShape {
		SPHERE,
		BOX,
		CAPSULE,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum Physics {
		FOLLOW_BONE,
		GRAVITY,
		GRAVITY_BONE,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct RigidBody {
		pub name_local: String,
		pub name_global: String,
		pub bone: i32,
		pub group: u8,
		pub mask: u16,
		pub shape: RigidShape,
		pub scale: [f32; 3],
		pub position: [f32; 3],
		pub rotation: [f32; 3],
		pub mass: f32,
		pub attenuation: f32,
		pub damping: f32,
		pub repulsion: f32,
		pub friction: f32,
		pub mode: Physics,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum JointType {
		SPRING_6DOF,
		SIX_DOF,
		P2P,
		CONE_TWIST,
		SLIDER,
		HINGE,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Joint {
		pub name_local: String,
		pub name_global: String,
		pub kind: JointType,
		pub indices: [i32; 2],
		pub position: [f32; 3],
		pub rotation: [f32; 3],
		pub position_min: [f32; 3],
		pub position_max: [f32; 3],
		pub rotation_min: [f32; 3],
		pub rotation_max: [f32; 3],
		pub position_spring: [f32; 3],
		pub rotation_spring: [f32; 3],
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum SoftShape {
		TRI_MESH,
		ROPE,
	}
	
	bitflags! {
		pub struct SoftBodyFlags: u8 {
			const B_LINK = 1;
			const CLUSTER_CREATION = 2;
			const LINK_CROSSING = 4;
		}
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum Aerodynamics {
		V_POINT,
		V_2SIDED,
		V_1SIDED,
		F_2SIDED,
		F_1SIDED,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct AnchorRigidBody {
		pub rigid_body: i32,
		pub vertex: i32,
		pub near_mode: u8,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct SoftBody {
		pub name_local: String,
		pub name_global: String,
		pub shape: Shape,
		pub material: i32,
		pub group: u8,
		pub mask: u16,
		pub flags: SoftBodyFlags,
		pub b_link_create_distance: u32,
		pub cluster_count: u32,
		pub total_mass: f32,
		pub collision_margin: f32,
		pub aerodynamics: Aerodynamics,
		pub vcf: f32,
		pub dp: f32,
		pub dg: f32,
		pub lf: f32,
		pub pr: f32,
		pub vc: f32,
		pub df: f32,
		pub mt: f32,
		pub chr: f32,
		pub khr: f32,
		pub shr: f32,
		pub ahr: f32,
		pub srhr_cl: f32,
		pub skhr_cl: f32,
		pub sshr_cl: f32,
		pub sr_splt_cl: f32,
		pub sk_splt_cl: f32,
		pub ss_splt_cl: f32,
		pub v_it: u32,
		pub p_it: u32,
		pub d_it: u32,
		pub c_it: u32,
		pub lst: u32,
		pub ast: u32,
		pub vst: u32,
		pub anchor_rigid_bodies: Vec<AnchorRigidBody>,
		pub vertex_pins: Vec<i32>,
	}

	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Model {
		pub header: Header,
		pub vertices: Vec<Vertex>,
		pub indices: Vec<i32>,
		pub textures: Vec<String>,
		pub materials: Vec<Material>,
		pub morphs: Vec<Morph>,
		pub frames: Vec<Frame>,
		pub rigid_bodies: Vec<RigidBody>,
		pub joints: Vec<Joint>,
		pub soft_bodies: Vec<SoftBody>,
	}
	
	fn idx(input: &[u8], size: u8) -> IResult<&[u8],i32> {
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
	
	fn string(input: &[u8], encoding: Encoding) -> IResult<&[u8],String> {
		match encoding {
			Encoding::UTF16LE => map!(input, length_bytes!(le_u32), |s| UTF_16LE_ENCODING.decode(s, DecoderTrap::Replace)),
			Encoding::UTF8 => map!(input, length_bytes!(le_u32), |s| String::from_utf8(s).unwrap_or_default()),
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
	)
	
	named!(header<Header>,
		do_parse!(
			tag!("PMX ") >>
			version: le_f32 >>
			settings: settings >>
			name_local: call!(string, settings.encoding) >>
			name_global: call!(string, settings.encoding) >>
			comment_local: call!(string, settings.encoding)) >>
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
	)
	
	named_args!(bdef1(input: &[u8], index_size: u8)<Skinning>,
		do_parse!(
			index: call!(idx, index_size) >>
			(Skinning::BDef1 {
				index: index,
			})
		)
	)
	
	named_args!(bdef2(input: &[u8], index_size: u8)<Skinning>,
		do_parse!(
			indices: count_fixed!(&[u32], call!(idx, index_size), 2) >>
			weight: le_f32 >>
			(Skinning::BDef2 {
				indices: indices,
				weight: weight,
			})
		)
	)
	
	named_args!(bdef4(input: &[u8], index_size: u8)<Skinning>,
		do_parse!(
			indices: count_fixed!(&[u32], call!(idx, index_size), 4) >>
			weights: count_fixed!(&[f32], le_f32, 4) >>
			(Skinning::BDef4 {
				indices: indices,
				weights: weights,
			})
		)
	)
	
	named_args!(sdef(input: &[u8], index_size: u8)<Skinning>,
		do_parse!(
			indices: count_fixed!(&[u32], call!(idx, index_size), 2) >>
			weight: le_f32 >>
			c: count_fixed!(&[f32], le_f32, 3) >>
			r: count_fixed!(&[&[f32]], count_fixed!(&[f32], le_f32, 3), 2) >>
			(Skinning::SDef {
				indices: indices,
				weight: weight,
				c: c,
				r: r,
			})
		)
	)
	
	named_args!(qdef(input: &[u8], index_size: u8)<Skinning>,
		do_parse!(
			indices: count_fixed!(&[u32], call!(idx, index_size), 4) >>
			weights: count_fixed!(&[f32], le_f32, 4) >>
			(Skinning::QDef {
				indices: indices,
				weights: weights,
			})
		)
	)
	
	named_args!(vertex(input: &[u8], index_size: u8)<Vertex>,
		do_parse!(
			position: count_fixed!(&[f32], le_f32, 3) >>
			normal: count_fixed!(&[f32], le_f32, 3) >>
			uv: count_fixed!(&[f32], le_f32, 2) >>
			uva: count!(count_fixed!(&[f32], le_f32, 4), settings.uv) >>
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
	)
	
	named_args!(material(input: &[u8], settings: &Settings)<Material>,
		do_parse!(
			name_local: call!(string, settings.encoding) >>
			name_global: call!(string, settings.encoding) >>
			diffuse: count_fixed!(&[f32], le_f32, 4) >>
			specular: count_fixed!(&[f32], le_f32, 3) >>
			specularity: le_f32 >>
			ambient: count_fixed!(&[f32], le_f32, 3) >>
			flags: le_u8 >>
			edge_color: count_fixed!(&[f32], le_f32, 4) >>
			edge_scale: le_f32 >>
			texture: call!(idx, settings.texture_index_size) >>
			environment: call!(idx, settings.texture_index_size) >>
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
			toon_internal: cond!(toon_texture.is_none(), call!(idx, index_size)) >>
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
			vec3: count_fixed!(&[f32], le_f32, 3) >>
			(TailPosition::Vec3(vec3))
		)
	);
	
	named_args!(tail_position_indexed(input: &[u8], index_size: u8)<TailPosition>,
		do_parse!(
			index: call!(idx, index_size) >>
			(TailPosition::Index(index))
		)
	);
	
	named_args!(inherit_bone(input: &[u8], index_size: u8)<InheritBone>,
		do_parse!(
			index: call!(idx, index_size) >>
			weight: le_f32 >>
			(InheritBone {
				index: index,
				weight: weight,
			})
		)
	);
	
	named!(local_coordinate<LocalCoordinate>,
		do_parse!(
			x: count_fixed!(&[f32], le_f32, 3) >>
			z: count_fixed!(&[f32], le_f32, 3) >>
			(LocalCoordinate {
				x: x,
				z: z,
			})
		)
	);
	
	named!(limit<Limit>,
		do_parse!(
			min: count_fixed!(&[f32], le_f32, 3) >>
			max: count_fixed!(&[f32], le_f32, 3) >>
			(Limit {
				min: min,
				max: max,
			})
		)
	);
	
	named_args!(link(input: &[u8], index_size: u8)<Link>,
		do_parse!(
			bone_index: call!(idx, index_size) >>
			has_limits: le_u8 >>
			limit_count: le_u32 >>
			limits: count!(limit, limit_count) >>
			(Link {
				bone_index: bone_index,
				has_limits: has_limits as bool,
				limits: limits,
			})
		)
	);
	
	named_args!(ik(input: &[u8], index_size: u8)<InverseKinematic>,
		do_parse!(
			target: call!(idx, index_size) >>
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
			position: count_fixed!(&[f32], le_f32, 3) >>
			parent: le_i32 >>
			layer: le_i32 >>
			flags: le_u16 >>
			tail_position: cond!((flags & 1) == 0, tail_position) >>
			tail_position_indexed: cond!((flags & 1) != 0, call!(tail_position_indexed, settings.bone_index_size)) >>
			inherit_bone: cond!((flags & (256 | 512)) != 0, call!(inherit_bone, settings.bone_index_size)) >>
			fixed_axis: cond!((flags & 1024) != 0, count_fixed!(&[f32], le_f32, 3)) >>
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

	named!(pub model<Model>,
		do_parse!(
			header: header >>
			vertex_count: le_u32 >>
			vertices: count!(call!(vertex, header.settings.bone_index_size), vertex_count) >>
			index_count: le_u32 >>
			indices: count!(call!(idx, header.settings.vertex_index_size), index_count) >>
			texture_count: le_u32 >>
			textures: count!(call!(string, header.settings.encoding), texture_count) >>
			material_count: le_u32 >>
			materials: count!(call!(material, &header.settings), material_count) >>
			bone_count: le_u32 >>
			bones: count!(call!(bone, &header.settings), bone_count) >>
	);
}

use bitflags::bitflags;

use cgmath::{
	Matrix4,
	Quaternion,
	Vector2,
	Vector3,
	Vector4
};

use nom::{
	call,
	do_parse,
	le_f32,
	le_i32,
	le_u8,
	le_u16,
	le_u32,
	le_u64,
	named,
	named_args,
	switch,
	value
};

use meshio::{
	BoundsF,
	ColorF
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Reference {
	entries: u32,
	index: u32,
	flags: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u16)]
enum Interpolation {
	CONSTANT = 0;
	LINEAR = 1;
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct AnimationReferenceHeader {
	interpolation: Interpolation,
	anim_flags: u16,
	anim_id: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum AnimationReference {
	Vector3 {
		header: AnimationReferenceHeader,
		init: Vector3<f32>,
		null: Vector3<f32>,
		unknown: i32,
	},
	Vector2 {
		header: AnimationReferenceHeader,
		init: Vector2<f32>,
		null: Vector2<f32>,
		unknown: i32,
	},
	Quaternion {
		header: AnimationReferenceHeader,
		init: Quaternion<f32>,
		null: Quaternion<f32>,
		unknown: i32,
	},
	UInt32 {
		header: AnimationReferenceHeader,
		init: u32,
		null: u32,
		unknown: i32,
	},
	UInt16 {
		header: AnimationReferenceHeader,
		init: u16,
		null: u16,
		unknown: i32,
	},
	Int16 {
		header: AnimationReferenceHeader,
		init: i16,
		null: i16,
		unknown: i32,
	},
	UInt8 {
		header: AnimationReferenceHeader,
		init: u8,
		null: u8,
		unknown: i32,
	},
	Float {
		header: AnimationReferenceHeader,
		init: f32,
		null: f32,
		unknown: i32,
	},
	Unknown4Byte {
		header: AnimationReferenceHeader,
		init: [u8; 4],
		null: [u8; 4],
		unknown: i32,
	},
	Color {
		header: AnimationReferenceHeader,
		init: ColorF,
		null: ColorF,
		unknown: i32,
	},
	BoundsF {
		header: AnimationReferenceHeader,
		init: BoundsF,
		null: BoundsF,
		unknown: i32,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct IndexEntry {
	tag: [u8; 4],
	offset: u32,
	repetitions: u32,
	version: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Vertex {
	Format182007D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		uv: Vector2<i16>,
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format1820261 {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		color: ColorF,
		uv: Vector2<i16>,
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format182027D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		color: ColorF,
		uv: Vector2<i16>,
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format1860061 {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		uvs: [Vector2<i16>; 2],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format186007D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		uvs: [Vector2<i16>; 2],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format1860261 {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		color: ColorF,
		uvs: [Vector2<i16>; 2],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format186027D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		color: ColorF,
		uvs: [Vector2<i16>; 2],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format18E0061 {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		uvs: [Vector2<i16>; 3],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format18E007D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		uvs: [Vector2<i16>; 3],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format18E0261 {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		color: ColorF,
		uvs: [Vector2<i16>; 3],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format18E027D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		color: ColorF,
		uvs: [Vector2<i16>; 3],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format19E0061 {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		uvs: [Vector2<i16>; 4],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format19E007D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		uvs: [Vector2<i16>; 4],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format19E0261 {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		color: ColorF,
		uvs: [Vector2<i16>; 4],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format19E027D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		color: ColorF,
		uvs: [Vector2<i16>; 4],
		tangent: Vector3<u8>,
		unused: u8,
	},
	Format4182007D {
		position: Vector3<f32>,
		bone_weights: [u8; 4],
		bone_lookup_indices: [u8; 4],
		normal: Vector3<u8>,
		sign: u8,
		uvs: [Vector2<i16>; 2],
		tangent: Vector3<u8>,
		unused: u8,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum EventData {
	V0 {
		name: Reference,
		unknown_76ff2940: i32,
		unknown_97b2ea12: i16,
		unknown_294cc515: u16,
		matrix: Matrix4<f32>,
		unknown_7275bfa0: u32,
		unknown_a20de85: u32,
		unknown_76cf1db0: u32,
	},
	V1 {
		name: Reference,
		unknown_76ff2940: i32,
		unknown_97b2ea12: i16,
		unknown_294cc515: u16,
		matrix: Matrix4<f32>,
		unknown_7275bfa0: u32,
		unknown_2a403b40: Reference,
		unknown_76cf1db0: u32,
	},
	V2 {
		name: Reference,
		unknown_76ff2940: i32,
		unknown_97b2ea12: i16,
		unknown_294cc515: u16,
		matrix: Matrix4<f32>,
		unknown_7275bfa0: u32,
		unknown_2a403b40: Reference,
		unknown_76cf1db0: u32,
		unknown_e5f1b2a7: u32,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Event {
	data: EventData,
	name: String,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct AnimationBlockData {
	frames: Reference,
	flags: u32,
	fend: u32,
	keys: Reference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum AnimationBlockKeys {
	Event(Vec<Event>),
	Vector2(Vec<Vector2<f32>>),
	Vector3(Vec<Vector3<f32>>),
	Real(Vec<f32>),
	Color(Vec<ColorF>),
	Int16(Vec<i16>),
	UInt16(Vec<u16>)
	UInt32(Vec<u32>),
	Quaternion(Vec<Quaternion<f32>>),
	Flag(Vec<u32>),
	BoundsF(Vec<BoundsF>),
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct AnimationBlock {
	data: AnimationBlockData,
	frames: Vec<i32>,
	keys: AnimationBlockKeys,
}

bitflags! {
	struct BoneFlags: u32 {
		const INHERIT_TRANSLATION = 1;
		const INHERIT_SCALE = 2;
		const INHERIT_ROTATION = 4;
		const BILLBOARD1 = 16;
		const BILLBOARD2 = 64;
		const TWO_D_PROJECTION = 256;
		const ANIMATED = 512;
		const INVERSE_KINEMATICS = 1024;
		const SKINNED = 2048;
		const REAL = 8192;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BoneData {
	d: i32,
	name: Reference,
	flags: BoneFlags,
	parent: i16,
	s: u16,
	location: AnimationReference,
	rotation: AnimationReference,
	scale: AnimationReference,
	ar: AnimationReference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Bone {
	data: BoneData,
	name: String,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct SequenceTransformCollectionData {
	name: Reference,
	runs_concurrent: u16,
	priority: u16,
	sts_index: u16,
	sts_index_copy: u16,
	anim_ids: Reference,
	anim_refs: Reference,
	unknown44: u32,
	sdev: Reference,
	sd2v: Reference,
	sd3v: Reference,
	sd4q: Reference,
	sdcc: Reference,
	sdr3: Reference,
	unknown_ref8: Reference,
	sds6: Reference,
	sdu6: Reference,
	unknown_ref11: Reference,
	sdu3: Reference,
	sdfg: Reference,
	sdmb: Reference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct SequenceTransformCollection {
	data: SequenceTransformCollectionData,
	name: String,
	animation_ids: Vec<u32>,
	animation_refs: Vec<u32>,
	sdev: AnimationBlock,
	sd2v: AnimationBlock,
	sd3v: AnimationBlock,
	sd4q: AnimationBlock,
	sdcc: AnimationBlock,
	sdr3: AnimationBlock,
	sds6: AnimationBlock,
	sdu6: AnimationBlock,
	sdu3: AnimationBlock,
	sdfg: AnimationBlock,
	sdmb: AnimationBlock,
}

bitflags! {
	struct SequenceFlags: u32 {
		const NOT_LOOPING = 1;
		const ALWAYS_GLOBAL = 2;
		const GLOBAL_IN_PREVIEWER = 8;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum SequenceData {
	V1 {
		unknown0: i32,
		unknown1: i32,
		name: Reference,
		anim_start_in_ms: u32,
		anim_end_in_ms: u32,
		movement_speed: f32,
		flags: SequenceFlags,
		frequency: u32,
		unknown2: u32,
		unknown3: u32,
		unknown4: u32,
		unknown5: u32,
		bounding_sphere: BoundsF,
		unknown6: u32,
		unknown7: u32,
		unknown8: u32,
	},
	V2 {
		unknown0: i32,
		unknown1: i32,
		name: Reference,
		anim_start_in_ms: u32,
		anim_end_in_ms: u32,
		movement_speed: f32,
		flags: SequenceFlags,
		frequency: u32,
		unknown2: u32,
		unknown3: u32,
		unknown4: u32,
		bounding_sphere: BoundsF,
		unknown6: u32,
		unknown7: u32,
		unknown8: u32,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Sequence {
	data: SequenceData,
	name: String,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct SequenceTransformGroupData {
	name: Reference,
	stc_indices: Reference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct SequenceTransformGroup {
	data: SequenceTransformGroupData,
	name: String,
	stc_indices: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Sts {
	anim_ids: Reference,
	unknown0: i32,
	unknown1: i32,
	unknown2: i32,
	s1: i16,
	s2: u16,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Region {
	V3 {
		unknown0: u32,
		unknown1: u32,
		first_vertex_index: u32,
		vertices: u32,
		first_face_vertex_index_index: u32,
		face_vertex_indices: u32,
		bones: u16,
		first_bone_lookup_index: u16,
		bone_lookup_indices: u16,
		unknown2: u16,
		bone_weight_pairs_per_vertex: u8,
		unknown4: u8,
		root_bone_index: u16,
	},
	V4 {
		unknown0: u32,
		unknown1: u32,
		first_vertex_index: u32,
		vertices: u32,
		first_face_vertex_index_index: u32,
		face_vertex_indices: u32,
		bones: u16,
		first_bone_lookup_index: u16,
		bone_lookup_indices: u16,
		unknown2: u16,
		bone_weight_pairs_per_vertex: u8,
		unknown4: u8,
		root_bone_index: u16,
		unknown5: u32,
	},
	V5 {
		unknown0: u32,
		unknown1: u32,
		first_vertex_index: u32,
		vertices: u32,
		first_face_vertex_index_index: u32,
		face_vertex_indices: u32,
		bones: u16,
		first_bone_lookup_index: u16,
		bone_lookup_indices: u16,
		unknown2: u16,
		bone_weight_pairs_per_vertex: u8,
		unknown4: u8,
		root_bone_index: u16,
		unknown5: u32,
		unknown_4d066bda: [u8; 8],
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Bat {
	unknown0: u32,
	region_index: u16,
	unknown1: u32,
	material_reference_index: u16,
	unknown2: i16,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Msec {
	unknown0: u32,
	boundings_animation: AnimationReference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct DivisionData {
	faces: Reference,
	regions: Reference,
	objects: Reference,
	msec: Reference,
	unknown: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Division {
	data: DivisionData,
	faces: Vec<u16>,
	regions: Vec<Region>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct AttachmentData {
	unknown: i32,
	name: Reference,
	bone: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Attachment {
	data: AttachmentData,
	name: String,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u8)]
enum LightType {
	DIRECTIONAL = 0,
	POINT = 1,
	SPOT = 2,
}

bitflags! {
	struct LightFlags: u32 {
		const SHADOW_CAST = 1;
		const SPECULAR = 2;
		const TURN_ON = 8;
		const APPLY_ON_TRANSPARENT_OBJECTS = 16;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Light {
	kind: LightType,
	unknown1: u8,
	bone_index: u16,
	flags: LiteFlags,
	unknown8: u32,
	unknown12: i32,
	color: AnimationReference,
	intensity: AnimationReference,
	spec_color: AnimationReference,
	spec_intensity: AnimationReference,
	attenuation_far: AnimationReference,
	unknown148: f32,
	attenuation_near: AnimationReference,
	hot_spot: AnimationReference,
	falloff: AnimationReference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct MaterialReference {
	kind: u32,
	index: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct TurrentPart {
	unknown_5c3999e5: f32,
	unknown_a3674aee: f32,
	unknown_dd0cc5f1: f32,
	unknown_6dee953c: f32,
	unknown_e3e2341f: f32,
	unknown_36188266: f32,
	unknown_f95a6198: f32,
	unknown_11a1536b: f32,
	unknown_1975afed: f32,
	unknown_de3466a8: f32,
	unknown_f69102f9: f32,
	unknown_71538f05: [u8; 16],
	unknown_a38ff662: f32,
	unknown_b2fb85a2: f32,
	unknown_aaff49cf: f32,
	unknown_fdfde349: f32,
	unknown_978ba241: f32,
	unknown_chen: f32,
	unknown_e2d91749: [u8; 4],
	unknown_4fc95268: [u8; 4],
	unknown_61840ebc: f32,
	bone_index: u32,
	unknown_142bb958: u32,
	unknown_ea9c2019: f32,
	unknown_c40c57a4: f32,
	unknown_698c931b: f32,
	unknown_175074c7: u32,
	unknown_d3d84176: f32,
	unknown_466cb20b: f32,
	unknown_a9a5325e: f32,
	unknown_19c50495: f32,
	unknown_aaefeecf: f32,
	unknown_63b2ca3b: f32,
	unknown_19193ff0: f32,
	unknown_d32f6ee7: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct TurrentBehaviorData {
	part_index: Reference,
	name: Reference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct TurrentBehavior {
	data: TurrentBehaviorData,
	parts: Vec<u32>,
	name: String,
}

bitflags! {
	struct LayerFlags: u32 {
		const TEXTURE_WRAP_X = 4;
		const TEXTURE_WRAP_Y = 8;
		const INVERT_COLOR = 16;
		const CLAMP_COLOR = 32;
		const USE_PARTICLE_FLIPBOOK = 256;
		const IS_VIDEO = 512;
		const COLOR_ENABLED = 1024;
		const IGNORED_FRESNEL_FLAG1 = 16384;
		const IGNORED_FRESNEL_FLAG2 = 32768;
		const FRESNEL_LOCAL_TRANSFORM = 65536;
		const FRESNEL_DO_NOT_MIRROR = 131072;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum ColorChannelSetting {
	RGB = 0,
	RGBA = 1,
	ALPHA_ONLY = 2,
	RED_ONLY = 3,
	GREEN_ONLY = 4,
	BLUE_ONLY = 5,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum VideoMode {
	LOOP = 0,
	HOLD = 1,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum LayerData {
	V22 {
		unknown_2daf9c00: u32,
		image_path: Reference,
		color: AnimationReference,
		flags: LayerFlags,
		uv_source: u32,
		color_channel_setting: ColorChannelSetting,
		bright_mult: AnimationReference,
		midtone_offset: AnimationReference,
		unknown_3b61017a: u32,
		rtt_channel: i32,
		video_frame_rate: u32,
		video_start_frame: u32,
		video_end_frame: i32,
		video_mode: VideoMode,
		video_sync_timing: u32,
		video_play: AnimationReference,
		video_restart: AnimationReference,
		flip_book_rows: u32,
		flip_book_columns: u32,
		flip_book_frame: AnimationReference,
		uv_offset: AnimationReference,
		uv_angle: AnimationReference,
		uv_tiling: AnimationReference,
		unknown_a4ec0796: AnimationReference,
		unknown_a44bf452: AnimationReference,
		brightness: AnimationReference,
		unknown_bd3f7b5d: i32,
		fresnel_type: u32,
		fresnel_exponent: f32,
		fresnel_min: f32,
		fresnel_max: f32,
		unknown15: f32,
	},
	V24 {
		unknown_2daf9c00: u32,
		image_path: Reference,
		color: AnimationReference,
		flags: LayerFlags,
		uv_source: u32,
		color_channel_setting: ColorChannelSetting,
		bright_mult: AnimationReference,
		midtone_offset: AnimationReference,
		unknown_3b61017a: u32,
		noise_amp: f32,
		noise_freq: f32,
		rtt_channel: i32,
		video_frame_rate: u32,
		video_start_frame: u32,
		video_end_frame: i32,
		video_mode: VideoMode,
		video_sync_timing: u32,
		video_play: AnimationReference,
		video_restart: AnimationReference,
		flip_book_rows: u32,
		flip_book_columns: u32,
		flip_book_frame: AnimationReference,
		uv_offset: AnimationReference,
		uv_angle: AnimationReference,
		uv_tiling: AnimationReference,
		unknown_a4ec0796: AnimationReference,
		unknown_a44bf452: AnimationReference,
		brightness: AnimationReference,
		tri_planar_offset: AnimationReference,
		tri_planar_scale: AnimationReference,
		unknown_bd3f7b5d: i32,
		fresnel_type: u32,
		fresnel_exponent: f32,
		fresnel_min: f32,
		fresnel_max: f32,
		unknown15: f32,
	},
	V25 {
		unknown_2daf9c00: u32,
		image_path: Reference,
		color: AnimationReference,
		flags: LayerFlags,
		uv_source: u32,
		color_channel_setting: ColorChannelSetting,
		bright_mult: AnimationReference,
		midtone_offset: AnimationReference,
		unknown_3b61017a: u32,
		noise_amp: f32,
		noise_freq: f32,
		rtt_channel: i32,
		video_frame_rate: u32,
		video_start_frame: u32,
		video_end_frame: i32,
		video_mode: VideoMode,
		video_sync_timing: u32,
		video_play: AnimationReference,
		video_restart: AnimationReference,
		flip_book_rows: u32,
		flip_book_columns: u32,
		flip_book_frame: AnimationReference,
		uv_offset: AnimationReference,
		uv_angle: AnimationReference,
		uv_tiling: AnimationReference,
		unknown_a4ec0796: AnimationReference,
		unknown_a44bf452: AnimationReference,
		brightness: AnimationReference,
		tri_planar_offset: AnimationReference,
		tri_planar_scale: AnimationReference,
		unknown_bd3f7b5d: i32,
		fresnel_type: u32,
		fresnel_exponent: f32,
		fresnel_min: f32,
		fresnel_max: f32,
		unknown15: f32,
		unknown_763da0ad: [u8; 8],
		fresnel_inverted_mask: Vector3<f32>,
		fresnel_rotation_yaw: f32,
		fresnel_rotation_pitch: f32,
		unknown_ed0e748d: u32,
	},
	V26 {
		unknown_2daf9c00: u32,
		image_path: Reference,
		color: AnimationReference,
		flags: LayerFlags,
		uv_source: u32,
		color_channel_setting: ColorChannelSetting,
		bright_mult: AnimationReference,
		midtone_offset: AnimationReference,
		unknown_3b61017a: u32,
		noise_amp: f32,
		noise_freq: f32,
		rtt_channel: i32,
		video_frame_rate: u32,
		video_start_frame: u32,
		video_end_frame: i32,
		video_mode: VideoMode,
		video_sync_timing: u32,
		video_play: AnimationReference,
		video_restart: AnimationReference,
		flip_book_rows: u32,
		flip_book_columns: u32,
		flip_book_frame: AnimationReference,
		uv_offset: AnimationReference,
		uv_angle: AnimationReference,
		uv_tiling: AnimationReference,
		unknown_a4ec0796: AnimationReference,
		unknown_a44bf452: AnimationReference,
		brightness: AnimationReference,
		tri_planar_offset: AnimationReference,
		tri_planar_scale: AnimationReference,
		unknown_bd3f7b5d: i32,
		fresnel_type: u32,
		fresnel_exponent: f32,
		fresnel_min: f32,
		fresnel_max: f32,
		unknown15: f32,
		unknown_763da0ad: [u8; 8],
		fresnel_inverted_mask: Vector3<f32>,
		fresnel_rotation_yaw: f32,
		fresnel_rotation_pitch: f32,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Layer {
	data: LayerData,
	image_path: String,
}

bitflags! {
	struct MaterialFlags: u64 {
		const USE_VERTEX_COLOR = 1;
		const USE_VERTEX_ALPHA = 2;
		const UNFOGGED = 4;
		const TWO_SIDED = 8;
		const UNSHADED = 16;
		const NO_SHADOWS_CAST = 32;
		const NO_HIT_TEST = 64;
		const NO_SHADOWS_RECEIVED = 128;
		const DEPTH_PREPASS = 256;
		const USE_TERRAIN_HDR = 512;
		const SPLAT_UV_FIX = 2048;
		const SOFT_BLENDING = 4096;
		const FOR_PARTICLES = 16384;
		const TRANSPARENCY = 65536;
		const DISABLE_SOFT = 262144;
		const DARK_NORMAL_MAPPING = 524288;
		const HAIR_LAYER_SORTING = 1048576;
		const ACCEPT_SPLATS_ONLY = 2097152;
		const DECAL_REQUIRED_ON_LOW_END = 4194304;
		const EMISSIVE_REQUIRED_ON_LOW_END = 8388608;
		const SPECULAR_REQUIRED_ON_LOW_END = 16777216;
		const ACCEPT_SPLATS = 33554432;
		const BACKGROUND_OBJECT = 67108864;
		const ZP_FILL_REQUIRED_ON_LOW_END = 268435456;
		const EXCLUDE_FROM_HIGHLIGHTING = 536870912;
		const CLAMP_OUTPUT = 1073741824;
		const GEOMETRY_VISIBLE = 2147483648;
		const USE_DEPTH_BLEND_FALLOFF = 4294967296;
		const MAKE_USE_OF_VERTEX_COLOR = 17179869184;
		const MAKE_USE_OF_VERTEX_ALPHA = 34359738368;
	}
	
	struct RTTChannels: u32 {
		const CHANNEL0 = 1;
		const CHANNEL1 = 2;
		const CHANNEL2 = 4;
		const CHANNEL3 = 8;
		const CHANNEL4 = 16;
		const CHANNEL5 = 32;
		const CHANNEL6 = 64;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum MaterialData {
	V15 {
		name: Reference,
		flags: MaterialFlags,
		blend_mode: u32,
		priority: i32,
		used_rtt_channels: RTTChannels,
		specularity: f32,
		depth_blend_falloff: f32,
		cutout_thresh: u8,
		unknown41: [u8; 3],
		spec_mult: f32,
		emis_mult: f32,
		diffuse_layer: Reference,
		decal_layer: Reference,
		specular_layer: Reference,
		emissive_layers: [Reference; 2],
		evio_layer: Reference,
		evio_mask_layer: Reference,
		alpha_mask_layers: [Reference; 2],
		normal_layer: Reference,
		height_layer: Reference,
		light_map_layer: Reference,
		ambient_occlusion_layer: Reference,
		unknown3: u32,
		layer_blend_type: u32,
		emis_blend_type: u32,
		emis_mode: u32,
		spec_type: u32,
		unknown_animation_ref1: AnimationReference,
		unknown_animation_ref2: AnimationReference,
	},
	V16_17_18 {
		name: Reference,
		flags: MaterialFlags,
		blend_mode: u32,
		priority: i32,
		used_rtt_channels: RTTChannels,
		specularity: f32,
		depth_blend_falloff: f32,
		cutout_thresh: u8,
		unknown41: [u8; 3],
		spec_mult: f32,
		emis_mult: f32,
		diffuse_layer: Reference,
		decal_layer: Reference,
		specular_layer: Reference,
		gloss_layer: Reference,
		emissive_layers: [Reference; 2],
		evio_layer: Reference,
		evio_mask_layer: Reference,
		alpha_mask_layers: [Reference; 2],
		normal_layer: Reference,
		height_layer: Reference,
		light_map_layer: Reference,
		ambient_occlusion_layer: Reference,
		unknown3: u32,
		layer_blend_type: u32,
		emis_blend_type: u32,
		emis_mode: u32,
		spec_type: u32,
		unknown_animation_ref1: AnimationReference,
		unknown_animation_ref2: AnimationReference,
	},
	V19 {
		name: Reference,
		flags: MaterialFlags,
		blend_mode: u32,
		priority: i32,
		used_rtt_channels: RTTChannels,
		specularity: f32,
		depth_blend_falloff: f32,
		cutout_thresh: u8,
		unknown41: [u8; 3],
		spec_mult: f32,
		emis_mult: f32,
		diffuse_layer: Reference,
		decal_layer: Reference,
		specular_layer: Reference,
		gloss_layer: Reference,
		emissive_layers: [Reference; 2],
		evio_layer: Reference,
		evio_mask_layer: Reference,
		alpha_mask_layers: [Reference; 2],
		normal_layer: Reference,
		height_layer: Reference,
		light_map_layer: Reference,
		ambient_occlusion_layer: Reference,
		unknown_layer1: Reference,
		unknown_layer2: Reference,
		unknown_layer3: Reference,
		unknown_layer4: Reference,
		unknown3: u32,
		layer_blend_type: u32,
		emis_blend_type: u32,
		emis_mode: u32,
		spec_type: u32,
		unknown_animation_ref1: AnimationReference,
		unknown_animation_ref2: AnimationReference,
		unknown_2410d9f0: [u8; 12],
	},
	V20 {
		name: Reference,
		flags: MaterialFlags,
		blend_mode: u32,
		priority: i32,
		used_rtt_channels: RTTChannels,
		specularity: f32,
		depth_blend_falloff: f32,
		cutout_thresh: u8,
		unknown41: [u8; 3],
		spec_mult: f32,
		emis_mult: f32,
		unknown_15e494: [u8; 12],
		diffuse_layer: Reference,
		decal_layer: Reference,
		specular_layer: Reference,
		gloss_layer: Reference,
		emissive_layers: [Reference; 2],
		evio_layer: Reference,
		evio_mask_layer: Reference,
		alpha_mask_layers: [Reference; 2],
		normal_layer: Reference,
		height_layer: Reference,
		light_map_layer: Reference,
		ambient_occlusion_layer: Reference,
		unknown_layer1: Reference,
		unknown_layer2: Reference,
		unknown_layer3: Reference,
		unknown_layer4: Reference,
		unknown3: u32,
		layer_blend_type: u32,
		emis_blend_type: u32,
		emis_mode: u32,
		spec_type: u32,
		unknown_animation_ref1: AnimationReference,
		unknown_animation_ref2: AnimationReference,
		unknown_2410d9f0: [u8; 12],
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Material {
	data: MaterialData,
	name: String,
	diffuse: Layer,
	decal: Layer,
	specular: Layer,
	gloss: Layer,
	emissive: [Layer; 2],
	evio: Layer,
	evio_mask: Layer,
	alpha_mask: [Layer; 2],
	normal: Layer,
	height: Layer,
	light_map: Layer,
	ambient_occlusion: Layer,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct DisplacementMaterialData {
	name: Reference,
	unknown0: u32,
	strength_factor: AnimationReference,
	normal_layer: Reference,
	strength_layer: Reference,
	flags: u32,
	priority: i32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct DisplacementMaterial {
	data: DisplacementMaterialData,
	name: String,
	normal: Layer,
	strength: Layer,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct CompositeMaterialSection {
	material_reference_index: u32,
	alpha_factor: AnimationReference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct CompositeMaterialData {
	name: Reference,
	unknown: u32,
	sections: Reference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct CompositeMaterial {
	data: CompositeMaterialData,
	name: String,
	sections: Vec<CompositeMaterialSection>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum TerrainMaterialData {
	V0 {
		name: Reference,
		terrain_layer: Reference,
	},
	V1 {
		name: Reference,
		terrain_layer: Reference,
		unknown_633fd422: [u8; 4],
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct TerrainMaterial {
	data: TerrainMaterialData,
	name: String,
	terrain: Layer,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct VolumeMaterialData {
	name: Reference,
	unknown0: u32,
	unknown1: u32,
	volume_density: AnimationReference,
	color_defining_layer: Reference,
	unknown_layer1: Reference,
	unknown_layer2: Reference,
	unknown2: u32,
	unknown3: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct VolumeMaterial {
	data: VolumeMaterialData,
	name: String,
	color_defining: Layer,
}

bitflags! {
	struct VolumeNoiseMaterialFlags: u32 {
		const DRAW_AFTER_TRANSPARENCY = 1;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct VolumeNoiseMaterialData {
	name: Reference,
	unknown_50762f82: u32,
	flags: VolumeNoiseMaterialFlags,
	volume_density: AnimationReference,
	near_plane: AnimationReference,
	falloff: AnimationReference,
	color_layer: Reference,
	noise_layers: [Reference; 2],
	scroll_rate: AnimationReference,
	translation: AnimationReference,
	scale: AnimationReference,
	rotation: AnimationReference,
	alpha_threshold: u32,
	unknown_1d13acfe: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct VolumeNoiseMaterial {
	data: VolumeNoiseMaterialData,
	name: String,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum CreepMaterialData {
	V0 {
		name: Reference,
		creep_layer: Reference,
	},
	V1 {
		name: Reference,
		creep_layer: Reference,
		unknown_da1b4eb3: [u8; 4],
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct CreepMaterial {
	data: CreepMaterialData,
	name: String,
	creep: Layer,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct SplatTerrainBakeMaterialData {
	name: Reference,
	diffuse_layer: Reference,
	normal_layer: Reference,
	specular_layer: Reference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct SplatTerrainBakeMaterial {
	data: SplatTerrainBakeMaterialData,
	name: String,
	diffuse: Layer,
	normal: Layer,
	specular: Layer,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct LensFlareMaterialSub {
	unknown_07c51b5b: u32,
	unknown_de754918: f32,
	unknown_d74895eb: f32,
	unknown_5d722d08: f32,
	unknown_fc180384: f32,
	unknown_713b7b48: f32,
	unknown_389805a1: [u8; 12],
	unknown_1e6f06e7: f32,
	unknown_402f350d: u32,
	unknown_42724138: [u8; 12],
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum LensFlareMaterialData {
	V2 {
		name: Reference,
		unknown_layer1: Reference,
		unknown_layer2: Reference,
		unknown_335fd923: Reference,
		unknown_7f492c0a: u32,
		unknown_7c0501fd: [u8; 28],
	},
	V3 {
		name: Reference,
		unknown_layer1: Reference,
		unknown_layer2: Reference,
		unknown_335fd923: Reference,
		source_file_path: Reference,
		unknown_191c7146: Reference,
		unknown_e5cb3d48: u32,
		unknown_62cd83e0: [u8; 4],
		unknown_7f492c0a: u32,
		unknown_592478e1: [u8; 4],
		unknown_d3786e1e: [u8; 4],
		unknown_8beb57a7: [u8; 4],
		unknown_9b624fc7: [u8; 4],
		unknown_baed442e: [u8; 4],
		unknown_7a47ad22: [u8; 4],
		unknown_8b614731: [u8; 4],
		unknown_1d8f2887: [u8; 4],
		unknown_43f3ad4d: [u8; 4],
		unknown_edeec90d: f32,
		unknown_374c15de: f32,
		unknown_5f5e0f87: [u8; 4],
		unknown_f6e1c072: [u8; 4],
		unknown_41cff88a: [u8; 4],
		unknown_1c5b9dc4: f32,
		unknown_57c02c2b: f32,
		unknown_294d3393: [u8; 4],
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct LensFlareMaterial {
	data: LensFlareMaterialData,
	name: String,
	source_file_path: String,
}

bitflags! {
	struct ParticleSystemAdditionalFlags: u32 {
		const RANDOMIZE_WITH_EMISSION_SPEED2 = 1;
		const RANDOMIZE_WITH_LIFESPAN2 = 2;
		const RANDOMIZE_WITH_MASS2 = 4;
		const TRAILING_ENABLED = 8;
	}
	
	struct ParticleSystemFlags: u32 {
		const SORT = 1;
		const COLLIDE_TERRAIN = 2;
		const COLLIDE_OBJECTS = 4;
		const SPAWN_ON_BOUNCE = 8;
		const CUTOUT_EMISSION_AREA = 16;
		const INHERIT_EMISSION_PARAMS = 32;
		const INHERIT_PARENT_VEL = 64;
		const SORT_BY_Z_HEIGHT = 128;
		const REVERSE_ITERATION = 256;
		const SMOOTH_ROTATION = 512;
		const BEZ_SMOOTH_ROTATION = 1024;
		const SMOOTH_SIZE = 2048;
		const BEZ_SMOOTH_SIZE = 4096;
		const SMOOTH_COLOR = 8192;
		const BEZ_SMOOTH_COLOR = 16384;
		const LIT_PARTS = 32768;
		const RAND_FLIP_BOOK_START = 65536;
		const MULTIPLY_BY_GRAVITY = 131072;
		const CLAMP_TAIL_PARTS = 262144;
		const SPAWN_TRAILING_PARTS = 524288;
		const FIX_LENGTH_TAIL_PARTS = 1048576;
		const USE_VERTEX_ALPHA = 2097152;
		const MODEL_PARTS = 4194304;
		const SWAP_YZ_ON_MODEL_PARTS = 8388608;
		const SCALE_TIME_BY_PARENT = 16777216;
		const USE_LOCAL_TIME = 33554432;
		const SIMULATE_ON_INIT = 67108864;
		const COPY = 134217728;
	}
	
	struct ParticleSystemRotationFlags: u32 {
		const RELATIVE = 2;
		const ALWAYS_SET = 4;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum ParticleType {
	SQUARE_BILLBOARDS1 = 0,
	SPEED_SCALED_ROTATED_BILLBOARDS = 1,
	SQUARE_BILLBOARDS2 = 2,
	SQUARE_BILLBOARDS3 = 3,
	SQUARE_BILLBOARDS4 = 4,
	SQUARE_BILLBOARDS5 = 5,
	RECTANGULAR_BILLBOARDS = 6,
	QUADS_SPEED_AS_NORMAL = 7,
	QUADS_BETWEEN_SPAWN_AND_CURRENT = 9,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum Smoothing {
	LINEAR = 0,
	SMOOTH = 1,
	BEZIER = 2,
	LINEAR_HOLD = 3,
	BEZIER_HOLD = 4,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum ParticleSystemData {
	V12 {
		bone: u32,
		material_reference_index: u32,
		emission_speeds: [AnimationReference; 2],
		randomize_with_emission_speed2: u32,
		emission_angle_x: AnimationReference,
		emission_angle_y: AnimationReference,
		emission_spread_x: AnimationReference,
		emission_spread_y: AnimationReference,
		lifespans: [AnimationReference; 2],
		randomize_with_lifespan2: u32,
		unknown_deb2bc5d: f32,
		unknown_f21cc980: [u8; 8],
		z_acceleration: f32,
		size_animation_middle: f32,
		color_animation_middle: f32,
		alpha_animation_middle: f32,
		rotation_animation_middle: f32,
		sizes: [AnimationReference; 2],
		rotations: [AnimationReference; 2],
		initial_colors: [AnimationReference; 2],
		middle_colors: [AnimationReference; 2],
		final_colors: [AnimationReference; 2],
		slowdown: f32,
		masses: [f32; 2],
		unknown_float_2c: f32,
		unknown4: u32,
		trailing_enabled: u32,
		local_force_channels: u16,
		world_force_channels: u16,
		force_channels_filler: u16,
		world_force_channels_copy: u16,
		noise_amplitude: f32,
		noise_frequency: f32,
		noise_cohesion: f32,
		noise_edge: f32,
		index_plus_highest_index: u32,
		max_particles: u32,
		emission_rate: AnimationReference,
		emission_area_type: u32,
		emission_area_size: AnimationReference,
		emission_area_cutout_size: AnimationReference,
		emission_area_radius: AnimationReference,
		emission_area_cutout_radius: AnimationReference,
		emission_type: u32,
		randomize_with_sizes2: u32,
		randomize_with_rotation2: u32,
		unknown7: u32,
		part_emit: AnimationReference,
		phase_start_image_indices: [u8; 2],
		phase_end_image_indices: [u8; 2],
		relative_phase1_length: f32,
		columns: u16,
		rows: u16,
		column_width: f32,
		row_height: f32,
		unknown_float4: f32,
		unknown_float5: f32,
		unknown9: i32,
		unknown_700df5cf: [u8; 20],
		particle_type: ParticleType,
		length_width_ratio: f32,
		unknown11: [u8; 8],
		unknown_float7: f32,
		unknown_9ffad0f0: u32,
		unknown_e0bd54c8: AnimationReference,
		unknown_a2d44d80: AnimationReference,
		unknown_b6d53a88: u32,
		unknown_f8e2b3d0: AnimationReference,
		unknown_54f4ae30: AnimationReference,
		unknown_f8b3d529: u32,
		unknown_5f54fb02: AnimationReference,
		unknown_84d843d6: AnimationReference,
		unknown_6b3b9f9: [u8; 4],
		unknown_9cb3dd18: AnimationReference,
		unknown_2e01be90: AnimationReference,
		unknown_eff4c5be: u32,
		unknown_f6193fc0: AnimationReference,
		unknown_a5e2260a: AnimationReference,
		unknown_9aa3bbb9: [u8; 4],
		unknown_485f7eea: AnimationReference,
		unknown_34b6f141: AnimationReference,
		unknown_b69653a4: u32,
		unknown_89cdf966: AnimationReference,
		unknown_4eefdfc1: AnimationReference,
		unknown_b47e5f82: [u8; 4],
		unknown_ab37a1d5: AnimationReference,
		unknown_bef7f4d3: AnimationReference,
		unknown_8262f69c: [u8; 4],
		unknown_b2dbf2f3: AnimationReference,
		unknown_3c76d64c: AnimationReference,
		unknown_bc151e17: AnimationReference,
		flags: ParticleSystemFlags,
		spawn_points: Reference,
		unknown_105fd76d: f32,
		unknown_5156cf07: u32,
		unknown_92dba6ab: u32,
		unknown_21ca0cea: AnimationReference,
		unknown_1e97145f: AnimationReference,
		trailing_particles_index: i32,
		trailing_particles_chance: f32,
		trailing_particles_rate: AnimationReference,
		unknown_fdab7ff3: [u8; 8],
		used_model: Reference,
		copy_indices: Reference,
	},
	V17 {
		bone: u32,
		material_reference_index: u32,
		additional_flags: ParticleSystemAdditionalFlags,
		emission_speeds: [AnimationReference; 2],
		emission_angle_x: AnimationReference,
		emission_angle_y: AnimationReference,
		emission_spread_x: AnimationReference,
		emission_spread_y: AnimationReference,
		lifespans: [AnimationReference; 2],
		unknown_deb2bc5d: f32,
		unknown_f21cc980: [u8; 8],
		z_acceleration: f32,
		size_animation_middle: f32,
		color_animation_middle: f32,
		alpha_animation_middle: f32,
		rotation_animation_middle: f32,
		size_hold_time: f32,
		color_hold_time: f32,
		alpha_hold_time: f32,
		rotation_hold_time: f32,
		sizes: [AnimationReference; 2],
		rotations: [AnimationReference; 2],
		initial_colors: [AnimationReference; 2],
		middle_colors: [AnimationReference; 2],
		final_colors: [AnimationReference; 2],
		slowdown: f32,
		masses: [f32; 2],
		unknown_float_2c: f32,
		local_force_channels: u16,
		world_force_channels: u16,
		force_channels_filler: u16,
		world_force_channels_copy: u16,
		noise_amplitude: f32,
		noise_frequency: f32,
		noise_cohesion: f32,
		noise_edge: f32,
		index_plus_highest_index: u32,
		max_particles: u32,
		emission_rate: AnimationReference,
		emission_area_type: u32,
		emission_area_size: AnimationReference,
		emission_area_cutout_size: AnimationReference,
		emission_area_radius: AnimationReference,
		emission_area_cutout_radius: AnimationReference,
		unknown_51fe79f5: Reference,
		emission_type: u32,
		randomize_with_sizes2: u32,
		randomize_with_rotation2: u32,
		unknown7: u32,
		part_emit: AnimationReference,
		phase_start_image_indices: [u8; 2],
		phase_end_image_indices: [u8; 2],
		relative_phase1_length: f32,
		columns: u16,
		rows: u16,
		column_width: f32,
		row_height: f32,
		unknown_float4: f32,
		unknown_float5: f32,
		unknown9: i32,
		unknown_700df5cf: [u8; 20],
		particle_type: ParticleType,
		length_width_ratio: f32,
		unknown11: [u8; 8],
		unknown_float7: f32,
		unknown5_added_v21: f32,
		unknown_9ffad0f0: u32,
		unknown_e0bd54c8: AnimationReference,
		unknown_a2d44d80: AnimationReference,
		unknown_b6d53a88: u32,
		unknown_f8e2b3d0: AnimationReference,
		unknown_54f4ae30: AnimationReference,
		unknown_f8b3d529: u32,
		unknown_5f54fb02: AnimationReference,
		unknown_84d843d6: AnimationReference,
		unknown_6b3b9f9: [u8; 4],
		unknown_9cb3dd18: AnimationReference,
		unknown_2e01be90: AnimationReference,
		unknown_eff4c5be: u32,
		unknown_f6193fc0: AnimationReference,
		unknown_a5e2260a: AnimationReference,
		unknown_9aa3bbb9: [u8; 4],
		unknown_485f7eea: AnimationReference,
		unknown_34b6f141: AnimationReference,
		unknown_b69653a4: u32,
		unknown_89cdf966: AnimationReference,
		unknown_4eefdfc1: AnimationReference,
		unknown_b47e5f82: [u8; 4],
		unknown_ab37a1d5: AnimationReference,
		unknown_bef7f4d3: AnimationReference,
		unknown_8262f69c: [u8; 4],
		unknown_b2dbf2f3: AnimationReference,
		unknown_3c76d64c: AnimationReference,
		unknown_bc151e17: AnimationReference,
		flags: ParticleSystemFlags,
		color_smoothing: Smoothing,
		size_smoothing: Smoothing,
		rotation_smoothing: Smoothing,
		unknown_22856fde: AnimationReference,
		unknown_b35ad6e1: AnimationReference,
		unknown_686e5943: AnimationReference,
		unknown_18a90564: AnimationReference,
		spawn_points: Reference,
		unknown_105fd76d: f32,
		unknown_5156cf07: u32,
		unknown_92dba6ab: u32,
		unknown_21ca0cea: AnimationReference,
		unknown_1e97145f: AnimationReference,
		trailing_particles_index: i32,
		trailing_particles_chance: f32,
		trailing_particles_rate: AnimationReference,
		unknown_fdab7ff3: [u8; 8],
		used_model: Reference,
		copy_indices: Reference,
	},
	V18_19_21 {
		bone: u32,
		material_reference_index: u32,
		additional_flags: ParticleSystemAdditionalFlags,
		emission_speeds: [AnimationReference; 2],
		emission_angle_x: AnimationReference,
		emission_angle_y: AnimationReference,
		emission_spread_x: AnimationReference,
		emission_spread_y: AnimationReference,
		lifespans: [AnimationReference; 2],
		unknown_deb2bc5d: f32,
		unknown_f21cc980: [u8; 8],
		z_acceleration: f32,
		size_animation_middle: f32,
		color_animation_middle: f32,
		alpha_animation_middle: f32,
		rotation_animation_middle: f32,
		size_hold_time: f32,
		color_hold_time: f32,
		alpha_hold_time: f32,
		rotation_hold_time: f32,
		sizes: [AnimationReference; 2],
		rotations: [AnimationReference; 2],
		initial_colors: [AnimationReference; 2],
		middle_colors: [AnimationReference; 2],
		final_colors: [AnimationReference; 2],
		slowdown: f32,
		masses: [f32; 2],
		unknown_float_2c: f32,
		local_force_channels: u16,
		world_force_channels: u16,
		force_channels_filler: u16,
		world_force_channels_copy: u16,
		noise_amplitude: f32,
		noise_frequency: f32,
		noise_cohesion: f32,
		noise_edge: f32,
		index_plus_highest_index: u32,
		max_particles: u32,
		emission_rate: AnimationReference,
		emission_area_type: u32,
		emission_area_size: AnimationReference,
		emission_area_cutout_size: AnimationReference,
		emission_area_radius: AnimationReference,
		emission_area_cutout_radius: AnimationReference,
		unknown_51fe79f5: Reference,
		emission_type: u32,
		randomize_with_sizes2: u32,
		randomize_with_rotation2: u32,
		unknown7: u32,
		part_emit: AnimationReference,
		phase_start_image_indices: [u8; 2],
		phase_end_image_indices: [u8; 2],
		relative_phase1_length: f32,
		columns: u16,
		rows: u16,
		column_width: f32,
		row_height: f32,
		unknown_float4: f32,
		unknown_float5: f32,
		unknown9: i32,
		unknown_700df5cf: [u8; 20],
		particle_type: ParticleType,
		length_width_ratio: f32,
		unknown11: [u8; 8],
		unknown_float7: f32,
		unknown5_added_v21: f32,
		unknown_9ffad0f0: u32,
		unknown_e0bd54c8: AnimationReference,
		unknown_a2d44d80: AnimationReference,
		unknown_b6d53a88: u32,
		unknown_f8e2b3d0: AnimationReference,
		unknown_54f4ae30: AnimationReference,
		unknown_f8b3d529: u32,
		unknown_5f54fb02: AnimationReference,
		unknown_84d843d6: AnimationReference,
		unknown_6b3b9f9: [u8; 4],
		unknown_9cb3dd18: AnimationReference,
		unknown_2e01be90: AnimationReference,
		unknown_eff4c5be: u32,
		unknown_f6193fc0: AnimationReference,
		unknown_a5e2260a: AnimationReference,
		unknown_9aa3bbb9: [u8; 4],
		unknown_485f7eea: AnimationReference,
		unknown_34b6f141: AnimationReference,
		unknown_b69653a4: u32,
		unknown_89cdf966: AnimationReference,
		unknown_4eefdfc1: AnimationReference,
		unknown_b47e5f82: [u8; 4],
		unknown_ab37a1d5: AnimationReference,
		unknown_bef7f4d3: AnimationReference,
		unknown_8262f69c: [u8; 4],
		unknown_b2dbf2f3: AnimationReference,
		unknown_3c76d64c: AnimationReference,
		unknown_bc151e17: AnimationReference,
		flags: ParticleSystemFlags,
		rotation_flags: ParticleSystemRotationFlags,
		color_smoothing: Smoothing,
		size_smoothing: Smoothing,
		rotation_smoothing: Smoothing,
		unknown_22856fde: AnimationReference,
		unknown_b35ad6e1: AnimationReference,
		unknown_686e5943: AnimationReference,
		unknown_18a90564: AnimationReference,
		spawn_points: Reference,
		unknown_105fd76d: f32,
		unknown_5156cf07: u32,
		unknown_92dba6ab: u32,
		unknown_21ca0cea: AnimationReference,
		unknown_1e97145f: AnimationReference,
		trailing_particles_index: i32,
		trailing_particles_chance: f32,
		trailing_particles_rate: AnimationReference,
		unknown_fdab7ff3: [u8; 8],
		used_model: Reference,
		copy_indices: Reference,
	},
	V22 {
		bone: u32,
		material_reference_index: u32,
		additional_flags: ParticleSystemAdditionalFlags,
		emission_speeds: [AnimationReference; 2],
		emission_angle_x: AnimationReference,
		emission_angle_y: AnimationReference,
		emission_spread_x: AnimationReference,
		emission_spread_y: AnimationReference,
		lifespans: [AnimationReference; 2],
		unknown_deb2bc5d: f32,
		unknown_f21cc980: [u8; 8],
		z_acceleration: f32,
		size_animation_middle: f32,
		color_animation_middle: f32,
		alpha_animation_middle: f32,
		rotation_animation_middle: f32,
		size_hold_time: f32,
		color_hold_time: f32,
		alpha_hold_time: f32,
		rotation_hold_time: f32,
		sizes: [AnimationReference; 2],
		rotations: [AnimationReference; 2],
		initial_colors: [AnimationReference; 2],
		middle_colors: [AnimationReference; 2],
		final_colors: [AnimationReference; 2],
		slowdown: f32,
		masses: [f32; 2],
		unknown_float_2c: f32,
		local_force_channels: u16,
		world_force_channels: u16,
		force_channels_filler: u16,
		world_force_channels_copy: u16,
		noise_amplitude: f32,
		noise_frequency: f32,
		noise_cohesion: f32,
		noise_edge: f32,
		index_plus_highest_index: u32,
		max_particles: u32,
		emission_rate: AnimationReference,
		emission_area_type: u32,
		emission_area_size: AnimationReference,
		emission_area_cutout_size: AnimationReference,
		emission_area_radius: AnimationReference,
		emission_area_cutout_radius: AnimationReference,
		unknown_51fe79f5: Reference,
		emission_type: u32,
		randomize_with_sizes2: u32,
		randomize_with_rotation2: u32,
		unknown7: u32,
		part_emit: AnimationReference,
		phase_start_image_indices: [u8; 2],
		phase_end_image_indices: [u8; 2],
		relative_phase1_length: f32,
		columns: u16,
		rows: u16,
		column_width: f32,
		row_height: f32,
		unknown_float4: f32,
		unknown_float5: f32,
		unknown9: i32,
		unknown_700df5cf: [u8; 20],
		particle_type: ParticleType,
		length_width_ratio: f32,
		unknown11: [u8; 8],
		unknown_float7: f32,
		unknown5_added_v21: f32,
		unknown_9ffad0f0: u32,
		unknown_e0bd54c8: AnimationReference,
		unknown_a2d44d80: AnimationReference,
		unknown_b6d53a88: u32,
		unknown_f8e2b3d0: AnimationReference,
		unknown_54f4ae30: AnimationReference,
		unknown_f8b3d529: u32,
		unknown_5f54fb02: AnimationReference,
		unknown_84d843d6: AnimationReference,
		unknown_6b3b9f9: [u8; 4],
		unknown_9cb3dd18: AnimationReference,
		unknown_2e01be90: AnimationReference,
		unknown_eff4c5be: u32,
		unknown_f6193fc0: AnimationReference,
		unknown_a5e2260a: AnimationReference,
		unknown_9aa3bbb9: [u8; 4],
		unknown_485f7eea: AnimationReference,
		unknown_34b6f141: AnimationReference,
		unknown_b69653a4: u32,
		unknown_89cdf966: AnimationReference,
		unknown_4eefdfc1: AnimationReference,
		unknown_b47e5f82: [u8; 4],
		unknown_ab37a1d5: AnimationReference,
		unknown_bef7f4d3: AnimationReference,
		unknown_8262f69c: [u8; 4],
		unknown_b2dbf2f3: AnimationReference,
		unknown_3c76d64c: AnimationReference,
		unknown_bc151e17: AnimationReference,
		unknown_8f507b52: AnimationReference,
		flags: ParticleSystemFlags,
		rotation_flags: ParticleSystemRotationFlags,
		color_smoothing: Smoothing,
		size_smoothing: Smoothing,
		rotation_smoothing: Smoothing,
		unknown_22856fde: AnimationReference,
		unknown_b35ad6e1: AnimationReference,
		unknown_686e5943: AnimationReference,
		unknown_18a90564: AnimationReference,
		spawn_points: Reference,
		unknown_105fd76d: f32,
		unknown_5156cf07: u32,
		unknown_92dba6ab: u32,
		unknown_21ca0cea: AnimationReference,
		unknown_1e97145f: AnimationReference,
		trailing_particles_index: i32,
		trailing_particles_chance: f32,
		trailing_particles_rate: AnimationReference,
		unknown_fdab7ff3: [u8; 8],
		used_model: Reference,
		copy_indices: Reference,
	},
	V23 {
		bone: u32,
		material_reference_index: u32,
		additional_flags: ParticleSystemAdditionalFlags,
		emission_speeds: [AnimationReference; 2],
		emission_angle_x: AnimationReference,
		emission_angle_y: AnimationReference,
		emission_spread_x: AnimationReference,
		emission_spread_y: AnimationReference,
		lifespans: [AnimationReference; 2],
		unknown_deb2bc5d: f32,
		unknown_f21cc980: [u8; 8],
		z_acceleration: f32,
		size_animation_middle: f32,
		color_animation_middle: f32,
		alpha_animation_middle: f32,
		rotation_animation_middle: f32,
		size_hold_time: f32,
		color_hold_time: f32,
		alpha_hold_time: f32,
		rotation_hold_time: f32,
		sizes: [AnimationReference; 2],
		rotations: [AnimationReference; 2],
		initial_colors: [AnimationReference; 2],
		middle_colors: [AnimationReference; 2],
		final_colors: [AnimationReference; 2],
		slowdown: f32,
		masses: [f32; 2],
		unknown_float_2c: f32,
		local_force_channels: u16,
		world_force_channels: u16,
		force_channels_filler: u16,
		world_force_channels_copy: u16,
		noise_amplitude: f32,
		noise_frequency: f32,
		noise_cohesion: f32,
		noise_edge: f32,
		index_plus_highest_index: u32,
		max_particles: u32,
		emission_rate: AnimationReference,
		emission_area_type: u32,
		emission_area_size: AnimationReference,
		emission_area_cutout_size: AnimationReference,
		emission_area_radius: AnimationReference,
		emission_area_cutout_radius: AnimationReference,
		unknown_51fe79f5: Reference,
		emission_type: u32,
		randomize_with_sizes2: u32,
		randomize_with_rotation2: u32,
		unknown7: u32,
		part_emit: AnimationReference,
		phase_start_image_indices: [u8; 2],
		phase_end_image_indices: [u8; 2],
		relative_phase1_length: f32,
		columns: u16,
		rows: u16,
		column_width: f32,
		row_height: f32,
		unknown_float4: f32,
		unknown_float5: f32,
		unknown9: i32,
		unknown_700df5cf: [u8; 20],
		particle_type: ParticleType,
		length_width_ratio: f32,
		unknown11: [u8; 8],
		unknown_float7: f32,
		unknown5_added_v21: f32,
		unknown_9ffad0f0: u32,
		unknown_e0bd54c8: AnimationReference,
		unknown_a2d44d80: AnimationReference,
		unknown_b6d53a88: u32,
		unknown_f8e2b3d0: AnimationReference,
		unknown_54f4ae30: AnimationReference,
		unknown_f8b3d529: u32,
		unknown_5f54fb02: AnimationReference,
		unknown_84d843d6: AnimationReference,
		unknown_6b3b9f9: [u8; 4],
		unknown_9cb3dd18: AnimationReference,
		unknown_2e01be90: AnimationReference,
		unknown_eff4c5be: u32,
		unknown_f6193fc0: AnimationReference,
		unknown_a5e2260a: AnimationReference,
		unknown_9aa3bbb9: [u8; 4],
		unknown_485f7eea: AnimationReference,
		unknown_34b6f141: AnimationReference,
		unknown_b69653a4: u32,
		unknown_89cdf966: AnimationReference,
		unknown_4eefdfc1: AnimationReference,
		unknown_b47e5f82: [u8; 4],
		unknown_ab37a1d5: AnimationReference,
		unknown_bef7f4d3: AnimationReference,
		unknown_8262f69c: [u8; 4],
		unknown_b2dbf2f3: AnimationReference,
		unknown_3c76d64c: AnimationReference,
		unknown_bc151e17: AnimationReference,
		unknown_8f507b52: AnimationReference,
		flags: ParticleSystemFlags,
		rotation_flags: ParticleSystemRotationFlags,
		color_smoothing: Smoothing,
		size_smoothing: Smoothing,
		rotation_smoothing: Smoothing,
		unknown_22856fde: AnimationReference,
		unknown_b35ad6e1: AnimationReference,
		unknown_686e5943: AnimationReference,
		unknown_18a90564: AnimationReference,
		spawn_points: Reference,
		unknown_105fd76d: f32,
		unknown_5156cf07: u32,
		unknown_92dba6ab: u32,
		unknown_21ca0cea: AnimationReference,
		unknown_1e97145f: AnimationReference,
		trailing_particles_index: i32,
		trailing_particles_chance: f32,
		trailing_particles_rate: AnimationReference,
		unknown_fdab7ff3: [u8; 8],
		used_model: Reference,
		copy_indices: Reference,
		unknown_9a7afdf2: [u8; 8],
	},
	V24 {
		bone: u32,
		material_reference_index: u32,
		additional_flags: ParticleSystemAdditionalFlags,
		emission_speeds: [AnimationReference; 2],
		emission_angle_x: AnimationReference,
		emission_angle_y: AnimationReference,
		emission_spread_x: AnimationReference,
		emission_spread_y: AnimationReference,
		lifespans: [AnimationReference; 2],
		unknown_deb2bc5d: f32,
		unknown_f21cc980: [u8; 8],
		z_acceleration: f32,
		size_animation_middle: f32,
		color_animation_middle: f32,
		alpha_animation_middle: f32,
		rotation_animation_middle: f32,
		size_hold_time: f32,
		color_hold_time: f32,
		alpha_hold_time: f32,
		rotation_hold_time: f32,
		sizes: [AnimationReference; 2],
		rotations: [AnimationReference; 2],
		initial_colors: [AnimationReference; 2],
		middle_colors: [AnimationReference; 2],
		final_colors: [AnimationReference; 2],
		slowdown: f32,
		masses: [f32; 2],
		unknown_float_2c: f32,
		local_force_channels: u16,
		world_force_channels: u16,
		force_channels_filler: u16,
		world_force_channels_copy: u16,
		noise_amplitude: f32,
		noise_frequency: f32,
		noise_cohesion: f32,
		noise_edge: f32,
		unknown_c851d80: f32,
		index_plus_highest_index: u32,
		max_particles: u32,
		emission_rate: AnimationReference,
		emission_area_type: u32,
		emission_area_size: AnimationReference,
		emission_area_cutout_size: AnimationReference,
		emission_area_radius: AnimationReference,
		emission_area_cutout_radius: AnimationReference,
		unknown_51fe79f5: Reference,
		emission_type: u32,
		randomize_with_sizes2: u32,
		randomize_with_rotation2: u32,
		unknown7: u32,
		part_emit: AnimationReference,
		phase_start_image_indices: [u8; 2],
		phase_end_image_indices: [u8; 2],
		relative_phase1_length: f32,
		columns: u16,
		rows: u16,
		column_width: f32,
		row_height: f32,
		unknown_float4: f32,
		unknown_float5: f32,
		unknown9: i32,
		unknown_700df5cf: [u8; 20],
		particle_type: ParticleType,
		length_width_ratio: f32,
		unknown11: [u8; 8],
		unknown_float7: f32,
		unknown5_added_v21: f32,
		unknown_9ffad0f0: u32,
		unknown_e0bd54c8: AnimationReference,
		unknown_a2d44d80: AnimationReference,
		unknown_b6d53a88: u32,
		unknown_f8e2b3d0: AnimationReference,
		unknown_54f4ae30: AnimationReference,
		unknown_f8b3d529: u32,
		unknown_5f54fb02: AnimationReference,
		unknown_84d843d6: AnimationReference,
		unknown_6b3b9f9: [u8; 4],
		unknown_9cb3dd18: AnimationReference,
		unknown_2e01be90: AnimationReference,
		unknown_eff4c5be: u32,
		unknown_f6193fc0: AnimationReference,
		unknown_a5e2260a: AnimationReference,
		unknown_9aa3bbb9: [u8; 4],
		unknown_485f7eea: AnimationReference,
		unknown_34b6f141: AnimationReference,
		unknown_b69653a4: u32,
		unknown_89cdf966: AnimationReference,
		unknown_4eefdfc1: AnimationReference,
		unknown_b47e5f82: [u8; 4],
		unknown_ab37a1d5: AnimationReference,
		unknown_bef7f4d3: AnimationReference,
		unknown_8262f69c: [u8; 4],
		unknown_b2dbf2f3: AnimationReference,
		unknown_3c76d64c: AnimationReference,
		unknown_bc151e17: AnimationReference,
		unknown_8f507b52: AnimationReference,
		flags: ParticleSystemFlags,
		rotation_flags: ParticleSystemRotationFlags,
		color_smoothing: Smoothing,
		size_smoothing: Smoothing,
		rotation_smoothing: Smoothing,
		unknown_22856fde: AnimationReference,
		unknown_b35ad6e1: AnimationReference,
		unknown_686e5943: AnimationReference,
		unknown_18a90564: AnimationReference,
		spawn_points: Reference,
		unknown_105fd76d: f32,
		unknown_5156cf07: u32,
		unknown_92dba6ab: u32,
		unknown_21ca0cea: AnimationReference,
		unknown_1e97145f: AnimationReference,
		trailing_particles_index: i32,
		trailing_particles_chance: f32,
		trailing_particles_rate: AnimationReference,
		unknown_fdab7ff3: [u8; 8],
		used_model: Reference,
		copy_indices: Reference,
		unknown_9a7afdf2: [u8; 8],
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct ParticleSystem {
	data: ParticleSystemData,
	used_model: String,
	copy_indices: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct ParticleSystemInstance {
	emission_rate: AnimationReference,
	part_emit: AnimationReference,
	bone: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Projection {
	V4 {
		kind: u32,
		bone_index: u32,
		material_reference_index: u32,
		unknown_58ae2b94: AnimationReference,
		unknown_f1f7110b: AnimationReference,
		unknown_2035f500: AnimationReference,
		unknown_80d8189b: AnimationReference,
		field_of_view: AnimationReference,
		aspect_ratio: AnimationReference,
		near: AnimationReference,
		far: AnimationReference,
		box_bottom_z_offset: AnimationReference,
		box_top_z_offset: AnimationReference,
		box_left_x_offset: AnimationReference,
		box_right_x_offset: AnimationReference,
		box_front_y_offset: AnimationReference,
		box_back_y_offset: AnimationReference,
		unknown_ca4e8e36: u32,
		alpha_over_time_start: f32,
		alpha_over_time_mid: f32,
		alpha_over_time_end: f32,
		splat_lifetime_attacks: [f32; 2],
		splat_lifetime_holds: [f32; 2],
		splat_lifetime_decays: [f32; 2],
		attenuation_plane_distance: f32,
		active: AnimationReference,
		unknown_7c53fb7b: u32,
		unknown_a0283820: [u8; 18],
	},
	V5 {
		kind: u32,
		bone_index: u32,
		material_reference_index: u32,
		unknown_58ae2b94: AnimationReference,
		unknown_f1f7110b: AnimationReference,
		unknown_2035f500: AnimationReference,
		unknown_80d8189b: AnimationReference,
		field_of_view: AnimationReference,
		aspect_ratio: AnimationReference,
		near: AnimationReference,
		far: AnimationReference,
		box_bottom_z_offset: AnimationReference,
		box_top_z_offset: AnimationReference,
		box_left_x_offset: AnimationReference,
		box_right_x_offset: AnimationReference,
		box_front_y_offset: AnimationReference,
		box_back_y_offset: AnimationReference,
		unknown_ca4e8e36: u32,
		alpha_over_time_start: f32,
		alpha_over_time_mid: f32,
		alpha_over_time_end: f32,
		splat_lifetime_attacks: [f32; 2],
		splat_lifetime_holds: [f32; 2],
		splat_lifetime_decays: [f32; 2],
		attenuation_plane_distance: f32,
		active: AnimationReference,
		unknown_7c53fb7b: u32,
		splat_layer: u16,
		unknown_cbb79159: u16,
		lod_reduce: u16,
		unknown_f0dfd391: u16,
		lod_cut: u16,
		unknown_df285471: u16,
	},
}

type PhysicalJoin = [u8; 180];

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct ClothConstraint {
	unknown1_1: Vector3,
	unknown1_2: u32,
	unknown2_1: Vector3,
	unknown2_2: u32,
	unknown3_1: Vector3,
	unknown3_2: u32,
	unknown4: Vector3,
	unknown5: Vector3,
	unknown6: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Phac {
	unknown1: u32,
	unknown2: u32,
	unknown3: Reference,
	unknown4: Reference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct ClothBehaviorData {
	unknown1: u32,
	unknown2: u32,
	unknown3: Reference,
	unknown4: Reference,
	unknown5: Reference,
	unknown6: Reference,
	unknown7: Reference,
	unknown8: Reference,
	unknown9: Vector3,
	unknown10: Vector3,
	unknown11: Vector3,
	unknown12: Vector3,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct ClothBehavior {
	data: ClothBehaviorData,
}

bitflags! {
	struct ForceFlags: u32 {
		const USE_FALLOFF = 1;
		const USE_HEIGHT_GRADIENT = 2;
		const UNBOUNDED = 4;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Force {
	kind: u32,
	shape: u32,
	unknown1: u32,
	bone_index: u32,
	flags: ForceFlags,
	channels: u32,
	strength: AnimationReference,
	width: AnimationReference,
	height: AnimationReference,
	length: AnimationReference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Dmse {
	unknown: i8,
	i0: u8,
	i1: u8,
	i2: u8,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u8)]
enum PhysicsShapeType {
	BOX = 0,
	SPHERE = 1,
	CAPSULE = 2,
	CYLINDER = 3,
	CONVEX_HULL = 4,
	MESH = 5,
	INVALID = 6,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum PhysicsShapeData {
	V1 {
		matrix: Matrix4x4,
		unknown64: u32,
		shape: PhysicsShapeType,
		unknown_1c286108: u8,
		unknown_4ddf14b3: u16,
		vertices: Reference,
		unknown84: Reference,
		faces: Reference,
		plane_equations: Reference,
		size: Vector3,
	},
	V3 {
		matrix: Matrix4x4,
		unknown64: u32,
		shape: Shape,
		unknown_1c286108: u8,
		unknown_4ddf14b3: u16,
		unknown_66ce545e: [u8; 24],
		size: Vector3,
		unknown_71459989: Reference,
		unknown_46c85f1e: Reference,
		unknown_4f8176e7: Reference,
		unknown_124fad04: Reference,
		unknown_bda8f787: [u8; 12],
		unknown_2e655ca4: [u8; 4],
		unknown_7df99f73: [u8; 4],
		unknown_5c04305e: [u8; 4],
		unknown_8d41b3f8: [u8; 4],
		unknown_56fece2: [u8; 4],
		unknown_6a9914ac: [u8; 84],
		unknown_9d115fa7: [u8; 4],
		unknown_9aea956e: [u8; 4],
		unknown_17c6f44e: [u8; 4],
		unknown_e444b392: [u8; 4],
		unknown_32885641: [u8; 4],
		unknown_721b9a99: [u8; 4],
		unknown_a495c84b: [u8; 4],
		unknown_12e15bc4: [u8; 4],
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct PhysicsShape {
	data: PhysicsShapeData,
	vertices: Vec<Vector3>,
	faces: Vec<u16>,
	plane_equations: Vec<Vector4>,
}

bitflags! {
	struct RigidBodyFlags: u32 {
		const COLLIDABLE = 1;
		const WALKABLE = 2;
		const STACKABLE = 4;
		const SIMULATE_ON_COLLISION = 8;
		const IGNORE_LOCAL_BODIES = 16;
		const ALWAYS_EXISTS = 32;
		const DO_NOT_SIMULATE = 128;
	}
	
	struct WorldForces: u16 {
		const WIND = 1;
		const EXPLOSION = 2;
		const ENERGY = 4;
		const BLOOD = 8;
		const MAGNETIC = 16;
		const GRASS = 32;
		const BRUSH = 64;
		const TREES = 128;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum RigidBodyData {
	V2 {
		unknown0: f32,
		unknown4: f32,
		unknown8: f32,
		unknown12: f32,
		unknown16: f32,
		unknown20: f32,
		unknown24: f32,
		unknown28: f32,
		unknown32: f32,
		unknown36: f32,
		unknown40: f32,
		unknown44: f32,
		unknown48: f32,
		unknown52: f32,
		unknown56: f32,
		bone_indices: [u16; 2],
		unknown64: [u8; 16],
	},
	V4 {
		unknown_7ee981a1: u16,
		unknown_9a561188: u16,
		bone_index: u32,
		unknown_6fe32955: f32,
		unknown_f4a3152b: f32,
		unknown_1cde99d0: f32,
		unknown_85048243: f32,
		unknown_a5f8324d: f32,
		unknown_1b54daab: f32,
		unknown_8de065e8: AnimationReference,
		unknown_57796a4d: f32,
		physics_shapes: Reference,
		flags: RigidBodyFlags,
		local_forces: u16,
		world_forces: WorldForces,
		priority: u32,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct RigidBody {
	data: RigidBodyData,
	physics_shapes: Vec<PhysicsShape>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum GeometricShapeType {
	CUBOID = 0,
	SPHERE = 1,
	CYLINDER = 2,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct SuperSimpleGeometricShape {
	shape: GeometricShapeType,
	bone_index: i16,
	unknown6: u16,
	matrix: Matrix4x4,
	unknown72: u32,
	unknown76: u32,
	unknown80: u32,
	unknown84: u32,
	unknown88: u32,
	unknown92: u32,
	sizes: [f32; 3],
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct AttachmentVolumeData {
	bones: [u32; 3], 
	kind: GeometricShapeType,
	matrix: Matrix4x4,
	unknown0: Reference,
	unknown1: Reference,
	sizes: [f32; 3],
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct AttachmentVolume {
	data: AttachmentVolumeData,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BillboardBehavior {
	unknown_ac35793a: [u8; 8],
	unknown_d9d65198: [u8; 4],
	bone_index: u8,
	unknown_8a35689c: u8,
	kind: u8,
	unknown_5cd4cd94: u8,
	unknown_76974b30: [u8; 4],
	unknown_b7903f37: [u8; 4],
	unknown_15901615: [u8; 4],
	unknown_8db56887: f32,
	unknown_8a4fe2ab: [u8; 4],
	unknown_51d106ab: [u8; 4],
	unknown_b410610b: [u8; 4],
	unknown_6a9fde72: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct SplineRibbon {
	unknown1: u32,
	unknown2: [u8; 16],
	tangents: [f32; 3],
	speed: AnimationReference,
	unknown_eee1a711: u32,
	bone_index: u32,
	unknown3: AnimationReference,
	unknown4: AnimationReference,
	yaw_noise_interpolation_type: u32,
	yaw_noise_amplitude: AnimationReference,
	yaw_noise_frequency: AnimationReference,
	pitch_noise_interpolation_type: u32,
	pitch_noise_amplitude: AnimationReference,
	pitch_noise_frequency: AnimationReference,
	speed_noise_interpolation_type: u32,
	speed_noise_amplitude: AnimationReference,
	speed_noise_frequency: AnimationReference,
	yaw: AnimationReference,
	pitch: AnimationReference,
}

bitflags! {
	struct RibbonFlags: u32 {
		const COLLIDE_WITH_TERRAIN = 2;
		const COLLIDE_WITH_OBJECTS = 4;
		const EDGE_FALLOFF = 8;
		const INHERIT_PARENT_VELOCITY = 16;
		const SMOOTH_SIZE = 32;
		const BEZIER_SMOOTH_SIZE = 64;
		const USE_VERTEX_ALPHA = 128;
		const SCALE_TIME_BY_PARENT = 256;
		const FORCE_LEGACY = 512;
		const USE_LOCALE_TIME = 1024;
		const SIMULATE_ON_INITIALIZATION = 2048;
		const USE_LENGTH_AND_TIME = 4096;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum RibbonType {
	PLANAR_BILLBOARDED = 0,
	PLANAR = 1,
	CYLINDER = 2,
	STAR_SHAPED = 3,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum RibbonData {
	V6 {
		bone_index: u8,
		short1: u8,
		short2: u8,
		short3: u8,
		material_reference_index: u32,
		wave_length: AnimationReference,
		unknown_e773692a: AnimationReference,
		unknown_d30655aa: i32,
		unknown_8940c27c: AnimationReference,
		unknown_c2ab76c5: AnimationReference,
		unknown_ee00ae0a: AnimationReference,
		unknown_1686c0b7: AnimationReference,
		unknown_e48f8f84: AnimationReference,
		unknown_9eba8df8: AnimationReference,
		unknown_26271cbe: u32,
		unknown_da0542fa: u32,
		unknown_76f48851: f32,
		unknown_fe32e718: f32,
		tip_offset_z: f32,
		center_bias: f32,
		unknown_cae7b5f7: f32,
		unknown_8652c547: f32,
		unknown_6a1cb998: f32,
		radius_scale: AnimationReference,
		twist: AnimationReference,
		unknown_504bc931: u32,
		unknown_8cfd3002: u32,
		unknown_2bfdc3e9: u32,
		unknown_b8fe150a: u32,
		base_coloring: AnimationReference,
		center_coloring: AnimationReference,
		tip_coloring: AnimationReference,
		stretch_amount: f32,
		unknown_b41d7d03: f32,
		stretch_limit: f32,
		unknown_c7004d01: f32,
		unknown_44a749fe: f32,
		no_end_points: u32,
		unknown_20683e1b: u32,
		unknown_df1a587b: u32,
		surface_noise_amplitude: f32,
		surface_noise_number_of_waves: f32,
		surface_noise_frequency: f32,
		surface_noise_scale: f32,
		unknown_afa854b0: u32,
		kind: RibbonType,
		filler1: i32,
		end_points: Reference,
		unknown_7e341928: AnimationReference,
		flags: RibbonFlags,
		unknown_30e0264e: u32,
		unknown_40ffe7f5: f32,
		unknown_39b23b19: u32,
		unknown_72f948ba: u32,
		direction_variation_bool: u32,
		direction_variation_amount: AnimationReference,
		direction_variation_frequency: AnimationReference,
		amplitude_variation_bool: u32,
		amplitude_variation_amount: AnimationReference,
		amplitude_variation_frequency: AnimationReference,
		length_variation_bool: u32,
		length_variation_amount: AnimationReference,
		length_variation_frequency: AnimationReference,
		radius_variation_bool: u32,
		radius_variation_amount: AnimationReference,
		radius_variation_frequency: AnimationReference,
		unknown_3e06cbc8: i32,
		unknown_4904046f: AnimationReference,
		unknown_a69b9387: AnimationReference,
		unknown_9a4a649a: AnimationReference,
		unknown_76569e33: AnimationReference,
	},
	V8 {
		bone_index: u8,
		short1: u8,
		short2: u8,
		short3: u8,
		material_reference_index: u32,
		unknown_1f9302c9: u32,
		wave_length: AnimationReference,
		unknown_e773692a: AnimationReference,
		unknown_8940c27c: AnimationReference,
		unknown_c2ab76c5: AnimationReference,
		unknown_ee00ae0a: AnimationReference,
		unknown_1686c0b7: AnimationReference,
		unknown_e48f8f84: AnimationReference,
		unknown_9eba8df8: AnimationReference,
		unknown_26271cbe: u32,
		unknown_da0542fa: u32,
		unknown_76f48851: f32,
		unknown_fe32e718: f32,
		tip_offset_z: f32,
		center_bias: f32,
		unknown_cae7b5f7: f32,
		unknown_8652c547: f32,
		unknown_6a1cb998: f32,
		unknown_bca99a4e: [u8; 12],
		radius_scale: AnimationReference,
		twist: AnimationReference,
		unknown_504bc931: u32,
		unknown_8cfd3002: u32,
		unknown_2bfdc3e9: u32,
		unknown_b8fe150a: u32,
		base_coloring: AnimationReference,
		center_coloring: AnimationReference,
		tip_coloring: AnimationReference,
		stretch_amount: f32,
		unknown_b41d7d03: f32,
		stretch_limit: f32,
		unknown_c7004d01: f32,
		unknown_44a749fe: f32,
		no_end_points: u32,
		surface_noise_amplitude: f32,
		surface_noise_number_of_waves: f32,
		surface_noise_frequency: f32,
		surface_noise_scale: f32,
		unknown_afa854b0: u32,
		kind: RibbonType,
		filler1: i32,
		end_points: Reference,
		unknown_7e341928: AnimationReference,
		flags: RibbonFlags,
		unknown_30e0264e: u32,
		unknown_40ffe7f5: f32,
		unknown_39b23b19: u32,
		unknown_72f948ba: u32,
		unknown_bccd0332: [u8; 8],
		direction_variation_bool: u32,
		direction_variation_amount: AnimationReference,
		direction_variation_frequency: AnimationReference,
		amplitude_variation_bool: u32,
		amplitude_variation_amount: AnimationReference,
		amplitude_variation_frequency: AnimationReference,
		length_variation_bool: u32,
		length_variation_amount: AnimationReference,
		length_variation_frequency: AnimationReference,
		radius_variation_bool: u32,
		radius_variation_amount: AnimationReference,
		radius_variation_frequency: AnimationReference,
		unknown_3e06cbc8: i32,
		unknown_4904046f: AnimationReference,
		unknown_a69b9387: AnimationReference,
		unknown_9a4a649a: AnimationReference,
		unknown_76569e33: AnimationReference,
	},
	V9 {
		bone_index: u8,
		short1: u8,
		short2: u8,
		short3: u8,
		material_reference_index: u32,
		unknown_1f9302c9: u32,
		wave_length: AnimationReference,
		unknown_e773692a: AnimationReference,
		unknown_8940c27c: AnimationReference,
		unknown_c2ab76c5: AnimationReference,
		unknown_ee00ae0a: AnimationReference,
		unknown_1686c0b7: AnimationReference,
		unknown_e48f8f84: AnimationReference,
		unknown_9eba8df8: AnimationReference,
		unknown_26271cbe: u32,
		unknown_da0542fa: u32,
		unknown_76f48851: f32,
		unknown_fe32e718: f32,
		tip_offset_z: f32,
		center_bias: f32,
		unknown_cae7b5f7: f32,
		unknown_8652c547: f32,
		unknown_6a1cb998: f32,
		unknown_bca99a4e: [u8; 12],
		radius_scale: AnimationReference,
		twist: AnimationReference,
		unknown_504bc931: u32,
		unknown_8cfd3002: u32,
		unknown_2bfdc3e9: u32,
		unknown_b8fe150a: u32,
		base_coloring: AnimationReference,
		center_coloring: AnimationReference,
		tip_coloring: AnimationReference,
		stretch_amount: f32,
		unknown_b41d7d03: f32,
		stretch_limit: f32,
		unknown_c7004d01: f32,
		unknown_44a749fe: f32,
		no_end_points: u32,
		unknown_fb168d8c: [u8; 4],
		surface_noise_amplitude: f32,
		surface_noise_number_of_waves: f32,
		surface_noise_frequency: f32,
		surface_noise_scale: f32,
		unknown_afa854b0: u32,
		kind: RibbonType,
		filler1: i32,
		end_points: Reference,
		unknown_7e341928: AnimationReference,
		flags: RibbonFlags,
		unknown_30e0264e: u32,
		unknown_40ffe7f5: f32,
		unknown_39b23b19: u32,
		unknown_72f948ba: u32,
		unknown_bccd0332: [u8; 8],
		direction_variation_bool: u32,
		direction_variation_amount: AnimationReference,
		direction_variation_frequency: AnimationReference,
		amplitude_variation_bool: u32,
		amplitude_variation_amount: AnimationReference,
		amplitude_variation_frequency: AnimationReference,
		length_variation_bool: u32,
		length_variation_amount: AnimationReference,
		length_variation_frequency: AnimationReference,
		radius_variation_bool: u32,
		radius_variation_amount: AnimationReference,
		radius_variation_frequency: AnimationReference,
		unknown_3e06cbc8: i32,
		unknown_4904046f: AnimationReference,
		unknown_a69b9387: AnimationReference,
		unknown_9a4a649a: AnimationReference,
		unknown_76569e33: AnimationReference,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Ribbon {
	data: RibbonData,
	end_points: Vec<SplineRibbon>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct IkJointBehavior {
	unknown1: u32,
	unknown2: u32,
	unknown3: i32,
	unknown4: u32,
	max_search_up: f32,
	max_search_down: f32,
	max_speed: f32,
	ik_goal_pos_threshold: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct ShadowBox {
	length: AnimationReference,
	width: AnimationReference,
	height: AnimationReference,
	bone_index: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum CameraData {
	V3 {
		bone_index: u32,
		name: Reference,
		field_of_view: AnimationReference,
		unknown_e35db92d: u32,
		far_clip: AnimationReference,
		near_clip: AnimationReference,
		clip2: AnimationReference,
		focal_depth: AnimationReference,
		falloff_start: AnimationReference,
		falloff_end: AnimationReference,
		depth_of_field: AnimationReference,
	},
	V5 {
		bone_index: u32,
		name: Reference,
		field_of_view: AnimationReference,
		unknown_e35db92d: u32,
		unknown_920dc1c4: [u8; 4],
		far_clip: AnimationReference,
		near_clip: AnimationReference,
		clip2: AnimationReference,
		focal_depth: AnimationReference,
		falloff_start: AnimationReference,
		falloff_end: AnimationReference,
		depth_of_field: AnimationReference,
		unknown_587dc7fb: AnimationReference,
		unknown_fff8cb33: AnimationReference,
		unknown_f726f834: AnimationReference,
		unknown_d506807d: AnimationReference,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Camera {
	data: CameraData,
	name: String,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Warp {
	unknown_861f507b: [u8; 4],
	bone_index: u32,
	unknown_cf03b586: [u8; 4],
	radius: AnimationReference,
	unknown_9306aac0: AnimationReference,
	compression_strength: AnimationReference,
	unknown_50c7f2b4: AnimationReference,
	unknown_8d9c977c: AnimationReference,
	unknown_ca6025a2: AnimationReference,
}

bitflags! {
	struct ModelFlags: u32 {
		const HAS_MESH = 1048576;
	}
	
	struct VertexFlags: u32 {
		const HAS_VERTEX_COLORS = 512;
		const HAS_VERTICES = 131072;
		const USE_UV_CHANNEL1 = 262144;
		const USE_UV_CHANNEL2 = 524288;
		const USE_UV_CHANNEL3 = 1048576;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum ModelData {
	V23 {
		name: Reference,
		flags: ModelFlags,
		sequences: Reference,
		sequence_transformation_collections: Reference,
		sequence_transformation_groups: Reference,
		unknown00: f32,
		unknown01: f32,
		unknown02: f32,
		unknown03: f32,
		sts: Reference,
		bones: Reference,
		number_of_bones_to_check_for_skin: u32,
		vflags: VertexFlags,
		vertices: Reference,
		divisions: Reference,
		bone_lookup: Reference,
		boundings: BoundsF,
		bounding_flags: u32,
		unknown04: u32,
		unknown05: u32,
		unknown06: u32,
		unknown07: u32,
		unknown08: u32,
		unknown09: u32,
		unknown10: u32,
		unknown11: u32,
		unknown12: u32,
		unknown13: u32,
		unknown14: u32,
		unknown15: u32,
		unknown16: u32,
		unknown17: u32,
		unknown18: u32,
		unknown19: u32,
		attachment_points: Reference,
		attachment_point_addons: Reference,
		lights: Reference,
		shbx_data: Reference,
		cameras: Reference,
		d: Reference,
		material_references: Reference,
		standard_materials: Reference,
		displacement_materials: Reference,
		composite_materials: Reference,
		terrain_materials: Reference,
		volume_materials: Reference,
		unknown20: Reference,
		creep_materials: Reference,
		particles: Reference,
		particle_copies: Reference,
		ribbons: Reference,
		projections: Reference,
		forces: Reference,
		warps: Reference,
		unknown21: Reference,
		rigid_bodies: Reference,
		unknown22: Reference,
		physical_joint: Reference,
		unknown23: Reference,
		ikjt_data: Reference,
		unknown24: Reference,
		parts_of_turrent_behaviors: Reference,
		turrent_behaviors: Reference,
		absolute_inverse_bone_rest_positions: Reference,
		tight_hit_test: SuperSimpleGeometricShape,
		fuzzy_hit_test_objects: Reference,
		attachment_volumes: Reference,
		attachment_volumes_addons: [Reference; 2],
		billboard_behaviors: Reference,
		tmd_data: Reference,
		unique_unknown_number: u32,
		unknown45: Reference,
	},
	V25 {
		name: Reference,
		flags: ModelFlags,
		sequences: Reference,
		sequence_transformation_collections: Reference,
		sequence_transformation_groups: Reference,
		unknown00: f32,
		unknown01: f32,
		unknown02: f32,
		unknown03: f32,
		sts: Reference,
		bones: Reference,
		number_of_bones_to_check_for_skin: u32,
		vflags: VertexFlags,
		vertices: Reference,
		divisions: Reference,
		bone_lookup: Reference,
		boundings: BoundsF,
		bounding_flags: u32,
		unknown04: u32,
		unknown05: u32,
		unknown06: u32,
		unknown07: u32,
		unknown08: u32,
		unknown09: u32,
		unknown10: u32,
		unknown11: u32,
		unknown12: u32,
		unknown13: u32,
		unknown14: u32,
		unknown15: u32,
		unknown16: u32,
		unknown17: u32,
		unknown18: u32,
		unknown19: u32,
		attachment_points: Reference,
		attachment_point_addons: Reference,
		lights: Reference,
		shbx_data: Reference,
		cameras: Reference,
		d: Reference,
		material_references: Reference,
		standard_materials: Reference,
		displacement_materials: Reference,
		composite_materials: Reference,
		terrain_materials: Reference,
		volume_materials: Reference,
		unknown20: Reference,
		creep_materials: Reference,
		volume_noise_materials: Reference,
		particles: Reference,
		particle_copies: Reference,
		ribbons: Reference,
		projections: Reference,
		forces: Reference,
		warps: Reference,
		unknown21: Reference,
		rigid_bodies: Reference,
		unknown22: Reference,
		physical_joint: Reference,
		unknown23: Reference,
		ikjt_data: Reference,
		unknown24: Reference,
		unknown_bd5657bd: [u8; 12],
		parts_of_turrent_behaviors: Reference,
		turrent_behaviors: Reference,
		absolute_inverse_bone_rest_positions: Reference,
		tight_hit_test: SuperSimpleGeometricShape,
		fuzzy_hit_test_objects: Reference,
		attachment_volumes: Reference,
		attachment_volumes_addons: [Reference; 2],
		billboard_behaviors: Reference,
		tmd_data: Reference,
		unique_unknown_number: u32,
		unknown45: Reference,
	},
	V26 {
		name: Reference,
		flags: ModelFlags,
		sequences: Reference,
		sequence_transformation_collections: Reference,
		sequence_transformation_groups: Reference,
		unknown00: f32,
		unknown01: f32,
		unknown02: f32,
		unknown03: f32,
		sts: Reference,
		bones: Reference,
		number_of_bones_to_check_for_skin: u32,
		vflags: VertexFlags,
		vertices: Reference,
		divisions: Reference,
		bone_lookup: Reference,
		boundings: BoundsF,
		bounding_flags: u32,
		unknown04: u32,
		unknown05: u32,
		unknown06: u32,
		unknown07: u32,
		unknown08: u32,
		unknown09: u32,
		unknown10: u32,
		unknown11: u32,
		unknown12: u32,
		unknown13: u32,
		unknown14: u32,
		unknown15: u32,
		unknown16: u32,
		unknown17: u32,
		unknown18: u32,
		unknown19: u32,
		attachment_points: Reference,
		attachment_point_addons: Reference,
		lights: Reference,
		shbx_data: Reference,
		cameras: Reference,
		d: Reference,
		material_references: Reference,
		standard_materials: Reference,
		displacement_materials: Reference,
		composite_materials: Reference,
		terrain_materials: Reference,
		volume_materials: Reference,
		unknown20: Reference,
		creep_materials: Reference,
		volume_noise_materials: Reference,
		splat_terrain_bake_materials: Reference,
		particles: Reference,
		particle_copies: Reference,
		ribbons: Reference,
		projections: Reference,
		forces: Reference,
		warps: Reference,
		unknown21: Reference,
		rigid_bodies: Reference,
		unknown22: Reference,
		physical_joint: Reference,
		unknown23: Reference,
		ikjt_data: Reference,
		unknown24: Reference,
		unknown_bd5657bd: [u8; 12],
		parts_of_turrent_behaviors: Reference,
		turrent_behaviors: Reference,
		absolute_inverse_bone_rest_positions: Reference,
		tight_hit_test: SuperSimpleGeometricShape,
		fuzzy_hit_test_objects: Reference,
		attachment_volumes: Reference,
		attachment_volumes_addons: [Reference; 2],
		billboard_behaviors: Reference,
		tmd_data: Reference,
		unique_unknown_number: u32,
		unknown45: Reference,
	},
	V28 {
		name: Reference,
		flags: ModelFlags,
		sequences: Reference,
		sequence_transformation_collections: Reference,
		sequence_transformation_groups: Reference,
		unknown00: f32,
		unknown01: f32,
		unknown02: f32,
		unknown03: f32,
		sts: Reference,
		bones: Reference,
		number_of_bones_to_check_for_skin: u32,
		vflags: VertexFlags,
		vertices: Reference,
		divisions: Reference,
		bone_lookup: Reference,
		boundings: BoundsF,
		bounding_flags: u32,
		unknown04: u32,
		unknown05: u32,
		unknown06: u32,
		unknown07: u32,
		unknown08: u32,
		unknown09: u32,
		unknown10: u32,
		unknown11: u32,
		unknown12: u32,
		unknown13: u32,
		unknown14: u32,
		unknown15: u32,
		unknown16: u32,
		unknown17: u32,
		unknown18: u32,
		unknown19: u32,
		attachment_points: Reference,
		attachment_point_addons: Reference,
		lights: Reference,
		shbx_data: Reference,
		cameras: Reference,
		d: Reference,
		material_references: Reference,
		standard_materials: Reference,
		displacement_materials: Reference,
		composite_materials: Reference,
		terrain_materials: Reference,
		volume_materials: Reference,
		unknown20: Reference,
		creep_materials: Reference,
		volume_noise_materials: Reference,
		splat_terrain_bake_materials: Reference,
		unknown25: Reference,
		particles: Reference,
		particle_copies: Reference,
		ribbons: Reference,
		projections: Reference,
		forces: Reference,
		warps: Reference,
		unknown21: Reference,
		rigid_bodies: Reference,
		unknown22: Reference,
		physical_joint: Reference,
		cloth_behavior: Reference,
		unknown23: Reference,
		ikjt_data: Reference,
		unknown24: Reference,
		unknown_bd5657bd: [u8; 12],
		parts_of_turrent_behaviors: Reference,
		turrent_behaviors: Reference,
		absolute_inverse_bone_rest_positions: Reference,
		tight_hit_test: SuperSimpleGeometricShape,
		fuzzy_hit_test_objects: Reference,
		attachment_volumes: Reference,
		attachment_volumes_addons: [Reference; 2],
		billboard_behaviors: Reference,
		tmd_data: Reference,
		unique_unknown_number: u32,
		unknown45: Reference,
	},
	V29 {
		name: Reference,
		flags: ModelFlags,
		sequences: Reference,
		sequence_transformation_collections: Reference,
		sequence_transformation_groups: Reference,
		unknown00: f32,
		unknown01: f32,
		unknown02: f32,
		unknown03: f32,
		sts: Reference,
		bones: Reference,
		number_of_bones_to_check_for_skin: u32,
		vflags: VertexFlags,
		vertices: Reference,
		divisions: Reference,
		bone_lookup: Reference,
		boundings: BoundsF,
		bounding_flags: u32,
		unknown04: u32,
		unknown05: u32,
		unknown06: u32,
		unknown07: u32,
		unknown08: u32,
		unknown09: u32,
		unknown10: u32,
		unknown11: u32,
		unknown12: u32,
		unknown13: u32,
		unknown14: u32,
		unknown15: u32,
		unknown16: u32,
		unknown17: u32,
		unknown18: u32,
		unknown19: u32,
		attachment_points: Reference,
		attachment_point_addons: Reference,
		lights: Reference,
		shbx_data: Reference,
		cameras: Reference,
		d: Reference,
		material_references: Reference,
		standard_materials: Reference,
		displacement_materials: Reference,
		composite_materials: Reference,
		terrain_materials: Reference,
		volume_materials: Reference,
		unknown20: Reference,
		creep_materials: Reference,
		volume_noise_materials: Reference,
		splat_terrain_bake_materials: Reference,
		unknown25: Reference,
		lens_flare_materials: Reference,
		particles: Reference,
		particle_copies: Reference,
		ribbons: Reference,
		projections: Reference,
		forces: Reference,
		warps: Reference,
		unknown21: Reference,
		rigid_bodies: Reference,
		unknown22: Reference,
		physical_joint: Reference,
		cloth_behavior: Reference,
		unknown23: Reference,
		ikjt_data: Reference,
		unknown24: Reference,
		unknown_bd5657bd: [u8; 12],
		parts_of_turrent_behaviors: Reference,
		turrent_behaviors: Reference,
		absolute_inverse_bone_rest_positions: Reference,
		tight_hit_test: SuperSimpleGeometricShape,
		fuzzy_hit_test_objects: Reference,
		attachment_volumes: Reference,
		attachment_volumes_addons: [Reference; 2],
		billboard_behaviors: Reference,
		tmd_data: Reference,
		unique_unknown_number: u32,
		unknown45: Reference,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Header {
	tag: [u8; 4],
	index_offset: u32,
	index_size: u32,
	model: Reference,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Model {
	data: ModelData,
	name: String,
	sequences: Vec<Sequence>,
	sequence_transformation_collections: Vec<SequenceTransformCollection>,
	sequence_transformation_groups: Vec<SequenceTransformGroup>,
	bones: Vec<Bone>,
	vertices: Vec<u8>,
	divisions: Vec<Division>,
	bone_lookups: Vec<u16>,
	attachments: Vec<Attachment>,
	attachment_point_addons: Vec<u16>,
	lights: Vec<Light>,
	shadow_boxes: Vec<ShadowBox>,
	cameras: Vec<Camera>,
	material_references: Vec<MaterialReference>,
	materials: Vec<Material>,
	displacement_materials: Vec<DisplacementMaterial>,
	composite_materials: Vec<CompositeMaterial>,
	terrain_materials: Vec<TerrainMaterial>,
	volume_materials: Vec<VolumeMaterial>,
	creep_materials: Vec<CreepMaterial>,
	volume_noise_materials: Vec<VolumeNoiseMaterial>,
	splat_terrain_bake_materials: Vec<SplatTerrainBakeMaterial>,
	lens_flare_materials: Vec<LensFlareMaterial>,
	particle_systems: Vec<ParticleSystem>,
	particle_copies: Vec<ParticleSystemInstance>,
	ribbons: Vec<Ribbon>,
	projections: Vec<Projection>,
	forces: Vec<Force>,
	warps: Vec<Warp>,
	rigid_bodies: Vec<RigidBody>,
	cloth_behaviors: Vec<ClothBehavior>,
	ik_joint_behaviors: Vec<IkJointBehavior>,
	turrent_behavior_parts: Vec<TurrentPart>,
	absolute_inverse_bone_rest_positions: Vec<Matrix4x4>,
	fuzzy_hit_test_objects: Vec<SuperSimpleGeometricShape>,
	attachment_volumes: Vec<AttachmentVolume>,
	attachment_volumes_addons: [Vec<u16>; 2],
	billboard_behaviors: Vec<BillboardBehavior>,
}

named!(reference<Reference>,
	do_parse!(
		entries: le_u32 >>
		index: le_u32 >>
		flags: le_u32 >>
		(Reference {
			entries: entries,
			index: index,
			flags: flags,
		})
	)
);

named!(bndsv0<BoundsF>,
	do_parse!(
		min: vec3v0 >>
		max: vec3v0 >>
		r: le_f32 >>
		(BoundsF {
			min: min,
			max: max,
			radius: r,
		})
	)
);

named!(animation_reference_header<AnimationReferenceHeader>,
	do_parse!(
	)
);

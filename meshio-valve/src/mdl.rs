use bitflags::bitflags;

use cgmath::{
	Matrix4,
	Quaternion,
	Vector3
};

bitflags! {
	struct Flags: u32 {
		const AUTOGENERATED_HITBOX = 1;
		const USES_ENV_CUBEMAP = 2;
		const FORCE_OPAQUE = 4;
		const TRANSLUCENT_TWOPASS = 8;
		const STATIC_PROP = 16;
		const USES_FB_TEXTURE = 32;
		const HAS_SHADOW_LOD = 64;
		const USES_BUMP_MAPPING = 128;
		const USE_SHADOW_LOD_MATERIALS = 256;
		const NO_FORCED_FADE = 2048;
		const FORCE_PHONEME_CROSSFADE = 4096;
		const CONSTANT_DIRECTIONAL_LIGHT_DOT = 8192;
		const FLEXES_CONVERTED = 16384;
		const BUILTIN_PREVIEW_MODE = 32768;
		const AMBIENT_BOOST = 65536;
		const DO_NOT_CAST_SHADOWS = 131072;
		const CAST_TEXTURE_SHADOWS = 262144;
		const SUBDIVISION_SURFACE = 524288;
		const VERT_ANIM_FIXED_POINT_SCALE = 1048576;
	}

	struct BoneFlags: u32 {
		const PHYSICALLY_SIMULATED = 1;
		const PHYSICS_PROCEDURAL = 2;
		const ALWAYS_PROCEDURAL = 4;
		const SCREEN_ALIGN_SPHERE = 8;
		const SCREEN_ALIGN_CYLINDER = 16;
		const CALCULATE_MASK = 31;
		const USED_BY_HITBOX = 256;
		const USED_BY_ATTACHMENT = 512;
		const USED_BY_VERTEX_LOD0 = 1024;
		const USED_BY_VERTEX_LOD1 = 2048;
		const USED_BY_VERTEX_LOD2 = 4096;
		const USED_BY_VERTEX_LOD3 = 8192;
		const USED_BY_VERTEX_LOD4 = 16384;
		const USED_BY_VERTEX_LOD5 = 32768;
		const USED_BY_VERTEX_LOD6 = 65536;
		const USED_BY_VERTEX_LOD7 = 131072;
		const USED_BY_BONE_MERGE = 262144;
		const USED_BY_ANYTHING = 524032;
	}

	struct Contents: u32 {
		const EMPTY = 0;
		const SOLID = 1;
		const WINDOW = 2;
		const AUX = 4;
		const GRATE = 8;
		const SLIME = 16;
		const WATER = 32;
		const BLOCK_LOS = 64;
		const OPAQUE = 128;
		const TEST_FOG_VOLUME = 256;
		const BLOCK_LIGHT = 1024;
		const TEAM1 = 2048;
		const TEAM2 = 4096;
		const IGNORE_NODRAW_OPAQUE = 8192;
		const MOVEABLE = 16384;
		const AREA_PORTAL = 32768;
		const PLAYER_CLIP = 65536;
		const MONSTER_CLIP = 131072;
		const CURRENT_0 = 262144;
		const CURRENT_90 = 524288;
		const CURRENT_180 = 1048576;
		const CURRENT_270 = 2097152;
		const CURRENT_UP = 4194304;
		const CURRENT_DOWN = 8388608;
		const ORIGIN = 16777216;
		const MONSTER = 33554432;
		const DEBRIS = 67108864;
		const DETAIL = 134217728;
		const TRANSLUCENT = 268435456;
		const LADDER = 536870912;
		const HITBOX = 1073741824;
	}

	struct SurfaceFlags: u16 {
		const LIGHT = 1;
		const SKY_2D = 2;
		const SKY = 4;
		const WARP = 8;
		const TRANS = 16;
		const NO_PORTAL = 32;
		const TRIGGER = 64;
		const NO_DRAW = 128;
		const HINT = 256;
		const SKIP = 512;
		const NO_LIGHT = 1024;
		const BUMP_LIGHT = 2048;
		const NO_SHADOWS = 4096;
		const NO_DECALS = 8192;
		const NO_CHOP = 16384;
		const HITBOX = 32768;
	}

	struct JiggleBoneFlags: i32 {
		const FLEXIBLE = 1;
		const RIGID = 2;
		const YAW_CONSTRAINT = 4;
		const PITCH_CONSTRAINT = 8;
		const ANGLE_CONSTRAINT = 16;
		const LENGTH_CONSTRAINT = 32;
		const BASE_SPRING = 64;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum ProceduralRule {
	AXIS_INTERP = 1,
	QUAT_INTERP = 2,
	AI_MAT_BONE = 3,
	AI_MAT_ATTACH = 4,
	JIGGLE = 5,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Bone {
	offset: u32,
	name_offset: u32,
	parent_index: i32,
	controller_index: [i32; 6],
	position: Vector3<f32>,
	quat: Quaternion<f32>,
	rotation: Vector3<f32>,
	position_scale: Vector3<f32>,
	rotation_scale: Vector3<f32>,
	pose_to_bone: Matrix4<f32>,
	alignment: Quaternion<f32>,
	flags: BoneFlags,
	procedural_rule: ProceduralRule,
	procedural_rule_offset: u32,
	physics_bone_index: u32,
	surface_prop_name_offset: u32,
	contents: Contents,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct JiggleBone {
	flags: JiggleBoneFlags,
	length: f32,
	tip_mass: f32,
	yaw_stiffness: f32,
	yaw_damping: f32,
	pitch_stiffness: f32,
	pitch_damping: f32,
	angle_stiffness: f32,
	angle_damping: f32,
	angle_limit: f32,
	min_yaw: f32,
	max_yaw: f32,
	yaw_friction: f32,
	yaw_bounce: f32,
	min_pitch: f32,
	max_pitch: f32,
	pitch_friction: f32,
	pitch_bounce: f32,
	base_mass: f32,
	base_min_left: f32,
	base_max_left: f32,
	base_left_friction: f32,
	base_min_up: f32,
	base_max_up: f32,
	base_up_friction: f32,
	base_min_forward: f32,
	base_max_forward: f32,
	base_forward_friction: f32,
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct AxisInterpBone {
	control: u32,
	pos: [Vector3<f32>; 6],
	quat: [Quaternion<f32>; 6]
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct QuatInterpBone {
	control_index: u32,
	trigger_
}

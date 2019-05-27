use bitflags::bitflags;

use cgmath::{
	Vector3,
	Vector4
};

use meshio::{
	BoundsF,
	ColorF
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct ModelData {
	pub name: String,
	pub animation_filename: String,
	pub extent: BoundsF,
	pub blend_time: u32,
}

bitflags! {
	pub(crate) struct NodeFlags: u32 {
		const HELPER = 0;
		const DONT_INHERIT_TRANSLATION = 1;
		const DONT_INHERIT_ROTATION = 2;
		const DONT_INHERIT_SCALING = 4;
		const BILLBOARDED = 8;
		const BILLBOARDED_LOCK_X = 16;
		const BILLBOARDED_LOCK_Y = 32;
		const BILLBOARDED_LOCK_Z = 64;
		const CAMERA_ANCHORED = 128;
		const BONE = 256;
		const LIGHT = 512;
		const EVENT_OBJECT = 1024;
		const ATTACHMENT = 2048;
		const PARTICLE_EMITTER = 4096;
		const COLLISION_SHAPE = 8192;
		const RIBBON_EMITTER = 16384;
		const EMITTER_USES_MDL_OR_UNSHADED = 32768;
		const EMITTER_USES_TGA_OR_SORT = 65536;
		const LINE_EMITTER = 131072;
		const UNFOGGED = 262144;
		const MODEL_SPACE = 524288;
		const XY_QUAD = 1048576;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Node {
	pub name: String,
	pub object_id: u32,
	pub parent_id: u32,
	pub translation: Vector3<f32>,
	pub rotation: Vector4<f32>,
	pub scaling: Vector3<f32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
pub(crate) enum SequenceFlag {
	LOOPING = 0,
	NON_LOOPING = 1,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Sequence {
	pub name: String,
	pub interval: [u32, 2],
	pub move_speed: f32,
	pub flags: SequenceFlag,
	pub rarity: f32,
	pub sync_point: u32,
	pub extent: BoundsF,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Texture {
	pub replaceable_id: u32,
	pub filename: String,
	pub flags: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct SoundTrack {
	pub filename: String,
	pub volume: f32,
	pub pitch: f32,
	pub flags: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
pub(crate) enum LayerFilter {
	NONE = 0,
	TRANSPARENT = 1,
	BLEND = 2,
	ADDITIVE = 3,
	ADD_ALPHA = 4,
	MODULATE = 5,
	MODULATE_2X = 6,
}

bitflags! {
	pub(crate) struct Shading {
		const UNSHADED = 1;
		const SPHERE_ENVIRONMENT_MAP = 2;
		const TWO_SIDED = 16;
		const UNFOGGED = 32;
		const NO_DEPTH_TEST = 48;
		const NO_DEPTH_SET = 64;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Layer {
	pub filter: LayerFilter,
	pub shading: Shading,
	pub texture_id: u32,
	pub texture_animation_id: u32,
	pub coord_id: u32,
	pub alpha: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Material {
	pub priority_plane: u32,
	pub flags: u32,
	pub layers: Vec<Layer>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct TextureAnimation {
	pub translation: Vector3<f32>,
	pub rotation: Vector4<f32>,
	pub scaling: Vector3<f32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
pub(crate) enum FaceTypeGroup {
	POINTS = 0,
	LINES = 1,
	LINE_LOOP = 2,
	LINE_STRIP = 3,
	TRIANGLES = 4,
	TRIANGLE_STRIP = 5,
	TRIANGLE_FAN = 6,
	QUADS = 7,
	QUAD_STRIP = 8,
	POLYGONS = 9,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Geoset {
	pub vertex_positions: Vec<Vector3<f32>>,
	pub vertex_normals: Vec<Vector3<f32>>,
	pub face_type_groups: Vec<FaceTypeGroup>,
	pub face_groups: Vec<u32>,
	pub faces: Vec<u16>,
	pub vertex_groups: Vec<u8>,
	pub matrix_groups: Vec<u32>,
	pub matrix_indices: Vec<u32>,
	pub material_id: u32,
	pub selection_group: u32,
	pub selection_flags: u32,
	pub extent: BoundsF,
	pub extents: Vec<BoundsF>,
	pub texture_coordinate_sets: Vec<Vec<Vector2f>>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct GeosetAnimation {
	pub flags: u32,
	pub color: ColorF,
	pub geoset_id: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Bone {
	pub node: Node,
	pub geoset_id: u32,
	pub geoset_animation_id: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
pub(crate) enum LightType {
	OMNI = 0,
	DIRECTIONAL = 1,
	AMBIENT = 2,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Light {
	pub node: Node,
	pub kind: LightType,
	pub attenuation_start: f32,
	pub attenuation_end: f32,
	pub color: ColorF,
	pub intensity: f32,
	pub ambient_color: ColorF,
	pub ambient_intensity: f32,
	pub visibility: f32,
}

pub(crate) type Helper = Node;

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Attachment {
	pub node: Node,
	pub path: String,
	pub id: u32,
	pub visibility: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct ParticleEmitter {
	pub node: Node,
	pub rate: f32,
	pub gravity: f32,
	pub longitude: f32,
	pub latitude: f32,
	pub spawn_model_filename: String,
	pub lifespan: f32,
	pub initial_velocity: f32,
	pub speed: f32,
	pub visibility: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
pub(crate) enum ParticleEmitter2Filter {
	BLEND = 0,
	ADDITIVE = 1,
	MODULATE = 2,
	MODULATE_2X = 3,
	ALPHA_KEY = 4,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
pub(crate) enum HeadOrTail {
	HEAD = 0,
	TAIL = 1,
	BOTH = 2,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct ParticleEmitter2 {
	pub node: Node,
	pub speed: f32,
	pub variation: f32,
	pub latitude: f32,
	pub gravity: f32,
	pub lifespan: f32,
	pub rate: f32,
	pub length: f32,
	pub width: f32,
	pub filter: ParticleEmitter2Filter,
	pub rows: u32,
	pub columns: u32,
	pub head_or_tail: HeadOrTail,
	pub tail_length: f32,
	pub time: f32,
	pub segment_color: [ColorF; 3],
	pub segment_alpha: [u8; 3],
	pub segment_scaling: Vector3<f32>,
	pub head_interval: [u32; 3],
	pub head_decay_interval: [u32; 3],
	pub tail_interval: [u32; 3],
	pub tail_decay_interval: [u32; 3],
	pub texture_id: u32,
	pub squirt: u32,
	pub priority_plane: u32,
	pub replaceable_id: u32,
	pub visibility: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct RibbonEmitter {
	pub node: Node,
	pub height_above: f32,
	pub height_below: f32,
	pub color: ColorF,
	pub lifespan: f32,
	pub texture_slot: u32,
	pub rate: u32,
	pub rows: u32,
	pub columns: u32,
	pub material_id: u32,
	pub gravity: f32,
	pub visibility: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Event {
	pub node: Node,
	pub global_sequence_id: u32,
	pub tracks: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) struct Camera {
	pub name: String,
	pub position: Vector3<f32>,
	pub field_of_view: f32,
	pub far_clipping_plane: f32,
	pub near_clipping_plane: f32,
	pub target_position: Vector3<f32>,
	pub translation: Vector3<f32>,
	pub target_translation: Vector3<f32>,
	pub rotation: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
pub(crate) enum Shape {
	CUBE = 0,
	PLANE = 1,
	SPHERE = 2,
	CYLINDER = 3,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) enum Vertices {
	Cube([Vector3<f32>; 2]),
	Plane([Vector3<f32>; 2]),
	Sphere([Vector3<f32>; 3]),
	Cylinder([Vector3<f32>; 2]),
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) Collision {
	pub node: Node,
	pub kind: Shape,
	pub vertices: Vertices,
	pub radius: Option<f32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub(crate) Model {
	pub version: u32,
	pub model: ModelData,
	pub sequences: Vec<Sequence>,
	pub global_sequences: Vec<u32>,
	pub textures: Vec<Texture>,
	pub sound_tracks: Vec<SoundTrack>,
	pub points: Vec<Vector3<f32>>,
	pub materials: Vec<Material>,
	pub texture_animations: Vec<TextureAnimation>,
	pub geosets: Vec<Geoset>,
	pub geoset_animations: Vec<GeosetAnimation>,
	pub bones: Vec<Bone>,
	pub lights: Vec<Light>,
	pub helpers: Vec<Helper>,
	pub attachments: Vec<Attachment>,
	pub particle_emitters: Vec<ParticleEmitter>,
	pub particle_emitters2: Vec<ParticleEmitter2>,
	pub ribbon_emitters: Vec<RibbonEmitter>,
	pub objects: Vec<Event>,
	pub cameras: Vec<Camera>,
	pub shapes: Vec<Collision>,
}

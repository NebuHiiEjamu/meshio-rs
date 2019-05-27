use bitflags::bitflags;

use cgmath::{
	Matrix4,
	Vector3
};

use nom::{
	alt,
	count,
	delimited,
	digit,
	do_parse,
	flat_map,
	float,
	many0,
	many1,
	named,
	opt,
	parse_to,
	recognize,
	switch,
	tag,
	value,
	ws
};

use meshio::{
	BoundsF,
	ColorF
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Scene {
	filename: String,
	first_frame: u32,
	last_frame: u32,
	frame_speed: f32,
	ticks_per_frame: f32,
	background: ColorF,
	ambient: ColorF,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct NodeTransformationMatrix {
	name: String,
	inherit_pos: Vector3<f32>,
	inherit_rot: Vector3<f32>,
	inherit_scl: Vector3<f32>,
	rows: Matrix4<f32>,
	pos: Vector3<f32>,
	rot_axis: Vector3<f32>,
	rot_angle: f32,
	scale: Vector3<f32>,
	scale_axis: Vector3<f32>,
	scale_axis_ang: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Node {
	name: String,
	parent: Option<String>,
	tranformation_matrices: Vec<NodeTransformationMatrix>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Face {
	a: u32,
	b: u32,
	c: u32,
	ab: u32,
	bc: u32,
	ca: u32,
	smoothing: u32,
	material: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Normal {
	face: Vector3<f32>,
	vertices: [Vector3<f32>; 3],
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Mesh {
	time_value: u32,
	vertices: Vec<Vector3<f32>>,
	faces: Vec<Face>,
	tvertices: Vec<Vector3<f32>>,
	tfaces: Vec<Vector3<u32>>,
	normals: Vec<Normal>,
	vertex_colors: Vec<ColorF>,
	cfaces: Vec<Vector3<u32>,
}

bitflags! {
	struct GeomFlags: u8 {
		const MOTION_BLUR = 1;
		const CAST_SHADOW = 2;
		const RECEIVE_SHADOW = 4;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Geom {
	name: String,
	node: Node,
	meshes: Vec<Mesh>,
	animations: Vec<TmAnimation>,
	flags: GeomFlags,
	wireframe_color: ColorF,
	material: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct ControlRotSample {
	frame: u32,
	rotation: Vector4<f32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct ControlPosSample {
	frame: u32,
	rotation: Vector3<f32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct TmAnimation {
	name: String,
	rot_samples: Vec<ControlRotSample>,
	pos_samples: Vec<ControlPosSample>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum CameraType {
	TARGET,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Camera {
	name: String,
	node: Node,
	kind: CameraType,
	time_value: u32,
	near: f32,
	far: f32,
	fov: f32,
	tdist: f32,
	animations: Vec<TmAnimation>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum HelperClass {
	DUMMY,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Helper {
	node: Node,
	class: HelperClass,
	bounding_box: BoundsF,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum MaterialClass {
	STANDARD,
	MULTI_SUB,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Shading {
	BLINN,
	PHONG,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Falloff {
	IN,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum MaterialXpType {
	FILTER,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum MapClass {
	BITMAP,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum MapType {
	SCREEN,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum BitmapFilter {
	PYRAMIDAL,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Map {
	name: String,
	class: MapClass,
	sub_no: u32,
	amount: f32,
	bitmap: String,
	kind: MapType,
	uvw_offset: Vector2<f32>,
	uvw_tiling: Vector2<f32>,
	uvw_angle: f32,
	uvw_blur: f32,
	uvw_blur_offset: f32,
	uvw_noise_amt: f32,
	uvw_noise_size: f32,
	uvw_noise_level: u32,
	bitmap_filter: BitmapFilter,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Material {
	name: String,
	class: MaterialClass,
	ambient: ColorF,
	diffuse: ColorF,
	specular: ColorF,
	shine: f32,
	shine_strength: f32,
	transparency: f32,
	wire_size: f32,
	shading: Shading,
	xp_falloff: f32,
	self_illum: f32,
	falloff: Falloff,
	xp_type: MaterialXpType,
	diffuse_map: Option<Map>,
	shine_strength_map: Option<Map>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum LightType {
	DIRECTIONAL,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Shadows {
	OFF,
	RAYTRACED,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum SpotShape {
	CIRCLE,
}

bitflags! {
	struct LightFlags: u8 {
		const USE_LIGHT = 1;
		const USE_GLOBAL = 2;
		const ABS_MAP_BIAS = 4;
		const OVERSHOOT = 8;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Light {
	name: String,
	node: Node,
	shadows: Shadows,
	flags: LightFlags,
	spot_shape: SpotShape,
	time_value: u32,
	color: ColorF,
	intens: f32,
	aspect: f32,
	hotspot: f32,
	falloff: f32,
	tdist: f32,
	map_bias: f32,
	map_range: f32,
	map_size: u32,
	ray_bias: f32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Model {
	version: u32,
	comment: String,
	scene: Scene,
	geoms: Vec<Geom>,
	cameras: Vec<Camera>,
	helpers: Vec<Helper>,
	materials: Vec<Material>,
	lights: Vec<Light>,
}

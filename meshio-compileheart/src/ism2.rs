use cgmath::{
	Vector2,
	Vector3,
	Vector4
};

use nom::{
	named,
	parse_to,
	take_until
};

use meshio::{
	ColorB,
	LocalizedStringMap
};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum SectionType {
	UNKNOWN = 0,
	BONE_DATA = 3,
	MESH_HEADER = 10,
	MESH_BLOCK_DATA = 11,
	BONE_TRANSLATION = 20,
	BONE_SCALE = 21,
	STRING_DATA = 33,
	TEXTURE_DATA = 46,
	BONE_MATRIX_DATA = 50,
	POLYGON_DATA = 69,
	POLYGON_BLOCK = 70,
	SURFACE_OFFSETS = 76,
	VERTEX_BLOCK_DATA = 89,
	BONE_TRANSFORMS = 91,
	BONE_PARENTING = 92,
	BONE_TRANSFORM_X = 93,
	BONE_TRANSFORM_Y = 94,
	BONE_TRANSFORM_Z = 95,
	MATERIAL_DATA = 97,
	JOINT_ORIENT_X = 103,
	JOINT_ORIENT_Y = 104,
	JOINT_ORIENT_Z = 105,
	BOUNDING_BOX = 110,
	COLLISION_FLAG = 112,
	COLLISION_RADIUS = 113,
	PHYSICS_FLAG = 114,
	PHYSICS_RADIUS = 115,
	PHYSICS_COST = 116,
	PHYSICS_MASS = 117,
	PHYSICS_EXPAND = 118,
	PHYSICS_SHAPE_MEMORY = 119,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Header {
	version: [u8; 4],
	unknown3: u32,
	unknown4: u32,
	file_size: u32,
	unknown7: u32,
	unknown8: u32,
	section_types: Vec<SectionType>,
	section_offsets: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BoneBlockHeader {
	section: SectionType,
	header_size: u32,
	string_offsets: [u32; 2],
	offsets: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Surface {
	section: SectionType,
	header_size: u32,
	total: u32,
	material_name_offset: u32,
	texture_name_offset: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum BoneSection {
	Translation(Vector3<f32>),
	Scale(Vector3<f32>),
	SurfaceOffsets {
		header_size: u32,
		total: u32,
		name_offset: u32,
		unknown1: u32,
		unknown2: u32,
		offsets: Vec<u32>,
	},
	Transforms {
		length: u32,
		total: u32,
	},
	TransformX(Vector4<f32>),
	TransformY(Vector4<f32>),
	TransformZ(Vector4<f32>),
	JointOrientX(Vector4<f32>),
	JointOrientY(Vector4<f32>),
	JointOrientZ(Vector4<f32>),
	CollisionFlag {
		unknown1: i32,
		unknown2: i32,
		unknown3: f32,
	},
	CollisionRadius {
		unknown1: i32,
		unknown2: i32,
		unknown3: f32,
	},
	PhysicsFlag {
		unknown1: i32,
		unknown2: i32,
		unknown3: i32,
	},
	PhysicsRadius {
		unknown1: i32,
		unknown2: i32,
		unknown3: f32,
	},
	PhysicsCost {
		unknown1: i32,
		unknown2: i32,
		unknown3: f32,
	},
	PhysicsMass {
		unknown1: i32,
		unknown2: i32,
		unknown3: f32,
	},
	PhysicsExpand {
		unknown1: i32,
		unknown2: i32,
		unknown3: f32,
	},
	PhysicsShapeMemory {
		unknown1: i32,
		unknown2: i32,
		unknown3: f32,
	},
	Unknown1 {
		unknown1: i32,
		unknown2: i32,
		unknown3: f32,
	},
	Unknown2 {
		unknown1: i16,
		unknown2: i16,
		unknown3: i32,
		unknown4: f32,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BoneHeader {
	section: SectionType,
	header_size: u32,
	name_offsets: [u32; 2],
	unknown1: u32,
	unknown2: u32,
	parent_offset: u32,
	unknown4: u32,
	unknown5: u32,
	unknown6: u32,
	id: i32,
	unknown8: u32,
	unknown9: u32,
	unknown10: u32,
	unknown11: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Bone {
	header: BoneHeader,
	sections: Vec<BoneSection>,
	surfaces: Vec<Surface>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct MeshBlockHeader {
	section: SectionType,
	header_size: u32,
	header_offsets: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct MeshHeader {
	section: SectionType,
	header_size: u32,
	unknown1: u32,
	unknown2: u32,
	unknown3: u32,
	unknown4: u32,
	unknown5: u32,
	offsets: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct PolygonBlockHeader {
	section: SectionType,
	data_size: u32,
	name_offset: u32,
	blank: u32,
	unknown1: u16,
	unknown2: u16,
	offsets: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum MeshData {
	Polygon {
		size: u32,
		types: [u16; 2],
		blank: u32,
		faces: Vec<Vector3<u32>>,
	},
	Vertex {
		length: u32,
		types: [u16; 2],
		size: u32,
		unknown: u32,
		data_offsets: Vec<u32>,
		uvw_offsets: Vec<u32>,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct VertexHeader {
	unknown1: u32,
	unknown2: u32,
	unknown3: u32,
	unknown4: u32,
	unknown5: u32,
	start_offset: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct UvWeighting {
	Type1 {
		v: Vector3<f32>,
		n: [Vector3<u16>; 2],
		uv: Vector2<u16>,
		rgba: ColorB,
	},
	Type2 {
		bones: [u16; 8],
		weights: [f32; 8],
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Vertex {
	header: VertexHeader,
	uvw: UvWeighting,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Mesh {
	header: MeshHeader,
	polygon_block_header: PolygonBlockHeader,
	data: Vec<MeshData>,
	vertices: Vec<Vertex>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BoundingBox {
	section: SectionType,
	unknown: [u8; 8],
	min: Vector4<f32>,
	max: Vector4<f32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct StringBlockHeader {
	section: SectionType,
	header_size: u32,
	offsets: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct TextureBlockHeader {
	section: SectionType,
	header_size: u32,
	offsets: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Texture {
	section: SectionType,
	data_string_offsets: [u32; 3],
	name_offset: u32,
	unknown1: u32,
	unknown2: u32,
	unknown3: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BoneMatrixData1 {
	section: SectionType,
	header_size: u32,
	unknown1: u32,
	unknown2: u32,
	next_offset: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BoneMatrixData2 {
	section: SectionType,
	header_size: u32,
	unknown1: u32,
	unknown2: u32,
	m: Matrix4x4f,
	offsets: [u32; 2],
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BoneMatrixData3 {
	section: SectionType,
	header_size: u32,
	unknown1: u32,
	unknown2: u16,
	unknown3: u16,
	unknown4: u32,
	unknown5: u32,
	unknown6: u32,
	m: Vec<Matrix4x4f>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct BoneMatrix {
	unknown1: BoneMatrixData1,
	unknown2: BoneMatrixData2,
	unknown3: BoneMatrixData3,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct MaterialBlockHeader {
	section: SectionType,
	header_size: u32,
	offsets: Vec<u32>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct MaterialData13 {
	section: SectionType,
	header_size: u32,
	total: u32,
	name_offset: u32,
	string_offsets: [u32; 2],
	blank: u32,
	offset108: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct MaterialData108 {
	section: SectionType,
	header_size: u32,
	total: u32,
	offset106: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct MaterialData106 {
	section: SectionType,
	header_size: u32,
	total: u32,
	string_offset: u32,
	blank: u32,
	offset107: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct MaterialData107 {
	section: SectionType,
	header_size: u32,
	total: u32,
	unknown1: u32,
	unknown2: u32,
	blank: u32,
	next_offset: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Material {
	unknown13: MaterialData13,
	unknown108: Option<MaterialData108>,
	unknown106: Option<MaterialData106>,
	unknown107: Option<[MaterialData107; 2]>,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Model {
	header: Header,
	bone_block_header: BoneMetaHeader,
	bones: Vec<Bone>,
	mesh_block_header: MeshBlockHeader,
	meshes: Vec<Mesh>,
	bounding_box: BoundingBox,
	string_block_header: StringBlockHeader,
	strings: Vec<String>,
	texture_block_header: TextureBlockHeader,
	textures: Vec<Texture>,
	bone_matrix: BoneMatrix,
	material_block_header: MaterialBlockHeader,
	materials: Vec<Material>,
}

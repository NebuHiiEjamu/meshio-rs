extern crate nom;
extern crate bitflags;

use bitflags::bitflags;

pub mod m3 {
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Reference {
		pub entries: u32,
		pub index: u32,
		pub flags: u32,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Vec2 {
		pub x: f32,
		pub y: f32,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Vec3 {
		pub x: f32,
		pub y: f32,
		pub z: f32,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Vec4 {
		pub x: f32,
		pub y: f32,
		pub z: f32,
		pub w: f32,
	}
	
	pub type Quat = Vec4;
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Matrix44 {
		pub x: Vec4,
		pub y: Vec4,
		pub z: Vec4,
		pub w: Vec4,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Vector3AsFixed8 {
		pub x: u8,
		pub y: u8,
		pub z: u8,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Vector2AsInt16 {
		pub x: i16,
		pub y: i16,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Col {
		pub blue: u8,
		pub green: u8,
		pub red: u8,
		pub alpha: u8,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Bnds {
		pub min: Vec3,
		pub max: Vec3,
		pub radius: f32,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct AnimationReferenceHeader {
		pub interpolation_type: u16,
		pub anim_flags: u16,
		pub anim_id: u32,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum AnimationReference {
		Vector3 {
			header: AnimationReferenceHeader,
			init: Vec3,
			null: Vec3,
			unknown: i32,
		},
		Vector2 {
			header: AnimationReferenceHeader,
			init: Vec2,
			null: Vec2,
			unknown: i32,
		},
		Quaternion {
			header: AnimationReferenceHeader,
			init: Quat,
			null: Quat,
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
			init: Col,
			null: Col,
			unknown: i32,
		},
		Bnds {
			header: AnimationReferenceHeader,
			init: Bnds,
			null: Bnds,
			unknown: i32,
		},
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct IndexEntry {
		pub tag: [u8; 4],
		pub offset: u32,
		pub repetitions: u32,
		pub version: u32,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum Vertex {
		Format182007D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			uv: Vector2AsInt16,
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format1820261 {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			color: Col,
			uv: Vector2AsInt16,
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format182027D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			color: Col,
			uv: Vector2AsInt16,
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format1860061 {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			uvs: [Vector2AsInt16; 2],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format186007D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			uvs: [Vector2AsInt16; 2],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format1860261 {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			color: Col,
			uvs: [Vector2AsInt16; 2],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format186027D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			color: Col,
			uvs: [Vector2AsInt16; 2],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format18E0061 {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			uvs: [Vector2AsInt16; 3],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format18E007D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			uvs: [Vector2AsInt16; 3],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format18E0261 {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			color: Col,
			uvs: [Vector2AsInt16; 3],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format18E027D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			color: Col,
			uvs: [Vector2AsInt16; 3],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format19E0061 {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			uvs: [Vector2AsInt16; 4],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format19E007D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			uvs: [Vector2AsInt16; 4],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format19E0261 {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			color: Col,
			uvs: [Vector2AsInt16; 4],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format19E027D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			color: Col,
			uvs: [Vector2AsInt16; 4],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
		Format4182007D {
			position: Vec3,
			bone_weights: [u8; 4],
			bone_lookup_indices: [u8; 4],
			normal: Vector3AsFixed8,
			sign: u8,
			uvs: [Vector2AsInt16; 2],
			tangent: Vector3AsFixed8,
			unused: u8,
		},
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub enum Evnt {
		V0 {
			name: Reference,
			unknown_76ff2940: i32,
			unknown_97b2ea12: i16,
			unknown_294cc515: u16,
			matrix: Matrix44,
			unknown_7275bfa0: u32,
			unknown_a20de85: u32,
			unknown_76cf1db0: u32,
		},
		V1 {
			name: Reference,
			unknown_76ff2940: i32,
			unknown_97b2ea12: i16,
			unknown_294cc515: u16,
			matrix: Matrix44,
			unknown_7275bfa0: u32,
			unknown_2a403b40: Reference,
			unknown_76cf1db0: u32,
		},
		V2 {
			name: Reference,
			unknown_76ff2940: i32,
			unknown_97b2ea12: i16,
			unknown_294cc515: u16,
			matrix: Matrix44,
			unknown_7275bfa0: u32,
			unknown_2a403b40: Reference,
			unknown_76cf1db0: u32,
			unknown_e5f1b2a7: u32,
		},
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct AnimationBlock {
		pub frames: Reference,
		pub flags: u32,
		pub fend: u32,
		pub keys: Reference,
	}
	
	pub type Sdev = AnimationBlock;
	pub type Sd2v = AnimationBlock;
	pub type Sd3v = AnimationBlock;
	pub type Sdr3 = AnimationBlock;
	pub type Sdcc = AnimationBlock;
	pub type Sds6 = AnimationBlock;
	pub type Sdu6 = AnimationBlock;
	pub type Sdu3 = AnimationBlock;
	pub type Sd4q = AnimationBlock;
	pub type Sdfg = AnimationBlock;
	pub type Sdmb = AnimationBlock;
	
	bitflags! {
		pub struct BoneFlags: u32 {
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
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Bone {
		pub d: i32,
		pub name: Reference,
		pub flags: BoneFlags,
		pub parent: i16,
		pub s: u16,
		pub location: AnimationReference,
		pub rotation: AnimationReference,
		pub scale: AnimationReference,
		pub ar: AnimationReference,
	}
	
	#[derive(Clone,Debug,PartialEq,Eq)]
	pub struct Stc {
		
	}
}

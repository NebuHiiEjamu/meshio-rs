extern crate nom;

pub mod m3 {
	pub struct Reference {
		pub entries: u32,
		pub index: u32,
		pub flags: u32,
	}
	
	pub struct Vec2 {
		pub x: f32,
		pub y: f32,
	}
	
	pub struct Vec3 {
		pub x: f32,
		pub y: f32,
		pub z: f32,
	}
	
	pub struct Vec4 {
		pub x: f32,
		pub y: f32,
		pub z: f32,
		pub w: f32,
	}
	
	pub type Quat = Vec4;
	
	pub struct Matrix44 {
		pub x: Vec4,
		pub y: Vec4,
		pub z: Vec4,
		pub w: Vec4,
	}
	
	pub struct Vector3AsFixed8 {
		pub x: u8,
		pub y: u8,
		pub z: u8,
	}
	
	pub struct Vector3AsInt16 {
		pub x: i16,
		pub y: i16,
	}
	
	pub struct Col {
		pub blue: u8,
		pub green: u8,
		pub red: u8,
		pub alpha: u8,
	}
	
	pub struct Bnds {
		pub min: Vec3,
		pub max: Vec3,
		pub radius: f32,
	}
	
	pub struct AnimationReferenceHeader {
		pub interpolation_type: u16,
		pub anim_flags: u16,
		pub anim_id: u32,
	}
	
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
	
	pub struct IndexEntry {
		pub tag: [u8; 4],
		pub offset: u32,
		pub repetitions: u32,
		pub version: u32,
	}
	
	pub enum Vertex {
		
	}
}

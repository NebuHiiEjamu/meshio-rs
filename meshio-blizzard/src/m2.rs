use bitflags::bitflags;

use meshio::{
	BoundsF
};

bitflags! {
	struct GlobalFlags: u32 {
		const TILT_X = 1;
		const TILT_Y = 2;
		const USE_TEXTURE_COMBINER_COMBOS = 8;
		const LOAD_PHYS_DATA = 32;
		const CAMERA_RELATED = 256;
		const NEW_PARTICLE_RECORD = 512;
		const TEXTURE_TRANSFORMS_USE_BONE_SEQUENCES = 2048;
	}
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Array {
	size: u32,
	offset: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum TrackBase {
	V1 {
		interpolation_type: u16,
		global_sequence: u16,
		interpolation_ranges: Array,
		timestamps: Array,
	},
	V2 {
		interpolation_type: u16,
		global_sequence: u16,
		timestamps: Array,
	},
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Track {
	base: TrackBase,
	values: Array,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct PartTrack {
	times: Array,
	values: Array,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Range {
	minimum: u32,
	maximum: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
#[repr(u32)]
enum Version {
	PRE_RELEASE,
	CLASSIC,
	BURNING_CRUSADE,
	WRATH_OF_THE_LICH_KING,
	CATACLYSM,
	MISTS_OF_PANDARIA,
	WARLORDS_OF_DRAENOR,
	LEGION,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct AnimFileId {
	anim_id: u16,
	sub_anim_id: u16,
	file_id: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct Exp2 {
	source: u32,
	unk1: u32,
	unk2: u32,
	unk3: u32,
	unk4: u32,
	unk5: u32,
	unk6: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
struct LodData {
	unk0: u16,
	count: u16,
	unk2: u32,
	unk3: u32,
	unk4: u32,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
enum Header {
	V1 {
		version: Version,
		name: Array,
		global_flags: GlobalFlags,
		global_loops: Array,
		sequences: Array,
		sequence_lookups: Array,
		playable_animation_lookup: Array,
		bones: Array,
		key_b
	},
}

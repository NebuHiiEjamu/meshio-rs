use std::collections::HashMap;

/// Languages primarily used as keys for [`LocalizedStringMap`]
#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub enum Language {
	English,
	Japanese
}

/// Type alias for a hash map that maps strings by [`Language`]
pub type LocalizedStringMap = HashMap<Language, String>;

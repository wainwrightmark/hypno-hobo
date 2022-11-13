use anyhow::bail;
use multimap::MultiMap;
use regex::internal::Char;
use serde::{Deserialize, Serialize};
use std::{
    num::NonZeroU8,
    sync::Arc,
};

use super::{character_stats::CharacterStats, BackgroundChoice, ClassLevel};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub name: Arc<String>,
    pub background: BackgroundChoice,

    pub levels: Arc<Vec<ClassLevel>>,

    pub stats: CharacterStats,

    pub backstory: Arc<String>,
}


impl Character{
    // pub fn to_markdown(&self)-> String{
    //     format!(r#"
    //         {}  
    //         {}
    //     "#, self.name, self.background.0) .to_string()
    // }
}
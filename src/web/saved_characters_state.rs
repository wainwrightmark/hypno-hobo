use std::default;

use crate::{data::data_dictionary::Character, web::prelude::*};

use itertools::Itertools;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use yewdux::prelude::*;


#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct SavedCharactersState {
    pub characters: Vec<Character>,
}

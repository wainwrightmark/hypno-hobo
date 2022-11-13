use std::sync::Arc;

use crate::data::*;

use serde::{Deserialize, Serialize};

use crate::data::data_dictionary::DataRow;
use yewdux::prelude::*;

use csv::{Reader, ReaderBuilder};
use itertools::Itertools;

use bytes::Buf;

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct CreationState {
    pub stage: Stage,
    pub character: Character,
    #[serde(skip)]
    pub dictionary: Arc<DataDictionary>,
}

impl Default for CreationState {
    fn default() -> Self {
        let character = Character {
            name: Default::default(),
            background: BackgroundChoice(Default::default()),
            levels: Default::default(),
            backstory: Default::default(),
            stats: Default::default(),
        };
        Self {
            stage: Default::default(),
            character,
            dictionary: Default::default(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub enum Stage {
    #[default]
    Name,
    Background,
    Levels,

    Class {
        class: Arc<String>,
    },
    ClassFeature {
        class: Arc<String>,
        feature: Option<Arc<String>>,
    },
    Stats,
    Backstory,

    Finished,
}

impl CreationState {
    pub fn change_stage(&self, stage: Stage) -> Self {
        Self {
            stage,
            character: self.character.clone(),
            dictionary: self.dictionary.clone(),
        }
    }

    pub async fn setup() {
        let dict = Self::create_dict().await.unwrap();
        Dispatch::<Self>::new().reduce_mut(|x| x.dictionary = dict.into());
    }

    pub async fn create_dict() -> Result<DataDictionary, anyhow::Error> {
        let url = "https://docs.google.com/spreadsheets/d/e/2PACX-1vS6n_bJgC33CE8b9o51niMx5g8WDr56x9-XIY_i61C5Bw0Cbs0vjEo0M4zIXPdX-mp8ooWx0fOICuMZ/pub?gid=0&single=true&output=tsv";
        let result = reqwest::get(url).await;
        let response = result?;
        //let text = data.text().await?;

        let bytes = response.bytes().await?;

        let mut rdr: Reader<_> = ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(bytes.reader());

        let rows: Vec<DataRow> = rdr.deserialize().try_collect()?;

        let data: DataDictionary = rows.try_into()?;

        Ok(data)
    }
}

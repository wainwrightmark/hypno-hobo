use std::sync::Arc;

use crate::data::*;

use serde::{Deserialize, Serialize};

use crate::data::data_dictionary::DataRow;
use yewdux::prelude::*;

use csv::{Reader, ReaderBuilder};
use itertools::Itertools;

use bytes::Buf;

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

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Default, Debug)]
pub enum Stage {
    #[default]
    Name,
    Background,
    Levels,

    Class {
        class: Arc<Class>,
    },
    ClassFeature {
        feature: Arc<ClassFeature>,
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

    pub fn mutate_character<F>(&self, func: F) -> Self
    where
        F: FnOnce(&mut Character),
    {
        let mut character = self.character.clone();
        func(&mut character);

        Self {
            stage: self.stage.clone(),
            character,
            dictionary: self.dictionary.clone(),
        }
    }

    pub fn add_level(&self, class: Arc<String>, feature: Option<Arc<String>>) -> Self {
        self.mutate_character(|character| {
            let mut levels = character.levels.as_ref().clone();
            levels.push(ClassLevel {
                class: class.clone(),
                feature: feature.clone(),
            });
            character.levels = levels.into();
        })
        .change_stage(Stage::Levels)
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

    pub fn try_get_class(&self, class_name: Arc<String>) -> Option<Class> {
        self.dictionary
            .classes
            .iter()
            .cloned()
            .find(|c| c.name == class_name)
    }

    pub fn get_level_of_class(&self, class_name: Arc<String>) -> usize {
        self.character
            .levels
            .iter()
            .filter(|x| x.class == class_name)
            .count()
    }

    pub fn first_feature_of_next_class_level(
        &self,
        class_name: Arc<String>,
    ) -> Result<Option<Arc<ClassFeature>>, ()> {
        if let Some(class) = self.try_get_class(class_name) {
            let level = self.get_level_of_class(class.name);
            if let Some(first_feature) = class.features_by_level.get(&(1 + level)) {
                Ok(Some(first_feature.clone()))
            } else {
                Ok(None) //No features
            }
        } else {
            Err(()) //Class not found
        }
    }
}

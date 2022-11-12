use anyhow::bail;
use multimap::MultiMap;
use serde::{Deserialize, Serialize};
use std::{num::NonZeroU8, sync::Arc};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Character {
    pub name: Arc<String>,
    pub background: Arc<Option<BackgroundChoice>>,
    pub levels: Arc<Vec<ClassLevel>>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterStats {
    pub strength: AbilityScore,
    pub dexterity: AbilityScore,
    pub constitution: AbilityScore,
    pub wisdom: AbilityScore,
    pub intelligence: AbilityScore,
    pub charisma: AbilityScore,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbilityScore(u8);

impl Default for AbilityScore {
    fn default() -> Self {
        Self(10)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackgroundChoice(Arc<String>);

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassLevel {
    pub class_name: Arc<String>,
    pub feature: Option<Arc<String>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImageUrl(String);

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DataDictionary {
    pub classes: Vec<Class>,
    pub backgrounds: Vec<Background>,
}

impl TryFrom<Vec<DataRow>> for DataDictionary {
    type Error = anyhow::Error;

    fn try_from(rows: Vec<DataRow>) -> Result<Self, Self::Error> {
        let mut backgrounds: Vec<Background> = Default::default();
        let mut classes: Vec<Class> = Default::default();
        let mut class_features: MultiMap<Arc<String>, ClassFeature> = Default::default();

        for row in rows {
            match row.data_row_type {
                DataRowType::Class => {
                    let class = Class {
                        name: row.name.into(),
                        description: row.description,
                        image: row.image,
                        features_by_level: Default::default(),
                    };
                    classes.push(class);
                }
                DataRowType::ClassFeature => {
                    let Some(class) = row.parent else{
                        bail!("Class Feature {} has no parent set", row.name );
                    };
                    let Some(level) = row.level else{
                        bail!("Class Feature {} has no level set", row.name );
                    };

                    let feature = ClassFeature {
                        name: row.name.into(),
                        description: row.description,
                        image: row.image,
                        level,
                    };
                    class_features.insert(class.into(), feature);
                }
                DataRowType::Background => {
                    let background = Background {
                        name: row.name.into(),
                        description: row.description,
                        image: row.image,
                    };
                    backgrounds.push(background);
                }
            }
        }

        for class in classes.iter_mut() {
            if let Some(features) = class_features.remove(&class.name) {
                let mm: MultiMap<_, _> = features
                    .into_iter()
                    .map(|x| (x.level, Arc::new(x)))
                    .collect();
                class.features_by_level = mm.into();
            }
        }

        for (class_name, feature) in class_features {
            bail!(
                "Feature '{}' belongs to class '{}' which does not exist",
                feature[0].name,
                class_name
            );
        }

        Ok(Self {
            classes,
            backgrounds,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Class {
    pub name: Arc<String>,
    pub description: String,
    pub image: Option<ImageUrl>,

    pub features_by_level: Arc<MultiMap<NonZeroU8, Arc<ClassFeature>>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassFeature {
    pub name: Arc<String>,
    pub description: String,
    pub image: Option<ImageUrl>,
    pub level: NonZeroU8,
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Background {
    pub name: Arc<String>,
    pub description: String,
    pub image: Option<ImageUrl>,
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataRow {
    pub name: String,
    pub data_row_type: DataRowType,
    pub parent: Option<String>,
    pub level: Option<NonZeroU8>,
    pub image: Option<ImageUrl>,
    pub description: String,
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataRowType {
    Class,
    ClassFeature,
    Background,
}

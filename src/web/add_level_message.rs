use std::{num::NonZeroU8, sync::Arc};

use crate::components::{ButtonProperty, StoreProperty};

use super::{CreationState, Stage};

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct AddLevelMessage {}

impl StoreProperty for AddLevelMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        match state.stage {
            Stage::Levels => {
                if let Some(class) = state.dictionary.classes.first() {
                    Some(
                        state
                            .change_stage(Stage::Class {
                                class: class.clone().into(),
                            })
                            .into(),
                    )
                } else {
                    None //No possible classes
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AddLevelOfClassMessage(pub Arc<String>);

impl StoreProperty for AddLevelOfClassMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        match state.stage {
            Stage::Levels => {
                match state.first_feature_of_next_class_level(self.0.clone()) {
                    Ok(Some(feature)) => Some(
                        state
                            .change_stage(Stage::ClassFeature {
                                feature: feature.clone(),
                            })
                            .into(),
                    ),
                    Ok(None) => Some(state.add_level(self.0.clone(), None).into()),
                    Err(_) => None, //class does not exist
                }
            }
            _ => None,
        }
    }
}

impl ButtonProperty for AddLevelMessage {
    fn button_text(&self, _state: std::rc::Rc<Self::State>) -> &'static str {
        "Add Level"
    }
}
impl ButtonProperty for AddLevelOfClassMessage {
    fn button_text(&self, _state: std::rc::Rc<Self::State>) -> &'static str {
        "+"
    }
}

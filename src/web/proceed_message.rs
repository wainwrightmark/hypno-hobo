use core::panic;

use regex::internal::Char;
use yewdux::prelude::Dispatch;

use crate::{components::{properties::ButtonProperty, StoreProperty}, data::Character};

use super::creation_state::{CreationState, Stage};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ProceedMessage {}

impl StoreProperty for ProceedMessage {
    type State = CreationState;
    type Value = ();

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        use Stage::*;
        match state.stage.clone() {

            CharacterSelect=>{
                Some(state.change_stage(Name).into())
            }

            Name => {
                if state.character.name.is_empty() {
                    None
                } else {
                    Some(state.change_stage(Background).into())
                }
            }
            Background => Some(state.change_stage(Levels).into()),
            Levels => {
                if state.character.levels.is_empty() {
                    None
                } else {
                    Some(state.change_stage(Stats).into())
                }
            }
            Class { class } => {
                match state.first_feature_of_next_class_level(class.name.clone()) {
                    Ok(Some(feature)) => Some(
                        state
                            .change_stage(Stage::ClassFeature {
                                feature: feature.clone(),
                            })
                            .into(),
                    ),
                    Ok(None) => Some(state.add_level(class.name.clone(), None).into()),
                    Err(_) => None, //class does not exist
                }
            }
            ClassFeature { feature } => Some(
                state
                    .add_level(feature.class_name.clone(), Some(feature.name.clone()))
                    .into(),
            ),
            Stats => {
                if state.character.stats.is_legal() {
                    Some(state.change_stage(Backstory).into())
                } else {
                    None
                }
            }
            Backstory => {
                if state.character.backstory.is_empty() {
                    None
                } else {
                    Some(state.change_stage(Finished).into())
                }
            }
            Finished => {

                let mut new_characters = state.saved_characters.as_ref().clone();
                new_characters.push(state.character.clone());

                let new_state = CreationState{
                    stage: Stage::CharacterSelect,
                    character: Character::new(),
                    dictionary: state.dictionary.clone(),
                    saved_characters: new_characters.into()

                };

                Some(new_state.into())
            },
        }
    }

    fn get_current_value(&self, _: &Self::State) -> Self::Value {
        ()
    }
}

impl ButtonProperty for ProceedMessage {
    fn button_text(&self, state: std::rc::Rc<Self::State>) -> &'static str {
        match state.stage{
            Stage::CharacterSelect => "New Character",
            Stage::Finished => "Save",
            _ => "Proceed",
        }
    }
}

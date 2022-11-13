use crate::{
    components::{messages::ButtonMessage, StoreMessage},
    data::data_dictionary::{Character, ClassLevel},
};

use super::creation_state::{CreationState, Stage};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ProceedMessage {}

impl StoreMessage for ProceedMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        use Stage::*;
        match state.stage.clone() {
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
            Class { class } => Some(
                state
                    .change_stage(ClassFeature {
                        class,
                        feature: None,
                    })
                    .into(),
            ),
            ClassFeature { class, feature } => {
                let mut levels = state.character.levels.as_ref().clone();
                levels.push(ClassLevel {
                    class_name: class,
                    feature,
                });
                let character = Character {
                    levels: levels.into(),
                    ..state.character.clone()
                };

                Some(
                    CreationState {
                        stage: Levels,
                        character: character.into(),
                        dictionary: state.dictionary.clone(),
                    }
                    .into(),
                )
            }
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
            Finished => None,
        }
    }
}

impl ButtonMessage for ProceedMessage {
    fn button_text(&self, state: std::rc::Rc<Self::State>) -> &'static str {
        "Proceed"
    }
}

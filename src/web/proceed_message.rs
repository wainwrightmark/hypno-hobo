use crate::components::{properties::ButtonProperty, StoreProperty};

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
            Finished => None,
        }
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        ()
    }
}

impl ButtonProperty for ProceedMessage {
    fn button_text(&self, _state: std::rc::Rc<Self::State>) -> &'static str {
        "Proceed"
    }
}

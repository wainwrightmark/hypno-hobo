use crate::{
    components::{messages::ButtonMessage, StoreMessage},
    data::data_dictionary::{Character, ClassLevel},
};

use super::creation_state::{CreationState, Stage};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct BackMessage {}

impl StoreMessage for BackMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        use Stage::*;
        match state.stage.clone() {
            Name => None,
            Background => Some(state.change_stage(Name).into()),
            Levels => Some(state.change_stage(Background).into()),
            Class { .. } => Some(state.change_stage(Levels).into()),
            ClassFeature { class, .. } => Some(state.change_stage(Class { class }).into()),
            Stats => Some(state.change_stage(Levels).into()),
            Backstory => Some(state.change_stage(Stats).into()),
            Finished => Some(state.change_stage(Backstory).into()),
        }
    }
}

impl ButtonMessage for BackMessage {
    fn button_text(&self, _: std::rc::Rc<Self::State>) -> &'static str {
        "Back"
    }
}

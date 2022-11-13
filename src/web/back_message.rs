use crate::{
    components::{properties::ButtonProperty, StoreProperty},
};

use super::creation_state::{CreationState, Stage};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct BackMessage {}

impl StoreProperty for BackMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        use Stage::*;
        match state.stage.clone() {
            Name => None,
            Background => Some(state.change_stage(Name).into()),
            Levels => Some(state.change_stage(Background).into()),
            Class { .. } => Some(state.change_stage(Levels).into()),
            ClassFeature { feature } => Some(state.change_stage(Levels).into()),
            Stats => Some(state.change_stage(Levels).into()),
            Backstory => Some(state.change_stage(Stats).into()),
            Finished => Some(state.change_stage(Backstory).into()),
        }
    }
}

impl ButtonProperty for BackMessage {
    fn button_text(&self, _: std::rc::Rc<Self::State>) -> &'static str {
        "Back"
    }
}

use crate::{
    components::{properties::ButtonProperty, StoreProperty},
};

use super::creation_state::{CreationState, Stage};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct BackMessage {}

impl StoreProperty for BackMessage {
    type State = CreationState;
    type Value = ();

    fn try_apply(&self,value: &Self::Value, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        use Stage::*;
        match state.stage.clone() {
            CharacterSelect => None,
            Name => Some(state.change_stage(CharacterSelect).into()),
            Background => Some(state.change_stage(Name).into()),
            Levels => Some(state.change_stage(Background).into()),
            Class { .. } => Some(state.change_stage(Levels).into()),
            ClassFeature { feature } => Some(state.change_stage(Levels).into()),
            Stats => Some(state.change_stage(Levels).into()),
            Backstory => Some(state.change_stage(Stats).into()),
            Finished => Some(state.change_stage(Backstory).into()),
        }
    }

    
    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        ()
    }
}

impl ButtonProperty for BackMessage {
    fn button_text(&self, _: std::rc::Rc<Self::State>) -> &'static str {
        "Back"
    }
}

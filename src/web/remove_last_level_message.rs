use itertools::Itertools;

use crate::components::{ButtonProperty, StoreProperty};

use super::CreationState;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RemoveLastLevelMessage {}

impl StoreProperty for RemoveLastLevelMessage {
    type State = CreationState;
    type Value = ();

    fn try_apply(&self, value: &Self::Value, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        if state.character.levels.len() == 0 {
            None
        } else {
            let mut character = state.character.clone();
            let levels = character
                .levels
                .iter()
                .take(state.character.levels.len() - 1)
                .cloned()
                .collect_vec();
            character.levels = levels.into();
            Some(CreationState {
                stage: state.stage.clone(),
                dictionary: state.dictionary.clone(),
                character,
                saved_characters: state.saved_characters.clone()
            }.into())
        }
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        ()
    }
}

impl ButtonProperty for RemoveLastLevelMessage {
    fn button_text(&self, _: std::rc::Rc<Self::State>) -> &'static str {
        "-"
    }
}

use crate::components::*;
use crate::data::Character;
use crate::web::*;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct LoadCharacterMessage(pub Arc<Character>);

impl ButtonProperty for LoadCharacterMessage {
    fn button_text(&self, state: std::rc::Rc<Self::State>) -> &'static str {
        "Load"
    }
}

impl StoreProperty for LoadCharacterMessage {
    type State = CreationState;
    type Value = ();

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        Some(
            Self::State {
                character: self.0.as_ref().clone(),
                dictionary: state.dictionary.clone(),
                stage: Stage::Finished,
                saved_characters: state.saved_characters.clone(),
            }
            .into(),
        )
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        ()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteCharacterMessage(pub Arc<Character>);

impl ButtonProperty for DeleteCharacterMessage {
    fn button_text(&self, state: std::rc::Rc<Self::State>) -> &'static str {
        "Delete"
    }
}

impl StoreProperty for DeleteCharacterMessage {
    type State = CreationState;
    type Value = ();

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        let index = state
            .saved_characters
            .iter()
            .position(|x| x == self.0.as_ref())?;

        let mut new_characters = state.saved_characters.as_ref().clone();
        new_characters.remove(index);

        Some(
            Self::State {
                character: state.character.clone(),
                dictionary: state.dictionary.clone(),
                stage: Stage::CharacterSelect,
                saved_characters: new_characters.into(),
            }
            .into(),
        )
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        ()
    }
}

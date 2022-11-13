use super::CreationState;
use crate::components::{StoreProperty, TextProperty};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SetNameProperty();

impl StoreProperty for SetNameProperty {
    type State = CreationState;
    type Value = String;

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        Some(
            state
                .mutate_character(|x| x.name = value.clone().into())
                .into(),
        )
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        state.character.name.as_ref().clone()
    }
}

impl TextProperty for SetNameProperty {
    fn placeholder(&self) -> &'static str {
        "Character Name"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SetBackstoryProperty();

impl StoreProperty for SetBackstoryProperty {
    type State = CreationState;
    type Value = String;

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        state.character.backstory.as_ref().clone()
    }

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        Some(
            state
                .mutate_character(|x| x.backstory = value.clone().into())
                .into(),
        )
    }
}

impl TextProperty for SetBackstoryProperty {
    fn placeholder(&self) -> &'static str {
        "Backstory"
    }
}

use std::str::FromStr;

use crate::components::{StoreProperty, TextProperty};

use super::CreationState;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetNameProperty(String);

impl StoreProperty for SetNameProperty{
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
            Some(state.mutate_character(|x|x.name = self.0.clone().into()).into())
    }
}

impl FromStr for SetNameProperty{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl TextProperty for SetNameProperty{
    fn get_current_value(state: &Self::State) -> String {
        state.character.name.as_ref().clone()
    }

    fn placeholder()-> &'static str {
        "Character Name"
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetBackstoryProperty(String);

impl StoreProperty for SetBackstoryProperty{
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
            Some(state.mutate_character(|x|x.backstory = self.0.clone().into()).into())
    }
}

impl FromStr for SetBackstoryProperty{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl TextProperty for SetBackstoryProperty{
    fn get_current_value(state: &Self::State) -> String {
        state.character.backstory.as_ref().clone()
    }

    fn placeholder()-> &'static str {
        "Backstory"
    }
}
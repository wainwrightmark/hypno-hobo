use crate::{
    components::{NumberProperty, StoreProperty},
    data::{Ability, AbilityScore},
};

use super::CreationState;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StatProperty(pub Ability);

impl StoreProperty for StatProperty {
    type State = CreationState;

    type Value = u8;

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        Some(state.mutate_character(|x| x.stats[self.0] = AbilityScore(value.clone())).into())
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        state.character.stats[self.0].0
    }
}

impl NumberProperty<u8> for StatProperty {
    fn min(&self) -> u8 {
        2
    }

    fn max(&self) -> u8 {
        20
    }

    fn step(&self) -> u8 {
        1
    }

    fn label(&self) -> String {
        self.0.to_string()
    }
}

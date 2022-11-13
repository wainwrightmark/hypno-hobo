use std::sync::Arc;

use crate::components::{ButtonMessage, StoreMessage};

use super::{CreationState, Stage};

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct AddLevelOfClassMessage(pub Option<Arc<String>>);

impl StoreMessage for AddLevelOfClassMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        match state.stage {
            Stage::Levels => {
                if let Some(class) = self.0.clone() {
                    Some(
                        state
                            .change_stage(Stage::ClassFeature {
                                class,
                                feature: None,
                            })
                            .into(),
                    )
                } else {
                    if let Some(class) = state.dictionary.classes.first() {
                        Some(
                            state
                                .change_stage(Stage::Class {
                                    class: class.name.clone(),
                                })
                                .into(),
                        )
                    } else {
                        None //There is no class to choose from
                    }
                }
            }
            _ => None,
        }
    }
}

impl ButtonMessage for AddLevelOfClassMessage {
    fn button_text(&self, state: std::rc::Rc<Self::State>) -> &'static str {
        "Add Level"
    }
}

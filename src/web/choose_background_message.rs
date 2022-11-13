use yew::html;
use yewdux::{prelude::Dispatch, store::Reducer};

use crate::{components::*, data::*};

use super::{creation_state::CreationState, proceed_message::ProceedMessage};

#[derive(Clone, PartialEq)]
pub struct ChooseBackgroundMessage(BackgroundChoice);

impl StoreProperty for ChooseBackgroundMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        Some(state.mutate_character(|x|x.background = self.0.clone()).into())
    }
}

impl CarouselProperty for ChooseBackgroundMessage {
    fn get_values(state: &Self::State) -> Vec<Self> {
        let vec: Vec<_> = state
            .dictionary
            .backgrounds
            .iter()
            .map(|x| Self(x.into()))
            .collect();
        vec
    }

    fn get_current_value(state: &Self::State) -> Self {
        Self(state.character.background.clone())
    }

    fn get_html(&self, state: &Self::State, classes: yew::Classes) -> yew::Html {
        let background: Background = {
            if let Some(b) = state
                .dictionary
                .backgrounds
                .iter()
                .find(|x| x.name == self.0 .0)
            {
                b.clone()
            } else {
                if let Some(b_first) = state.dictionary.backgrounds.first() {
                    b_first.clone()
                } else {
                    let b_default = Background {
                        name: "Unknown".to_string().into(),
                        description: Default::default(),
                        image: None,
                    };
                    b_default
                }
            }
        };

        let onclick = Dispatch::<CreationState>::new()
            .apply_callback(|_| PropertyReducer(ProceedMessage::default()));
        html!(
            <div class={classes}>
                <h5 class="title" style="text-align: center;">{background.name}</h5>
                {
                    if let Some(url) = background.image{
                        html!(
                            <img class="image" onclick={onclick}
                            src={format!("{}", url.0)  }
                                  />
                        )
                    }
                    else{
                        html!(<></>)
                    }
                }
                    <p class="description" >
                    {background.description}
                    </p>
            </div>
        )
    }

    // fn as_button() -> Option<Box<dyn yewdux::store::Reducer<Self::State>>> {
    //     Some(Box::new(PropertyReducer(ProceedMessage{})))
    // }
}

impl Reducer<CreationState> for ChooseBackgroundMessage {
    fn apply(self, state: std::rc::Rc<CreationState>) -> std::rc::Rc<CreationState> {
        if state.character.background == self.0 {
            state
        } else {
            let character = Character {
                name: state.character.name.clone(),
                background: self.0,
                stats: state.character.stats.clone(),
                levels: state.character.levels.clone(),
                backstory: state.character.backstory.clone(),
            };
            let s = CreationState {
                stage: state.stage.clone(),
                character,
                dictionary: state.dictionary.clone(),
            };
            s.into()
        }
    }
}

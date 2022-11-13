use yew::html;
use yewdux::{prelude::Dispatch};

use crate::{components::*, data::*};

use super::{creation_state::CreationState, proceed_message::ProceedMessage};

#[derive(Clone, PartialEq, Default, Copy)]
pub struct ChooseBackgroundMessage();

impl StoreProperty for ChooseBackgroundMessage {
    type State = CreationState;
    type Value = BackgroundChoice;

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        Some(
            state
                .mutate_character(|x| x.background = value.clone())
                .into(),
        )
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        state.character.background.clone()
    }
}

impl CarouselProperty for ChooseBackgroundMessage {
    fn get_values(&self, state: &Self::State) -> Vec<Self::Value> {
        let vec: Vec<_> = state
            .dictionary
            .backgrounds
            .iter()
            .map(|x| x.into())
            .collect();
        vec
    }

    fn get_html(
        &self,
        value: &Self::Value,
        state: &Self::State,
        classes: yew::Classes,
    ) -> yew::Html {
        let background: Background = {
            if let Some(b) = state
                .dictionary
                .backgrounds
                .iter()
                .find(|x| x.name == value.0)
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

        let onclick = Dispatch::<CreationState>::new().apply_callback(|_| PropertyReducer::<
            ProceedMessage,
        > {
            property: Default::default(),
            value: (),
        });
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

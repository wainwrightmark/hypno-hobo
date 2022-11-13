use std::sync::Arc;

use itertools::Itertools;
use yew::html;
use yewdux::prelude::Dispatch;

use crate::components::*;
use crate::data::Class;
use crate::web::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ChooseClassMessage(pub Arc<Class>);

impl StoreProperty for ChooseClassMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        if let Stage::Class { .. } = state.stage.clone() {
            Some(
                state
                    .change_stage(Stage::Class {
                        class: self.0.clone(),
                    })
                    .into(),
            )
        } else {
            None
        }
    }
}

impl CarouselProperty for ChooseClassMessage {
    fn get_values(state: &Self::State) -> Vec<Self> {
        state
            .dictionary
            .classes
            .iter()
            .map(|x| Self(x.clone().into()))
            .collect_vec()
    }

    fn get_current_value(state: &Self::State) -> Self {
        if let Stage::Class { class } = state.stage.clone() {
            Self(class.clone())
        } else {
            Self(
                Class {
                    name: "Unknown".to_string().into(),
                    description: Default::default(),
                    image: None,
                    features_by_level: Default::default(),
                }
                .into(),
            ) //not good
        }
    }

    fn get_html(&self, state: &Self::State, classes: yew::Classes) -> yew::Html {
        let onclick = Dispatch::<CreationState>::new()
            .apply_callback(|_| PropertyReducer(ProceedMessage::default()));
        html!(
            <div class={classes}>
                <h5 class="title" style="text-align: center;">{self.0.name.clone()}</h5>
                {
                    if let Some(url) = self.0.image.clone(){
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
                    {self.0.description.clone()}
                    </p>
            </div>
        )
    }
}

use std::sync::Arc;

use itertools::Itertools;
use yew::html;
use yewdux::prelude::Dispatch;

use crate::components::*;
use crate::data::Class;
use crate::web::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChooseClassMessage();

impl StoreProperty for ChooseClassMessage {
    type State = CreationState;
    type Value = Arc<Class>;

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        if let Stage::Class { .. } = state.stage.clone() {
            Some(
                state
                    .change_stage(Stage::Class {
                        class: value.clone(),
                    })
                    .into(),
            )
        } else {
            None
        }
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        if let Stage::Class { class } = state.stage.clone() {
            class.clone()
        } else {
            Class {
                name: "Unknown".to_string().into(),
                description: Default::default(),
                image: None,
                features_by_level: Default::default(),
            }
            .into()
            //not good
        }
    }
}

impl CarouselProperty for ChooseClassMessage {
    fn get_values(&self, state: &Self::State) -> Vec<Self::Value> {
        state
            .dictionary
            .classes
            .iter()
            .map(|x| x.clone().into())
            .collect_vec()
    }

    fn get_html(
        &self,
        value: &Self::Value,
        state: &Self::State,
        classes: yew::Classes,
    ) -> yew::Html {
        let onclick = Dispatch::<CreationState>::new().apply_callback(|_| PropertyReducer::<
            ProceedMessage,
        > {
            property: Default::default(),
            value: (),
        });
        html!(
            <div class={classes}>
                <h5 class="title" style="text-align: center;">{value.name.clone()}</h5>
                {
                    if let Some(url) = value.image.clone(){
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
                    {value.description.clone()}
                    </p>
            </div>
        )
    }
}

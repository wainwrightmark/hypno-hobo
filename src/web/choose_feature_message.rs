use core::panic;
use std::sync::Arc;

use itertools::Itertools;
use yew::html;
use yewdux::prelude::Dispatch;

use crate::components::*;
use crate::data::{Class, ClassFeature};
use crate::web::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ChooseFeatureMessage(pub Arc<ClassFeature>);

impl StoreProperty for ChooseFeatureMessage {
    type State = CreationState;

    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>> {
        if let Stage::ClassFeature { feature: _ } = state.stage.clone() {
            Some(
                state
                    .change_stage(Stage::ClassFeature {
                        feature: self.0.clone(),
                    })
                    .into(),
            )
        } else {
            None
        }
    }
}

impl CarouselProperty for ChooseFeatureMessage {
    fn get_values(state: &Self::State) -> Vec<Self> {
        if let Stage::ClassFeature { feature } = state.stage.clone() {
            if let Some(c) = state
                .dictionary
                .classes
                .iter()
                .find(|x| x.name == feature.class_name)
            {
                //log::info!("Features by level");
                let level = state.get_level_of_class(c.name.clone());
                if let Some(vec) = c.features_by_level.get_vec(&(level + 1)) {
                    //log::info!("Found features");
                    return vec.iter().map(|x| Self(x.clone())).collect();
                }
            }
        }
        //log::info!("No features found in {:?}", state.stage, );
        vec![]
    }

    fn get_current_value(state: &Self::State) -> Self {
        if let Stage::ClassFeature { feature } = state.stage.clone() {
            Self(feature)
        } else {
            Self(
                ClassFeature {
                    name: "Unknown".to_string().into(),
                    class_name: "Unknown".to_string().into(),
                    description: Default::default(),
                    image: None,
                    level: 1,
                }
                .into(),
            )
        }
    }

    fn get_html(&self, _: &Self::State, classes: yew::Classes) -> yew::Html {
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

    // fn as_button() -> Option<Box<dyn yewdux::store::Reducer<Self::State>>> {
    //     Some(Box::new(PropertyReducer(ProceedMessage{})))
    // }
}

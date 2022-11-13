use std::sync::Arc;
use yew::html;
use yewdux::prelude::Dispatch;

use crate::components::*;
use crate::data::{ ClassFeature};
use crate::web::*;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct ChooseFeatureMessage();

impl StoreProperty for ChooseFeatureMessage {
    type State = CreationState;
    type Value = Arc<ClassFeature>;

    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>> {
        if let Stage::ClassFeature { feature: _ } = state.stage.clone() {
            Some(
                state
                    .change_stage(Stage::ClassFeature {
                        feature: value.clone(),
                    })
                    .into(),
            )
        } else {
            None
        }
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value {
        if let Stage::ClassFeature { feature } = state.stage.clone() {
            feature
        } else {
            ClassFeature {
                name: "Unknown".to_string().into(),
                class_name: "Unknown".to_string().into(),
                description: Default::default(),
                image: None,
                level: 1,
            }
            .into()
        }
    }
}

impl CarouselProperty for ChooseFeatureMessage {
    fn get_values(&self, state: &Self::State) -> Vec<Self::Value> {
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
                    return vec.iter().map(|x| x.clone()).collect();
                }
            }
        }
        //log::info!("No features found in {:?}", state.stage, );
        vec![]
    }

    fn get_html(&self, value: &Self::Value, _: &Self::State, classes: yew::Classes) -> yew::Html {
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

    // fn as_button() -> Option<Box<dyn yewdux::store::Reducer<Self::State>>> {
    //     Some(Box::new(PropertyReducer(ProceedMessage{})))
    // }
}

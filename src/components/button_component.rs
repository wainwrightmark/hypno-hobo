use crate::components::PropertyReducer;

use super::{properties::ButtonProperty, BasicProps};
use yew::prelude::*;
use yewdux::prelude::*;


#[function_component(ButtonComponent)]
pub fn button<T: ButtonProperty + 'static>(props: &BasicProps<T>) -> Html {
    let (state, dispatch) = use_store::<T::State>();

    let property = props.property.clone();

    let onclick = dispatch.apply_callback(move |_| PropertyReducer{property: property.clone(), value: ()} );
    let disabled = props.property.is_disabled(&(), state.clone());
    let button_text = props.property.button_text(state);

    html! {
        <button aria-label={button_text} class={props.classes.clone()} style={props.style.clone()} disabled={disabled} onclick={onclick}>{{button_text}}</button>
    }
}

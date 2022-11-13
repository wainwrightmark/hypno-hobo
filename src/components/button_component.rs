use crate::components::MessageReducer;

use super::messages::ButtonMessage;
use yew::prelude::*;
use yewdux::prelude::*;
#[derive(Properties, PartialEq)]
pub struct ButtonProps<T: ButtonMessage> {
    pub message: T,

    #[prop_or_default()]
    pub classes: Classes,
    #[prop_or_default()]
    pub style: AttrValue,
}

#[function_component(ButtonComponent)]
pub fn button<T: ButtonMessage + 'static>(props: &ButtonProps<T>) -> Html {
    let (state, dispatch) = use_store::<T::State>();

    let message = props.message.clone();

    let onclick = dispatch.apply_callback(move |_| MessageReducer(message.clone()));
    let disabled = props.message.is_disabled(state.clone());
    let button_text = props.message.button_text(state);

    html! {
        <button aria-label={button_text} class={props.classes.clone()} style={props.style.clone()} disabled={disabled} onclick={onclick}>{{button_text}}</button>
    }
}

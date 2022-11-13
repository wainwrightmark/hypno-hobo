use crate::components::PropertyReducer;

use super::{BasicProps, TextProperty};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(TextComponent)]
pub fn text_box<T: TextProperty + 'static>(props: &BasicProps<T>) -> Html {
    //TODO disabled
    //TODO aria-invalid
    //TODO debounce
    let value = {
        let property = props.property.clone();
        use_selector(move |state: &T::State| property.get_current_value(state))
    };

    let oninput = {
        let property = props.property.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Dispatch::<T::State>::new().apply(PropertyReducer {
                property: property.clone(),
                value: input.value(),
            })
        })
    };

    html!(
        <input type="text"  name="input" placeholder={props.property.placeholder()} class={props.classes.clone()} style={props.style.clone()} value={value.to_string()} {oninput}/>
    )
}

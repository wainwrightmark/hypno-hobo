use crate::components::PropertyReducer;

use super::{BasicProps, TextProperty};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(TextComponent)]
pub fn text_box<T: TextProperty + 'static>(props: &BasicProps) -> Html {
    //TODO disabled
    //TODO aria-invalid
    //TODO debounce
    let value = use_selector(|state: &T::State| T::get_current_value(state));

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        if let Ok(string) = T::from_str(input.value().as_str()) {
            Dispatch::<T::State>::new().apply(PropertyReducer(string))
        };
    });

    html!(
        <input type="text"  name="input" placeholder={T::placeholder()} class={props.classes.clone()} style={props.style.clone()} value={value.to_string()} {oninput}/>
    )
}

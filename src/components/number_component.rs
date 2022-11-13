use std::fmt::Display;

use crate::components::PropertyReducer;

use super::{BasicProps, NumberProperty};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(NumberComponent)]
pub fn text_box<N: num::Num + Display + 'static, T: NumberProperty<N> + 'static>(
    props: &BasicProps<T>,
) -> Html {
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

            if let Ok(n) = N::from_str_radix(input.value().as_str(), 10) {
                Dispatch::<T::State>::new().apply(PropertyReducer {
                    property: property.clone(),
                    value: n,
                })
            }
        })
    };
    let property = props.property.clone();

    html!(
        <label for="input"> {property.label()}
        <input
         type="number"
         name="input"
         min = {property.min().to_string()}
         max = {property.max().to_string()}
         step = {property.step().to_string()}
          class={props.classes.clone()}
          style={props.style.clone()}
           value={value.to_string()}
            {oninput}/>
            </label>

    )
}

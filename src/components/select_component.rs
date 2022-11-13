use itertools::Itertools;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::*;

#[function_component(SelectComponent)]
pub fn select_component<T: SelectMessage>(props: &BasicProps) -> Html {
    let (state, dispatch) = use_store::<T::State>();

    let onchange = dispatch.apply_callback(|e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let s = input.value();

        let v = T::parse_repr(s.as_str());
        MessageReducer(v)
    });
    let current_value = T::get_current_value(state.as_ref());

    let options = T::get_values(state.as_ref())
        .into_iter()
        .map(|value| {
            let selected = value == &current_value;
            let disabled = T::is_disabled(value, state.clone());
            html!(  <option value={value.repr()} {selected} disabled={disabled}> {value.text()}  </option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange} class={props.classes.clone()} style={props.style.clone()}>
            {options}
        </select>
    )
}

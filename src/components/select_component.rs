use itertools::Itertools;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::*;

#[function_component(SelectComponent)]
pub fn select_component<T: SelectProperty + 'static>(props: &BasicProps<T>) -> Html {
    let (state, dispatch) = use_store::<T::State>();
    
    let onchange = 
    
    {
        let property = props.property.clone();
        dispatch.apply_callback(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let s = input.value();
    
            let v = T::parse_repr(s.as_str());
            PropertyReducer{property: property.clone(), value: v}
        })
    };
    
    let current_value = props.property.get_current_value(state.as_ref());

    let options = T::get_values(state.as_ref())
        .into_iter()
        .map(|value| {
            let selected = value == &current_value;
            let disabled = props.property.is_disabled(value, state.clone());
            let repr = T::repr(value);
            let text = T::text(value);
            html!(  <option value={repr} {selected} disabled={disabled}> {text}  </option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange} class={props.classes.clone()} style={props.style.clone()}>
            {options}
        </select>
    )
}

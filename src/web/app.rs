use itertools::Itertools;

use std::str::FromStr;

use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{data::data_dictionary::Character, web::prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    //let character = Character{};
    html! {
        <div class="site">
            <div class="container-sm" >
            <span> {"Hello World"} </span>
            </div>
        </div>
    }
}

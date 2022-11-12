use yew::prelude::*;
use yewdux::prelude::*;

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

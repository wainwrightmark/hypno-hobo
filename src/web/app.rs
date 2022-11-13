use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::*;
use crate::data::*;
use crate::web::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="site">
            <div class="container" >
            <Content />
            </div>
        </div>
    }
}

#[function_component(Content)]
pub fn content() -> Html {
    let stage = use_selector(|state: &CreationState| state.stage.clone());

    use crate::web::creation_state::Stage::*;
    match stage.as_ref() {
        Name => html!(<NameControl/>),
        Background => html!(<BackgroundControl />),
        Levels => html!(<LevelsControl />),
        Class { class } => todo!(),
        ClassFeature { class, feature } => todo!(),
        Stats => todo!(),
        Backstory => todo!(),
        Finished => todo!(),
    }
}

#[function_component(NameControl)]
pub fn name_control() -> Html {
    let value = use_selector(|state: &CreationState| state.character.name.clone());

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        Dispatch::<CreationState>::new().reduce_mut(|x| x.character.name = input.value().into());
    });

    html!(
        <div>
            <input type="text" id="name-input" name="input" placeholder="Character Name" value={value.to_string()} {oninput}/>
            <ButtonComponent<ProceedMessage> message={ProceedMessage::default()}/>
        </div>
    )
}

#[function_component(BackgroundControl)]
pub fn background_control() -> Html {
    html!(
        <div>
        <CarouselComponent<ChooseBackgroundMessage> />
        <ButtonComponent<ProceedMessage> message={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> message={BackMessage::default()}/>
        </div>
    )
}

#[function_component(LevelsControl)]
pub fn levels_control() -> Html {
    //TODO show class details - allow adding and subtracting levels by class
    let character = use_selector(|state: &CreationState| state.character.clone());
    let rows = character
        .levels
        .iter()
        .map(|x| {
            html!(
                <tr>
                // <td>{x.} </td>
                </tr>
            )
        })
        .collect::<Html>();

    html!(
        <div>

        <table>
            {rows}
        </table>

        <CarouselComponent<ChooseBackgroundMessage> />
        <ButtonComponent<AddLevelOfClassMessage> message={AddLevelOfClassMessage::default()}/>
        <ButtonComponent<ProceedMessage> message={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> message={BackMessage::default()}/>

        </div>
    )
}

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::*;

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
        Class { class: _ } => html!(<ClassControl />),
        ClassFeature { feature: _ } => html!(<FeatureControl />),
        Stats => html!(<StatsControl />),
        Backstory => html!(<BackstoryControl />),
        Finished => todo!(),
    }
}

#[function_component(NameControl)]
pub fn name_control() -> Html {

    html!(
        <div>
            <TextComponent<SetNameProperty> />
            <ButtonComponent<ProceedMessage> message={ProceedMessage::default()}/>
        </div>
    )
}

#[function_component(BackstoryControl)]
pub fn backstory_control() -> Html {

    html!(
        <div>
            <TextComponent<SetBackstoryProperty> />
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
        .enumerate()
        .map(|(index, x)| {
            let is_last = index + 1 == character.levels.len();
            html!(
                <tr>
                <td>{x.class.clone()} </td>
                <td>{x.feature.clone()} </td>
                <td><ButtonComponent<AddLevelOfClassMessage> message={AddLevelOfClassMessage(x.class.clone())}  /> </td>
                <td>
                {if is_last {html!(<ButtonComponent<RemoveLastLevelMessage> message={RemoveLastLevelMessage::default()} />)} else{html!(<> </>)}}
                </td>
                </tr>
            )
        })
        .collect::<Html>();

    html!(
        <div>

        <table>
            {rows}
        </table>

        <ButtonComponent<AddLevelMessage> message={AddLevelMessage::default()}/>
        <ButtonComponent<ProceedMessage> message={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> message={BackMessage::default()}/>

        </div>
    )
}

#[function_component(ClassControl)]
pub fn class_control() -> Html {
    html!(
        <div>
        <CarouselComponent<ChooseClassMessage> />

        <ButtonComponent<ProceedMessage> message={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> message={BackMessage::default()}/>

        </div>
    )
}

#[function_component(FeatureControl)]
pub fn feature_control() -> Html {
    html!(
        <div>
        <CarouselComponent<ChooseFeatureMessage> />

        <ButtonComponent<ProceedMessage> message={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> message={BackMessage::default()}/>

        </div>
    )
}

#[function_component(StatsControl)]
pub fn stats_control() -> Html{
    html!(
        <div>
        <CarouselComponent<ChooseFeatureMessage> />

        <ButtonComponent<ProceedMessage> message={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> message={BackMessage::default()}/>

        </div>
    )
}

use std::sync::Arc;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::*;

use crate::web::*;

#[function_component(App)]
pub fn app() -> Html {
    let no_data = *use_selector(|x: &CreationState| x.dictionary.classes.is_empty()).as_ref();

    if no_data {
        html! {
            <div class="site">
                <div class="container" >
                {"Loading"}
                </div>
            </div>
        }
    } else {
        html! {
            <div class="site">
                <div class="container" >
                <Content />
                </div>
            </div>
        }
    }
}

#[function_component(Content)]
pub fn content() -> Html {
    let stage = use_selector(|state: &CreationState| state.stage.clone());

    use crate::web::creation_state::Stage::*;
    match stage.as_ref() {
        CharacterSelect => html!(<CharacterSelectControl/>),
        Name => html!(<NameControl/>),
        Background => html!(<BackgroundControl />),
        Levels => html!(<LevelsControl />),
        Class { class: _ } => html!(<ClassControl />),
        ClassFeature { feature: _ } => html!(<FeatureControl />),
        Stats => html!(<StatsControl />),
        Backstory => html!(<BackstoryControl />),
        Finished => html!(< FinishedControl />),
    }
}

#[function_component(CharacterSelectControl)]
pub fn character_select_control() -> Html {
    let characters = use_selector(|x: &CreationState| x.saved_characters.clone());

    let rows = characters.iter().map(|x| {
        
        let arc = Arc::new(x.clone());
        html!(
            <tr>
            <td>{x.name.clone()} </td>
            <td>{x.background.0.clone()} </td>
            <td><ButtonComponent<LoadCharacterMessage> property={LoadCharacterMessage(arc.clone())}  /> </td>
            <td><ButtonComponent<DeleteCharacterMessage> property={DeleteCharacterMessage(arc.clone())}  /> </td>
            </tr>
        )
    })
    .collect::<Html>();

    html!(
        <div>
        <h2>{"Choose Character"}</h2>
        <table>
            {rows}
        </table>
            <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
        </div>
    )
}

#[function_component(NameControl)]
pub fn name_control() -> Html {
    html!(
        <div>
        <h2>{"Name"}</h2>
            <TextComponent<SetNameProperty> property={SetNameProperty::default()} />
            <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
            <ButtonComponent<BackMessage> property={BackMessage::default()}/>
        </div>
    )
}

#[function_component(BackstoryControl)]
pub fn backstory_control() -> Html {
    html!(
        <div>
        <h2>{"Backstory"}</h2>
            <TextComponent<SetBackstoryProperty> property={SetBackstoryProperty::default()} />
            <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
            <ButtonComponent<BackMessage> property={BackMessage::default()}/>
        </div>
    )
}

#[function_component(BackgroundControl)]
pub fn background_control() -> Html {
    html!(
        <div>
        <h2>{"Background"}</h2>
        <CarouselComponent<ChooseBackgroundMessage> property={ChooseBackgroundMessage::default()} />
        <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> property={BackMessage::default()}/>
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
                <td><ButtonComponent<AddLevelOfClassMessage> property={AddLevelOfClassMessage(x.class.clone())}  /> </td>
                <td>
                {if is_last {html!(<ButtonComponent<RemoveLastLevelMessage> property={RemoveLastLevelMessage::default()} />)} else{html!(<> </>)}}
                </td>
                </tr>
            )
        })
        .collect::<Html>();

    html!(
        <div>
        <h2>{"Levels"}</h2>
        <table>
            {rows}
        </table>

        <ButtonComponent<AddLevelMessage> property={AddLevelMessage::default()}/>
        <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> property={BackMessage::default()}/>

        </div>
    )
}

#[function_component(ClassControl)]
pub fn class_control() -> Html {
    html!(
        <div>
        <h2>{"Class"}</h2>
        <CarouselComponent<ChooseClassMessage > property={ChooseClassMessage::default()} />

        <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> property={BackMessage::default()}/>

        </div>
    )
}

#[function_component(FeatureControl)]
pub fn feature_control() -> Html {
    html!(
        <div>
        <h2>{"Feature"}</h2>
        <CarouselComponent<ChooseFeatureMessage> property={ChooseFeatureMessage::default()} />

        <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> property={BackMessage::default()}/>

        </div>
    )
}

#[function_component(StatsControl)]
pub fn stats_control() -> Html {
    use crate::data::Ability::*;

    let remaining_points = use_selector(|x: &CreationState| x.character.stats.points_remaining());

    html!(
        <div>
        <h2>{"Stats"}</h2>
        <form>
        <div class="grid">
            <NumberComponent <u8, StatProperty> property={StatProperty(Strength)} />
            <NumberComponent <u8, StatProperty> property={StatProperty(Dexterity)} />
            <NumberComponent <u8, StatProperty> property={StatProperty(Constitution)} />
            <NumberComponent <u8, StatProperty> property={StatProperty(Wisdom)} />
            <NumberComponent <u8, StatProperty> property={StatProperty(Intelligence)} />
            <NumberComponent <u8, StatProperty> property={StatProperty(Charisma)} />

            <label for="remaining"> {"Points Remaining"}
                <input type="number" name="remaining" readonly={true} value= {remaining_points.to_string()} />
          </label>

          </div>

        </form>

        <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> property={BackMessage::default()}/>

        </div>
    )
}

#[function_component(FinishedControl)]
pub fn finished_control() -> Html {
    let character = use_selector(|x: &CreationState| x.character.clone());

    //let v = serde_json::to_string(&character).unwrap_or_default();
    let character_html = character.to_html_string();
    let md = character.to_markdown_string();
    let encoded_md = base64::encode(&md);
    let file_name = character.name.as_ref().clone() + ".md";

    html!(
        <div>

        <SafeHtml html={character_html} />
        <a role="button"  style="width: 100%; margin-bottom: var(--spacing);" href={format!("data:application/octet-stream;charset=utf-8;base64,{}", encoded_md)}  download={file_name}>{"Download"}</a>

        <ButtonComponent<ProceedMessage> property={ProceedMessage::default()}/>
        <ButtonComponent<BackMessage> property={BackMessage::default()}/>
        </div>
    )
}

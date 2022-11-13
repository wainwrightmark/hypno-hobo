use yew::{AttrValue, Classes, Properties};

#[derive(Properties, PartialEq)]
pub struct BasicProps {
    #[prop_or_default()]
    pub classes: Classes,
    #[prop_or_default()]
    pub style: AttrValue,
}

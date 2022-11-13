use yew::{AttrValue, Classes, Properties};

use super::StoreProperty;

#[derive(Properties, PartialEq)]
pub struct BasicProps<T : StoreProperty> {

    pub property: T,

    #[prop_or_default()]
    pub classes: Classes,
    #[prop_or_default()]
    pub style: AttrValue,
}
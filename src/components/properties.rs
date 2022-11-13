use std::str::FromStr;

use yew::{Classes, Html};
use yewdux::store::{Reducer, Store};

#[derive(Clone, Debug)]
pub struct PropertyReducer<T: StoreProperty>(pub T);

pub trait StoreProperty: Clone + PartialEq {
    type State: Store;

    /// Try to apply the property change to the state
    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>>;

    /// Whether the button should be disabled
    fn is_disabled(&self, state: std::rc::Rc<Self::State>) -> bool {
        Self::try_apply(self, state).is_none()
    }
}


impl<T> Reducer<T::State> for PropertyReducer<T>
where
    T: StoreProperty,
{
    fn apply(self, state: std::rc::Rc<T::State>) -> std::rc::Rc<T::State> {
        if let Some(new_state) = T::try_apply(&self.0, state.clone()) {
            new_state
        } else {
            state
        }
    }
}

/// A property that can be sent by writing in a text field
pub trait TextProperty : StoreProperty + FromStr {
    /// Get the current value of the text
    fn get_current_value(state: &Self::State) -> String;

    fn placeholder()-> &'static str;
}

pub trait NumberProperty<T> : StoreProperty {
    fn get_current_value(state: &Self::State) -> T;
}

/// A property that can be changed by clicking a button
pub trait ButtonProperty: StoreProperty {
    /// The text of the button
    fn button_text(&self, state: std::rc::Rc<Self::State>) -> &'static str;
}

/// A property that can be changed by selecting something in a dropdown
pub trait SelectProperty: StoreProperty + PartialEq {
    /// Get all the values that can appear in the dropdown
    fn get_values(state: &Self::State) -> &[Self];
    /// Get the current value of the dropdown
    fn get_current_value(state: &Self::State) -> Self;

    /// Parse a string representation of this value
    fn parse_repr(s: &str) -> Self;

    /// Get a string representation of this value
    fn repr(&self) -> &'static str;

    /// Get the value that will appear in the dropdown
    fn text(&self) -> &'static str;
}

/// A property that can be controlled by scrolling a carousel
pub trait CarouselProperty: StoreProperty + PartialEq {
    /// Get all the values that can appear in the carousel
    fn get_values(state: &Self::State) -> Vec<Self>;
    /// Get the current value of the carousel
    fn get_current_value(state: &Self::State) -> Self;

    // /// action to perform when the carousel is clicked
    // fn as_button() -> Option<Box<dyn Reducer<Self::State>>>;

    /// Get the html for a carousel element. Injecting the classes
    fn get_html(&self, state: &Self::State, classes: Classes) -> Html;

    
}

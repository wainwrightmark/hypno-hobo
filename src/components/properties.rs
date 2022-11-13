use std::fmt::{Debug, Display};

use num::Num;
use yew::{Classes, Html};
use yewdux::store::{Reducer, Store};

#[derive(Clone, Debug)]
pub struct PropertyReducer<T: StoreProperty> {
    pub property: T,
    pub value: <T as StoreProperty>::Value,
}

impl<T> Reducer<T::State> for PropertyReducer<T>
where
    T: StoreProperty,
{
    fn apply(self, state: std::rc::Rc<T::State>) -> std::rc::Rc<T::State> {
        if let Some(new_state) = T::try_apply(&self.property, &self.value, state.clone()) {
            new_state
        } else {
            state
        }
    }
}

pub trait StoreProperty: Clone + PartialEq {
    type State: Store;
    type Value: Clone + Debug + PartialEq;

    /// Try to apply the property change to the state
    fn try_apply(
        &self,
        value: &Self::Value,
        state: std::rc::Rc<Self::State>,
    ) -> Option<std::rc::Rc<Self::State>>;

    /// Whether a particular value can be applied
    fn is_disabled(&self, value: &Self::Value, state: std::rc::Rc<Self::State>) -> bool {
        Self::try_apply(self, value, state).is_none()
    }

    fn get_current_value(&self, state: &Self::State) -> Self::Value;
}

/// A property that can be sent by writing in a text field
pub trait TextProperty: StoreProperty<Value = String> {
    fn placeholder(&self) -> &'static str;

    //TODO is valid
    //TODO is disabled
    //TODO debounce interval
}

pub trait NumberProperty<T: Num + Display>: StoreProperty<Value = T> {
    fn min(&self) -> T;
    fn max(&self) -> T;
    fn step(&self) -> T;

    fn label(&self) -> String;
    //TODO min, max, step
}

/// A property that can be changed by clicking a button
pub trait ButtonProperty: StoreProperty<Value = ()> {
    /// The text of the button
    fn button_text(&self, state: std::rc::Rc<Self::State>) -> &'static str;
}

/// A property that can be changed by selecting something in a dropdown
pub trait SelectProperty: StoreProperty {
    /// Get all the values that can appear in the dropdown
    fn get_values(state: &Self::State) -> &[Self::Value];

    /// Parse a string representation of this value
    fn parse_repr(s: &str) -> Self::Value;

    /// Get a string representation of this value
    fn repr(value: &Self::Value) -> &'static str;

    /// Get the value that will appear in the dropdown
    fn text(value: &Self::Value) -> &'static str;
}

/// A property that can be controlled by scrolling a carousel
pub trait CarouselProperty: StoreProperty + PartialEq {
    /// Get all the values that can appear in the carousel
    fn get_values(&self, state: &Self::State) -> Vec<Self::Value>;

    // /// action to perform when the carousel is clicked
    // fn as_button() -> Option<Box<dyn Reducer<Self::State>>>;

    /// Get the html for a carousel element. Injecting the classes
    fn get_html(&self, value: &Self::Value, state: &Self::State, classes: Classes) -> Html;
}

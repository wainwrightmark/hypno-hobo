use yew::{Classes, Html};
use yewdux::store::{Reducer, Store};

#[derive(Clone, Debug)]
pub struct MessageReducer<T: StoreMessage>(pub T);

pub trait StoreMessage: Clone + PartialEq {
    type State: Store;

    /// Try to apply the button message to the state
    fn try_apply(&self, state: std::rc::Rc<Self::State>) -> Option<std::rc::Rc<Self::State>>;

    /// Whether the button should be disabled
    fn is_disabled(&self, state: std::rc::Rc<Self::State>) -> bool {
        Self::try_apply(self, state).is_none()
    }
}

/// A message that can be sent by clicking a button
pub trait ButtonMessage: StoreMessage {
    /// The text of the button
    fn button_text(&self, state: std::rc::Rc<Self::State>) -> &'static str;
}

impl<T> Reducer<T::State> for MessageReducer<T>
where
    T: StoreMessage,
{
    fn apply(self, state: std::rc::Rc<T::State>) -> std::rc::Rc<T::State> {
        if let Some(new_state) = T::try_apply(&self.0, state.clone()) {
            new_state
        } else {
            state
        }
    }
}

/// A message that can be sent by selecting something in a dropdown
pub trait SelectMessage: StoreMessage + PartialEq {
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

/// A message that can be sent by scrolling a carousel
pub trait CarouselMessage: StoreMessage + PartialEq {
    /// Get all the values that can appear in the carousel
    fn get_values(state: &Self::State) -> Vec<Self>;
    /// Get the current value of the carousel
    fn get_current_value(state: &Self::State) -> Self;

    /// Get the html for a carousel element. Injecting the classes
    fn get_html(&self, state: &Self::State, classes: Classes) -> Html;
}

use std::rc::Rc;

use yewdux::store::Reducer;

use crate::components::messages::ButtonMessage;

use super::creation_state::CreationState;




#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ProceedMessage {}

impl ButtonMessage<CreationState> for ProceedMessage {
    fn can_apply(state: &CreationState) -> bool {
        match state {
            _=> panic!()
        }
    }

    fn get_name() -> &'static str {
        "Proceed"
    }
}

impl Reducer<CreationState> for ProceedMessage {
    fn apply(self, state: std::rc::Rc<CreationState>) -> std::rc::Rc<CreationState> {
        match state.as_ref() {
            _=> panic!()
        }
    }
}
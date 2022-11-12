pub mod app;
pub mod creation_state;
pub mod data_state;
pub mod saved_characters_state;
pub mod proceed_message;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::creation_state::*;
    pub use crate::web::data_state::*;
    pub use crate::web::saved_characters_state::*;
}

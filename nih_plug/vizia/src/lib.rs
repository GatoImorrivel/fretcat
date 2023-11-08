//! # Vizia

pub use vizia_baseview::{Application, ParentWindow, WindowScalePolicy};

pub use vizia_core::*;

#[doc(hidden)]
pub mod prelude {
    pub use vizia_core::prelude::*;

    pub use vizia_baseview::{Application, ApplicationEvent};
}

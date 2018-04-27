//! The layout module provides everything necessary for working with Layouts. A
//! Layout allows you to combine multiple components together to visualize a
//! variety of information the runner is interested in.

mod component;
mod component_settings;
mod component_state;
pub mod editor;
mod general_settings;
mod layout;
mod layout_settings;
mod layout_state;

pub use self::component::Component;
pub use self::component_settings::ComponentSettings;
pub use self::component_state::ComponentState;
pub use self::editor::Editor;
pub use self::general_settings::GeneralSettings;
pub use self::layout::Layout;
pub use self::layout_settings::LayoutSettings;
pub use self::layout_state::LayoutState;

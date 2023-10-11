use crate::prelude::*;
use vizia_window::Position;

/// Modifiers for setting the properties of a window.
pub trait WindowModifiers {
    /// Sets the title of the window to the given value. Accepts a type, or lens to a type, which implements `ToString`.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .title("Vizia Application")
    /// .run();
    /// ```
    fn title<T: ToString>(self, title: T) -> Self;
    /// Sets the inner size of the window to the given value. Accepts a value, or lens, which can be converted to a [`WindowSize`].
    ///
    /// The inner size is the window area excluding the window borders.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .inner_size((300, 300))
    /// .run();
    /// ```
    fn inner_size<S: Into<WindowSize>>(self, size: S) -> Self;
    /// Sets the minimum inner size of the window to the given value. Accepts an optional value, or lens, which can be converted to a [`WindowSize`].
    ///
    /// Setting the minimum inner size to `None` removes the minimum inner size constraint from the window.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .min_inner_size(Some((300, 300)))
    /// .run();
    /// ```
    fn min_inner_size<S: Into<WindowSize>>(self, size: Option<S>) -> Self;
    /// Sets the maximum inner size of the window to the given value. Accepts an optional value, or lens, which can be converted to a [`WindowSize`].
    ///
    /// Setting the maximum inner size to `None` removes the maximum inner size constraint from the window.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .max_inner_size(Some((1000, 1000)))
    /// .run();
    /// ```
    fn max_inner_size<S: Into<WindowSize>>(self, size: Option<S>) -> Self;
    /// Sets the position of the window to the given value. Accepts a value, or lens, which can be converted to a [`Position`].
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .position((100, 200))
    /// .run();
    /// ```
    fn position<P: Into<Position>>(self, position: P) -> Self;
    /// Sets whether the window can be resized. Accepts a boolean value, or lens to a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .resizable(false)
    /// .run();
    /// ```
    fn resizable(self, flag: bool) -> Self;
    /// Sets whether the window is minimized. Accepts a boolean value, or lens to a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .minimized(true)
    /// .run();
    /// ```
    fn minimized(self, flag: bool) -> Self;
    /// Sets whether the window is maximized. Accepts a boolean value, or lens to a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .maximized(true)
    /// .run();
    /// ```
    fn maximized(self, flag: bool) -> Self;
    /// Sets whether the window is visible. Accepts a boolean value, or lens to a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .visible(false)
    /// .run();
    /// ```
    fn visible(self, flag: bool) -> Self;
    /// Sets whether the window is transparent. Accepts a boolean value, or lens to a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .transparent(true)
    /// .run();
    /// ```
    fn transparent(self, flag: bool) -> Self;
    /// Sets whether the window has decorations. Accepts a boolean value, or lens to a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .decorations(false)
    /// .run();
    /// ```
    fn decorations(self, flag: bool) -> Self;
    /// Sets whether the window should be on top of other windows. Accepts a boolean value, or lens to a boolean value.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .always_on_top(true)
    /// .run();
    /// ```
    fn always_on_top(self, flag: bool) -> Self;
    /// Sets whether the window has vsync enabled.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// .vsync(true)
    /// .run();
    /// ```
    fn vsync(self, flag: bool) -> Self;
    /// Sets the icon used for the window.
    ///
    /// # Example
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// Application::new(|cx|{
    ///     // Content here
    /// })
    /// // .icon() TODO
    /// .run();
    /// ```
    fn icon(self, image: Vec<u8>, width: u32, height: u32) -> Self;
    #[cfg(target_arch = "wasm32")]
    fn canvas(self, canvas: &str) -> Self;
}

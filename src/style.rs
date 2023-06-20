use nih_plug_iced::{container, Color};

pub struct Container;
pub struct ContainerWithPadding;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::WHITE.into()),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}
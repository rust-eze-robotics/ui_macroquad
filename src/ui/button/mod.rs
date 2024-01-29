pub mod factory;
pub mod square;

#[derive(Debug, PartialEq)]
pub enum ButtonState {
    Active,
    Down,
    Disabled,
    Hovered,
}

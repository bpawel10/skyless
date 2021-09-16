use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u16)]
#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
pub enum Items {
    Grass = 106,
    StoneSwitch = 431,
    StoneSwitchActivated = 430,
    LeverLeft = 2772,
    LeverRight = 2773,
}

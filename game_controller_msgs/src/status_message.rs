use anyhow::{bail, Error};
use bytes::{Buf, Bytes};

use crate::bindings::{
    GAMECONTROLLER_RETURN_STRUCT_HEADER, GAMECONTROLLER_RETURN_STRUCT_SIZE,
    GAMECONTROLLER_RETURN_STRUCT_VERSION, MAX_NUM_PLAYERS,
};

/// This struct corresponds to `RoboCupGameControlReturnData`.
/// `RoboCupGameControlReturnData::header` and `RoboCupGameControlReturnData::version` are
/// implicitly added/removed when converting to/from the binary format.
pub struct StatusMessage {
    /// This field corresponds to `RoboCupGameControlReturnData::playerNum`.
    pub player_number: u8,
    /// This field corresponds to `RoboCupGameControlReturnData::teamNum`.
    pub team_number: u8,
    /// This field corresponds to `RoboCupGameControlReturnData::fallen`.
    pub fallen: bool,
    /// This field corresponds to `RoboCupGameControlReturnData::pose`.
    pub pose: [f32; 3],
    /// This field corresponds to `RoboCupGameControlReturnData::ballAge`.
    pub ball_age: f32,
    /// This field corresponds to `RoboCupGameControlReturnData::ball`.
    pub ball: [f32; 2],
}

impl TryFrom<Bytes> for StatusMessage {
    type Error = Error;

    fn try_from(mut bytes: Bytes) -> Result<Self, Self::Error> {
        if bytes.len() != GAMECONTROLLER_RETURN_STRUCT_SIZE {
            bail!("wrong length");
        }
        let header = bytes.copy_to_bytes(4);
        if header != GAMECONTROLLER_RETURN_STRUCT_HEADER[..4] {
            bail!("wrong header");
        }
        let version = bytes.get_u8();
        if version != GAMECONTROLLER_RETURN_STRUCT_VERSION {
            bail!("wrong version");
        }
        let player_number = bytes.get_u8();
        if !(1..=MAX_NUM_PLAYERS).contains(&player_number) {
            bail!("invalid player number");
        }
        let team_number = bytes.get_u8();
        let fallen = bytes.get_u8();
        if fallen > 1 {
            bail!("invalid fallen");
        }
        let pose = [bytes.get_f32_le(), bytes.get_f32_le(), bytes.get_f32_le()];
        if pose.iter().any(|component| component.is_nan()) {
            bail!("invalid pose");
        }
        let ball_age = bytes.get_f32_le();
        if ball_age.is_nan() {
            bail!("invalid ball age");
        }
        let ball = [bytes.get_f32_le(), bytes.get_f32_le()];
        if ball.iter().any(|component| component.is_nan()) {
            bail!("invalid ball");
        }
        assert!(!bytes.has_remaining());
        Ok(StatusMessage {
            player_number,
            team_number,
            fallen: fallen == 1,
            pose,
            ball_age,
            ball,
        })
    }
}

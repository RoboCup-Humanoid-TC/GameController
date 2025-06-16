use anyhow::{bail, Error};
use bytes::{Buf, Bytes};

use crate::bindings::{
    GAMECONTROLLER_RETURN_STRUCT_HEADER, GAMECONTROLLER_RETURN_STRUCT_SIZE,
    GAMECONTROLLER_RETURN_STRUCT_VRC_GESTURE_MAX, GAMECONTROLLER_RETURN_STRUCT_VRC_GESTURE_MIN,
    GAMECONTROLLER_RETURN_STRUCT_VRC_VERSION, MAX_NUM_PLAYERS,
};

/// This struct corresponds to `RoboCupGameControlReturnData`, modified for the visual referee
/// challenge result reports. `RoboCupGameControlReturnData::header`,
/// `RoboCupGameControlReturnData::version`, `RoboCupGameControlReturnData::pose`, and
/// `RoboCupGameControlReturnData::ball` are implicitly added/removed when converting to/from the
/// binary format.
pub struct VrcMessage {
    /// This field corresponds to `RoboCupGameControlReturnData::playerNum`.
    pub player_number: u8,
    /// This field corresponds to `RoboCupGameControlReturnData::teamNum`.
    pub team_number: u8,
    /// This field corresponds to `RoboCupGameControlReturnData::fallen`.
    pub gesture: u8,
    /// This field corresponds to `RoboCupGameControlReturnData::ballAge`.
    pub whistle_age: f32,
}

impl TryFrom<Bytes> for VrcMessage {
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
        if version != GAMECONTROLLER_RETURN_STRUCT_VRC_VERSION {
            bail!("wrong version");
        }
        let player_number = bytes.get_u8();
        if !(1..=MAX_NUM_PLAYERS).contains(&player_number) {
            bail!("invalid player number");
        }
        let team_number = bytes.get_u8();
        let gesture = bytes.get_u8();
        if !(GAMECONTROLLER_RETURN_STRUCT_VRC_GESTURE_MIN
            ..=GAMECONTROLLER_RETURN_STRUCT_VRC_GESTURE_MAX)
            .contains(&gesture)
        {
            bail!("invalid gesture");
        }
        // ignore pose
        let _ = [bytes.get_f32_le(), bytes.get_f32_le(), bytes.get_f32_le()];
        let whistle_age = bytes.get_f32_le();
        if whistle_age.is_nan() {
            bail!("invalid whistle age");
        }
        // ignore ball
        let _ = [bytes.get_f32_le(), bytes.get_f32_le()];
        assert!(!bytes.has_remaining());
        Ok(VrcMessage {
            player_number,
            team_number,
            gesture,
            whistle_age,
        })
    }
}

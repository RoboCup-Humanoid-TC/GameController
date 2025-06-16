use anyhow::{bail, Error};
use bytes::{Buf, Bytes};

use crate::bindings::{
    GAMECONTROLLER_RETURN_STRUCT_HEADER,
    MAX_NUM_PLAYERS,
};

/// This struct corresponds to `RoboCupGameControlReturnData`.
/// `RoboCupGameControlReturnData::header` and `RoboCupGameControlReturnData::version` are
/// implicitly added/removed when converting to/from the binary format.
pub struct HlStatusMessage {
    /// version of the protocoll
    pub version: u8,
    /// This field corresponds to `RoboCupGameControlReturnData::teamNum`.
    pub team_number: u8,
    /// This field corresponds to `RoboCupGameControlReturnData::playerNum`.
    pub player_number: u8,
    /// message to the robot
    pub message: u8,
}

impl TryFrom<Bytes> for HlStatusMessage {
    type Error = Error;

    fn try_from(mut bytes: Bytes) -> Result<Self, Self::Error> {
        if bytes.len() != 8 {
            bail!("wrong length");
        }
        let header = bytes.copy_to_bytes(4);
        if header != GAMECONTROLLER_RETURN_STRUCT_HEADER[..4] {
            bail!("wrong header");
        }
        let version = bytes.get_u8();
        if version != 2 {
            bail!("wrong version");
        }
        let team_number = bytes.get_u8();
        let player_number = bytes.get_u8();
        if !(1..=MAX_NUM_PLAYERS).contains(&player_number) {
            bail!("invalid player number");
        }
        let message = bytes.get_u8();
        assert!(!bytes.has_remaining());
        Ok(HlStatusMessage {
            version,
            team_number,
            player_number,
            message,
        })
    }
}

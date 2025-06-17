use anyhow::{bail, Error};
use bytes::{Buf, Bytes};

#[derive(Debug)]
pub struct RefereeMessage {
    /// primary command
    pub command_1: u8,
    /// secondary command
    pub command_2: u8,
    /// helper command
    pub command_3: u8,
    /// helper command
    pub command_4: u8,
    /// helper command
    pub command_5: u8,
}

impl TryFrom<Bytes> for RefereeMessage {
    type Error = Error;

    fn try_from(mut bytes: Bytes) -> Result<Self, Self::Error> {
        let header = bytes.copy_to_bytes(4);
        if header != "RGrf" {
            bail!("wrong header!")
        }
        let version = bytes.get_u8();
        if version != 0 {
            bail!("wrong version!")
        }
        let command_1 = bytes.get_u8();
        let command_2 = bytes.get_u8();
        let command_3 = bytes.get_u8();
        let command_4 = bytes.get_u8();
        let command_5 = bytes.get_u8();

        Ok(RefereeMessage {
            command_1,
            command_2,
            command_3,
            command_4,
            command_5,
        })
    }
}

use anyhow::{bail, Error};
use bytes::{Buf, Bytes};

/// This struct represents a request from a monitor application for "true" data.
pub struct MonitorRequest();

impl TryFrom<Bytes> for MonitorRequest {
    type Error = Error;

    fn try_from(mut bytes: Bytes) -> Result<Self, Self::Error> {
        if bytes.len() != 5 {
            bail!("wrong length");
        }
        let header = bytes.copy_to_bytes(4);
        if header != b"RGTr"[..4] {
            bail!("wrong header");
        }
        let version = bytes.get_u8();
        if version != 0 {
            bail!("wrong version");
        }
        assert!(!bytes.has_remaining());
        Ok(MonitorRequest())
    }
}

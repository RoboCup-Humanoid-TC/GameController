use std::{io::Result, net::SocketAddr};
use tokio::net::UdpSocket;

/// This function wraps [tokio::net::UdpSocket::recv_from]. It handles the fact that Windows throws
/// an error (`WSAEMSGSIZE`) when a received datagram is larger than the user-supplied buffer.
/// Unfortunately, it is not possible to get the sender address in that case. Instead, the local
/// socket address is returned, so the origin of overlong packets is not reliable on Windows (it is
/// not reliable in general because of the nature of UDP, but that is another topic).
pub async fn recv_from(socket: &UdpSocket, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
    match socket.recv_from(buf).await {
        #[cfg(target_os = "windows")]
        Err(error) if error.raw_os_error() == Some(10040) => Ok((buf.len(), socket.local_addr()?)),
        result => result,
    }
}

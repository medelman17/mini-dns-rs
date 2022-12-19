use std::fmt;

#[derive(Debug)]
pub enum PacketBufferError {
    EndOfBuffer,
    JumpLimitExceeded,
    LabelTooBig,
}

impl std::error::Error for PacketBufferError {}

impl fmt::Display for PacketBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketBufferError::EndOfBuffer => write!(f, "End of Buffer Error"),
            PacketBufferError::JumpLimitExceeded => write!(f, "Jump Limit Exceeded"),
            PacketBufferError::LabelTooBig => write!(f, "Single label exceeds 63 chars in lenght"),
        }
    }
}

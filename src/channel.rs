use crate::{ENetIncomingCommand, ENetList};

#[derive(Default)]
pub(crate) struct Channel {
    pub(crate) outgoing_reliable_sequence_number: u16,
    pub(crate) outgoing_unreliable_sequence_number: u16,
    pub(crate) used_reliable_windows: u16,
    pub(crate) reliable_windows: [u16; 16],
    pub(crate) incoming_reliable_sequence_number: u16,
    pub(crate) incoming_unreliable_sequence_number: u16,
    pub(crate) incoming_reliable_commands: ENetList<ENetIncomingCommand>,
    pub(crate) incoming_unreliable_commands: ENetList<ENetIncomingCommand>,
}

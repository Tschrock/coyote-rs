use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    /// Heartbeat packet data
    Heartbeat,
    /// ID relationship binding
    Bind,
    // Waveform sending/intensity change/queue clearing and other data instructions
    #[serde(rename = "msg")]
    Message,
    /// Disconnect
    Break,
    /// Service error
    Error,
}

pub struct WebsocketMessage {
    pub message_type: MessageType,
    pub client_id: String,
    pub target_id: String,
    pub message: String,
}

pub enum ErrorCode {
    /// Success
    Success = 200,
    /// The other client has disconnected
    ClientDisconnected = 209,
    /// There is no valid clientID in the QR code
    InvalidClientId = 210,
    /// The socket is connected, but the server is slow to send the app ID for binding
    BindingTimeout = 211,
    /// This id has been bound by another client
    ClientIdAlreadyBound = 400,
    /// The target client to bind to does not exist
    TargetClientNotFound = 401,
    /// The recipient and sender are not bound to each other
    ClientsNotBound = 402,
    /// The content sent is not a standard JSON object
    InvalidJson = 403,
    /// Recipient Not Found (Offline)
    RecipientNotFound = 404,
    /// The length of the message sent is greater than 1950
    MessageTooLong = 405,
    /// Internal server error
    InternalServerError = 500,
}

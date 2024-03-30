enum Accept {
    Ok = 0,
    Failure = 1,
    InternalError = 2,
    NotSupported = 3,
    PermanentResourceLimitation = 4,
    TemporaryResourceLimitation = 5,
}

struct ServerStart {
    mbz_start: [u8; 15],
    accept: Accept,
    server_iv: [u8; 16],
    start_time: [u8; 64],
    mbz_end: [u8; 8],
}

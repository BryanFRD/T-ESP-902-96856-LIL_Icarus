/// Stream ID type alias for clarity
pub type StreamId = u32;

/// Control stream ID (reserved for protocol control messages)
pub const CONTROL_STREAM_ID: StreamId = 0;

/// Minimum stream ID for application data
pub const MIN_DATA_STREAM_ID: StreamId = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_stream_id_is_zero() {
        assert_eq!(CONTROL_STREAM_ID, 0);
    }

    #[test]
    fn min_data_stream_id_is_one() {
        assert_eq!(MIN_DATA_STREAM_ID, 1);
    }

    #[test]
    fn stream_id_is_u32() {
        let id: StreamId = 42;
        assert_eq!(id, 42_u32);
    }
}

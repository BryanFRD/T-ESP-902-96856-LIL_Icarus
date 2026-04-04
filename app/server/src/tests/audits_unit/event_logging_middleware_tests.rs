use crate::middlewares::event_logging::EventLoggingConfig;

#[test]
fn test_event_logging_config_default_track_reads() {
    let config = EventLoggingConfig::default();
    assert!(config.track_reads);
}

#[test]
fn test_event_logging_config_default_excluded_paths() {
    let config = EventLoggingConfig::default();
    assert!(config.excluded_paths.contains(&"/v1/status".to_string()));
    assert!(config.excluded_paths.contains(&"/v1/swagger".to_string()));
    assert!(config.excluded_paths.contains(&"/v1/events".to_string()));
    assert_eq!(config.excluded_paths.len(), 3);
}

#[test]
fn test_event_logging_config_clone() {
    let config = EventLoggingConfig::default();
    let cloned = config.clone();
    assert_eq!(cloned.track_reads, config.track_reads);
    assert_eq!(cloned.excluded_paths, config.excluded_paths);
}

//! Record information about errors in traces and metrics.

use query_engine_metrics::metrics;

/// Record an execution error in the current trace, and increment a counter.
pub fn execution_error(error: &query_engine_execution::error::Error, metrics: &metrics::Metrics) {
    use query_engine_execution::error::*;
    // TODO(PY): fix the correct error category
    match error {
        Error::Query(_)
        | Error::Mutation(_)
        | Error::ConnectionPool(_)
        | Error::TiberiusError(_) => {
            metrics.error_metrics.record_invalid_request();
        }
    }
}

/// Record a translation error in the current trace, and increment a counter.
pub fn translation_error(
    error: &query_engine_translation::translation::error::Error,
    metrics: &metrics::Metrics,
) {
    use query_engine_translation::translation::error::*;
    tracing::error!("{}", error);
    match error {
        Error::CapabilityNotSupported(_) => {
            metrics.error_metrics.record_unsupported_capability();
        }
        Error::NotImplementedYet(_) => {
            metrics.error_metrics.record_unsupported_feature();
        }
        _ => {
            metrics.error_metrics.record_invalid_request();
        }
    }
}

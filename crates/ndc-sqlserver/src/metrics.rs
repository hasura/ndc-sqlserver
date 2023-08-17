//! Metrics setup and update for our connector.

use super::configuration::InitializationError;
use ndc_hub::connector;
use prometheus::core::{AtomicU64, GenericCounter};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub query_total: GenericCounter<AtomicU64>,
    pub explain_total: GenericCounter<AtomicU64>,
}

/// Create a new int counter metric and register it with the provided Prometheus Registry
fn add_int_counter_metric(
    metrics_registry: &mut prometheus::Registry,
    metric_name: &str,
    metric_description: &str,
) -> Result<GenericCounter<AtomicU64>, connector::InitializationError> {
    let int_counter =
        prometheus::IntCounter::with_opts(prometheus::Opts::new(metric_name, metric_description))
            .map_err(|prometheus_error| {
            connector::InitializationError::Other(
                InitializationError::PrometheusError(prometheus_error).into(),
            )
        })?;

    metrics_registry
        .register(Box::new(int_counter.clone()))
        .map_err(|prometheus_error| {
            connector::InitializationError::Other(
                InitializationError::PrometheusError(prometheus_error).into(),
            )
        })?;

    Ok(int_counter)
}

/// Setup counters and gauges used to produce Prometheus metrics
pub async fn initialise_metrics(
    metrics_registry: &mut prometheus::Registry,
) -> Result<Metrics, connector::InitializationError> {
    let query_total = add_int_counter_metric(
        metrics_registry,
        "sqlserver_ndc_query_total",
        "Total successful queries.",
    )?;

    let explain_total = add_int_counter_metric(
        metrics_registry,
        "sqlserver_ndc_explain_total",
        "Total successful explains.",
    )?;

    Ok(Metrics {
        query_total,
        explain_total,
    })
}

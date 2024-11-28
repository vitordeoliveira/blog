use anyhow::{Ok, Result};
use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, Tracer};
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::resource::{
    DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION,
};
use opentelemetry_semantic_conventions::SCHEMA_URL;
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::{EnvFilter, Layer};

pub struct Tracing;

fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT, env!("RUST_ENV")),
        ],
        SCHEMA_URL,
    )
}

fn init_tracer_provider(tracer_url: &str) -> Result<Tracer> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(tracer_url),
        )
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                // Customize sampling strategy
                .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                    1.0,
                ))))
                // If export trace to AWS X-Ray, you can use XrayIdGenerator
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(resource()),
        )
        .install_batch(runtime::Tokio)?
        .tracer("tracing-otel-subscriber");

    Ok(tracer)
}

// To help impl https://github.com/tokio-rs/tracing-opentelemetry/blob/v0.1.x/examples/opentelemetry-otlp.rs
// fn init_metrics_provider() -> Result<SdkMeterProvider> {
//     todo!()
// }

impl Tracing {
    pub fn setup(tracer_url: &str, rust_log: &str) -> Result<()> {
        let tracer = init_tracer_provider(tracer_url)?;
        let env_filter = format!("{rust_log},h2=off,tower::buffer::worker=off");

        let console_layer = tracing_subscriber::fmt::layer()
            .pretty()
            .with_filter(EnvFilter::builder().parse(env_filter).unwrap());

        tracing_subscriber::registry()
            .with(console_layer)
            .with(OpenTelemetryLayer::new(tracer).with_filter(LevelFilter::INFO))
            .init();

        Ok(())
    }
}

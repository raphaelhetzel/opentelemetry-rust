use futures_core::future::BoxFuture;
use opentelemetry_sdk::export::trace::{ExportResult, SpanData, SpanExporter};

use super::OtlpHttpClient;

impl SpanExporter for OtlpHttpClient {
    fn export(&mut self, batch: Vec<SpanData>) -> BoxFuture<'static, ExportResult> {
        self.export_inner(batch, &self.resource)
    }

    fn shutdown(&mut self) {
        let _ = self.client.lock().map(|mut c| c.take());
    }

    fn set_resource(&mut self, resource: &opentelemetry_sdk::Resource) {
        self.resource = resource.into();
    }
}

impl opentelemetry_sdk::export::trace::MultiServiceSpanExporter for OtlpHttpClient {
    fn export_with_resource(
        &mut self,
        batch: Vec<SpanData>,
        res: &opentelemetry_sdk::resource::Resource,
    ) -> BoxFuture<'static, ExportResult> {
        self.export_inner(batch, &res.into())
    }
}

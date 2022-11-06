// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{future::Future, net::SocketAddr};

use tonic::transport::Server;

use super::LogInterceptor;

pub async fn serve(
    addr: SocketAddr,
    logger: &slog::Logger,
    shutdown: impl Future<Output = ()>,
) -> Result<(), tonic::transport::Error> {
    // Setup initial health reporter/service.
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_service_status("", tonic_health::ServingStatus::Serving)
        .await;

    // Setup reflection server.
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(
            tonic_health::proto::GRPC_HEALTH_V1_FILE_DESCRIPTOR_SET,
        )
        .build()
        .unwrap();

    // Setup log interceptor.
    let _interceptor = LogInterceptor::new(logger);

    info!(logger, "Riftd has started and is listening for gRPC requests."; "addr" => addr.to_string());

    // Actually listen and serve the configured services.
    Server::builder()
        .add_service(reflection)
        .add_service(health_service)
        .serve_with_shutdown(addr, shutdown)
        .await
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    use crate::logging;

    #[tokio::test]
    async fn test_server() {
        let logger = logging::noop();

        let srv = serve("0.0.0.0:9090".parse().unwrap(), &logger, async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
        });
        srv.await.unwrap()
    }
}

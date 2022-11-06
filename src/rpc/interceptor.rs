// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use tonic::service::Interceptor;
use tonic::{Request, Status};

const REQ_ID_META: &str = "x-request-id";
const REQ_ID_NAME: &str = "request_id";

/// The LoggerExt handles injecting a request specific logger into the gRPC execution
/// chain.
pub struct LogExt {
    /// The logger to use throughout this requests life cycle.
    pub logger: slog::Logger,
}

/// The interceptor wrapper to have all gRPC requests pass through.
#[derive(Debug, Clone)]
pub struct LogInterceptor {
    logger: slog::Logger,
}

impl LogInterceptor {
    /// Create a new RaftInterceptor based on the supplied arguments.
    pub fn new(logger: &slog::Logger) -> LogInterceptor {
        LogInterceptor {
            logger: logger.new(o!("name" => "interceptor")),
        }
    }
}

impl Interceptor for LogInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let req_id = if let Some(req_id) = req.metadata().get(REQ_ID_META) {
            req_id.to_str().unwrap().to_string()
        } else {
            uuid::Uuid::new_v4().to_string()
        };

        req.extensions_mut().insert(LogExt {
            logger: self.logger.new(o!(REQ_ID_NAME => req_id)),
        });

        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use tonic::metadata::MetadataValue;

    use crate::logging;

    use super::*;

    #[test]
    fn test_interceptor() {
        let logger = logging::noop();
        let original = LogInterceptor::new(&logger);
        let mut interceptor = original.clone();

        assert_eq!(format!("{:?}", interceptor), format!("{:?}", original));

        let req = Request::new(());
        let res = interceptor.call(req);
        assert!(res.is_ok());

        let res = res.unwrap();
        let ext = res.extensions().get::<LogExt>();
        assert!(ext.is_some());

        let mut req = Request::new(());
        req.metadata_mut()
            .insert(REQ_ID_META, MetadataValue::from_static("hello"));

        let res = interceptor.call(req);
        assert!(res.is_ok());

        let res = res.unwrap();
        let ext = res.extensions().get::<LogExt>();
        assert!(ext.is_some());
    }
}

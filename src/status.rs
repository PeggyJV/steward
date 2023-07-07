use lazy_static::lazy_static;
use tonic::{async_trait, Request, Response, Status};

use crate::proto::{self, VersionRequest, VersionResponse};

lazy_static! {
    static ref STEWARD_VERSION: &'static str = env!("CARGO_PKG_VERSION");
}

pub struct StatusHandler;

#[async_trait]
impl proto::status_service_server::StatusService for StatusHandler {
    async fn version(
        &self,
        _: Request<VersionRequest>,
    ) -> Result<Response<VersionResponse>, Status> {
        Ok(Response::new(VersionResponse {
            version: STEWARD_VERSION.to_string(),
        }))
    }
}

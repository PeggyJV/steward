use lazy_static::lazy_static;
use steward_proto::steward::{self, VersionRequest, VersionResponse};
use tonic::{async_trait, Request, Response, Status};

lazy_static! {
    static ref STEWARD_VERSION: &'static str = env!("CARGO_PKG_VERSION");
}

pub struct StatusHandler;

#[async_trait]
impl steward::status_server::Status for StatusHandler {
    async fn version(
        &self,
        _: Request<VersionRequest>,
    ) -> Result<Response<VersionResponse>, Status> {
        Ok(Response::new(VersionResponse {
            version: STEWARD_VERSION.to_string(),
        }))
    }
}

use hyper::body::Incoming as IncomingBody;
use hyper::service::Service;
use hyper::Request;
use std::future::Future;
use std::pin::Pin;
use hyper::StatusCode;
use hyper::http::Response;
use http_body_util::{BodyExt, Full};
use bytes;
use hyper::header::{HeaderValue, CONTENT_TYPE};

use crate::config::Config;

/*
    BoxedResponse is a type.
    It should work with hyper responses across
    different libraries and dependencies.
*/
// use response::{build_response, BoxedResponse, ResponseParams};

use crate::type_flyweight::{BoxedResponse};

#[derive(Clone, Debug)]
pub struct Svc {
    // response_params: ResponseParams,
}

impl Svc {
    pub fn from(config: Config) -> Svc {
        Svc {
            // response_params: ResponseParams::from(
            //     config.directory,
            //     config.filepath_404,
            //     config.content_encodings,
            // ),
        }
    }
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = BoxedResponse;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        // let response_params = self.response_params.clone();

        Box::pin(async move { build_response(StatusCode::OK, "hello world!") })
    }
}

pub fn build_response(
    status_code: StatusCode,
    body: &'static str,
) -> Result<BoxedResponse, hyper::http::Error> {
    Response::builder()
        .status(status_code)
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8"))
        .body(
            Full::new(bytes::Bytes::from(body))
                .map_err(|e| match e {})
                .boxed(),
        )
}
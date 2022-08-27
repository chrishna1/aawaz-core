use actix_web::HttpResponse;

use self::error::ApiError;

pub mod error;

pub type EndpointResult = Result<HttpResponse, ApiError>;

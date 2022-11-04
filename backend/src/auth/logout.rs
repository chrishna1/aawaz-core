use actix_session::Session;
use actix_web::HttpResponse;

use crate::util::EndpointResult;

pub async fn logout(session: Session) -> EndpointResult {
    session.purge();
    return Ok(HttpResponse::NoContent().finish());
}

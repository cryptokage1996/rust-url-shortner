use rocket::{Request, Response, http::Status, response::Responder};
use std::io::Cursor;

#[derive(Debug)]
pub enum RedirectError {
    NotFound,
    Internal,
}

impl<'r> Responder<'r, 'static> for RedirectError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            RedirectError::NotFound => Response::build()
                .status(Status::NotFound)
                .sized_body(
                    "Short code not found".len(),
                    Cursor::new("Short code not found"),
                )
                .ok(),
            RedirectError::Internal => Response::build()
                .status(Status::InternalServerError)
                .sized_body("Internal error".len(), Cursor::new("Internal error"))
                .ok(),
        }
    }
}

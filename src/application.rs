use header::Headers;
use status::StatusCode;
use method::Method;

pub trait Application: Send + Sync + 'static {
    fn on_request<R: Responder>(&self, request: Request, responder: R);
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: Vec<u8>,
}

pub trait Responder {
    type R: BodyResponder;

    fn start(self, status_code: StatusCode, headers: Headers) -> Self::R;
}

pub trait BodyResponder {
    fn send(&mut self, data: Vec<u8>);
    fn finish(self);
}

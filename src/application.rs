pub trait Application: Send + Sync + 'static {
    fn on_request<R: Responder>(&self, request: Request, responder: R);
}

pub struct Request {
    pub path: String // TODO: Improve on how we pass URL data to the application
}

pub trait Responder {
    type R: BodyResponder;

    fn start(self, header: Header) -> Self::R;
}

pub trait BodyResponder {
    fn send(&mut self, data: Vec<u8>);
    fn finish(self);
}

pub struct Header {
}
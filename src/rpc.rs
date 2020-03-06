use crate::{consts, Consumer, Producer, Queue};
use crate::authenticator::{Error, Request, Response};

// PRIOR ART:
// https://xenomai.org/documentation/xenomai-2.4/html/api/group__native__queue.html
// https://doc.micrium.com/display/osiiidoc/Using+Message+Queues

type RequestPipeLength = consts::U1;
type ResponsePipeLength = consts::U8;

// only one client served at a time
pub type RequestPipe = Queue::<Request, RequestPipeLength, u8>;
// may need to trigger keepalives
pub type ResponsePipe = Queue::<Result<Response, Error>, ResponsePipeLength, u8>;

/// during setup, allocate pipes (e.g. statically),
/// then split with this function.
pub fn new_endpoints<'a>(
    request_pipe: &'a mut RequestPipe,
    response_pipe: &'a mut ResponsePipe,
) 
    -> (TransportEndpoint<'a>, AuthenticatorEndpoint<'a>)
{
    let (req_send, req_recv) = request_pipe.split();
    let (resp_send, resp_recv) = response_pipe.split();
    let transport_endpoint = TransportEndpoint { recv: resp_recv, send: req_send };
    let authenticator_endpoint = AuthenticatorEndpoint { recv: req_recv, send: resp_send };
    (transport_endpoint, authenticator_endpoint)
}

pub struct AuthenticatorEndpoint<'a> {
    pub recv: Consumer<'a, Request, RequestPipeLength, u8>,
    pub send: Producer<'a, Result<Response, Error>, ResponsePipeLength, u8>,
}

pub struct TransportEndpoint<'a> {
    pub recv: Consumer<'a, Result<Response, Error>, ResponsePipeLength, u8>,
    pub send: Producer<'a, Request, RequestPipeLength, u8>,
}

use crate::local_transport::LocalTransport;

struct Server<'a> {
    transports: [LocalTransport<'a>],
}

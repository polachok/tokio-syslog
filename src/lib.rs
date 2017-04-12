extern crate tokio_io;
extern crate tokio_uds;
extern crate tokio_proto;
extern crate futures;
extern crate bytes;

use std::io;
use std::str;
use std::os::unix::net::SocketAddr;
use std::io::BufWriter;
use std::path::PathBuf;
use bytes::{BytesMut, BufMut};
use tokio_io::codec::{Encoder, Decoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_uds::UnixDatagramCodec;
use tokio_proto::pipeline::ClientProto;

#[allow(non_camel_case_types)] 
#[derive(Copy,Clone,Debug)]
pub enum Facility {
  LOG_KERN     = 0  << 3,
  LOG_USER     = 1  << 3,
  LOG_MAIL     = 2  << 3,
  LOG_DAEMON   = 3  << 3,
  LOG_AUTH     = 4  << 3,
  LOG_SYSLOG   = 5  << 3,
  LOG_LPR      = 6  << 3,
  LOG_NEWS     = 7  << 3,
  LOG_UUCP     = 8  << 3,
  LOG_CRON     = 9  << 3,
  LOG_AUTHPRIV = 10 << 3,
  LOG_FTP      = 11 << 3,
  LOG_LOCAL0   = 16 << 3,
  LOG_LOCAL1   = 17 << 3,
  LOG_LOCAL2   = 18 << 3,
  LOG_LOCAL3   = 19 << 3,
  LOG_LOCAL4   = 20 << 3,
  LOG_LOCAL5   = 21 << 3,
  LOG_LOCAL6   = 22 << 3,
  LOG_LOCAL7   = 23 << 3
}

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Debug)]
pub enum Severity {
    LOG_EMERG,
    LOG_ALERT,
    LOG_CRIT,
    LOG_ERR,
    LOG_WARNING,
    LOG_NOTICE,
    LOG_INFO,
    LOG_DEBUG
}

pub struct LocalSyslogCodec;

impl UnixDatagramCodec for LocalSyslogCodec {
    type In = ();
    type Out = (Severity, Facility, String);

    fn decode(&mut self, _: &SocketAddr, buf: &[u8]) -> std::io::Result<Self::In> {
        panic!("todo")
    }

    fn encode(&mut self, item: Self::Out, buf: &mut Vec<u8>) -> std::io::Result<PathBuf> {
        let prio = item.0 as u8 | item.1 as u8;

        buf.push(b'<');
        buf.extend_from_slice(prio.to_string().as_bytes()); // XXX
        buf.extend_from_slice(b">");
        buf.extend_from_slice(item.2.as_bytes());
        let p = PathBuf::from("/dev/log");
        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    extern crate tokio_core;
    extern crate tokio_service;
    extern crate tokio_uds;
    extern crate tokio_uds_proto;
    use ::{LocalSyslogCodec,Severity,Facility};
    use self::tokio_core::reactor::Core;
    use std::os::unix::net::UnixDatagram as StdUnixDatagram;
    use self::tokio_uds_proto::UnixClient;
    use self::tokio_service::Service;
    use tokio_proto::pipeline::ClientService;
    use tokio_uds::UnixDatagram;
    use futures::sink::Sink;
    use futures::Future;

    #[test]
    fn it_works() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let s = StdUnixDatagram::unbound().unwrap();
        let sock = UnixDatagram::from_datagram(s, &handle).unwrap();
        sock.connect("/dev/log").unwrap();
        let framed = sock.framed(LocalSyslogCodec);
        let f = framed.send((Severity::LOG_EMERG, Facility::LOG_USER, "ty pidor".to_string()));
        core.run(f).unwrap();
    }
}

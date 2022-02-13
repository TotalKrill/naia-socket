use super::server_addr::ServerAddr;

pub fn candidate_to_addr(_candidate_str: &str) -> ServerAddr {
    // TODO: parse a SocketAddr out of candidate_str, probably using regex
    // candidate_str should look something like
    unimplemented!()
}

#[cfg(test)]
mod tests {

    use crate::{server_addr::ServerAddr, wasm_utils::candidate_to_addr};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn candidate_to_addr_works() {
        assert_eq!(
            candidate_to_addr("candidate:1 1 UDP 1755993416 127.0.0.1 14192 typ host"),
            ServerAddr::Found(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                14192
            ))
        );
    }
}

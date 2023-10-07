use std::collections::BTreeMap;

#[derive(Debug)]
struct HashRing {
    ring: BTreeMap<u128, String>,
}

impl HashRing {
    fn new() -> Self {
        Self {
            ring: BTreeMap::new(),
        }
    }

    fn add_server(&mut self, server: impl Into<String>) {
        let server: String = server.into();

        let hash = u128::from_le_bytes(md5::compute(&server).0);

        self.ring.insert(hash, server);
    }

    fn remove_server(&mut self, server: impl AsRef<str>) {
        let hash = u128::from_le_bytes(md5::compute(server.as_ref()).0);
        let _ = self.ring.remove(&hash);
    }

    fn server_for(&self, value: &str) -> Option<&str> {
        let hash = u128::from_le_bytes(md5::compute(value).0);

        if let Some(server) = self
            .ring
            .range(hash..)
            .next()
            .map(|(_hash, server)| server.as_ref())
        {
            return Some(server);
        }

        self.ring
            .first_key_value()
            .map(|(_hash, server)| server.as_ref())
    }
}

fn main() {
    let mut hash_ring = HashRing::new();

    hash_ring.add_server("localhost:8001");
    hash_ring.add_server("localhost:8002");
    hash_ring.add_server("localhost:8003");

    dbg!(&hash_ring);

    dbg!(hash_ring.server_for("key_3"));
    dbg!(hash_ring.server_for("key_2"));
    dbg!(hash_ring.server_for("key_1"));

    hash_ring.remove_server("localhost:8002");

    println!("--- SERVER REMOVED ---");

    dbg!(&hash_ring);

    dbg!(hash_ring.server_for("key_1"));
    dbg!(hash_ring.server_for("key_2"));
    dbg!(hash_ring.server_for("key_3"));
}

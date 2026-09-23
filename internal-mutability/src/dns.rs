use std::collections::HashMap;

pub trait DnsLookup {
    fn lookup(&self, hostname: &str) -> Option<&str>;
}

pub struct DnsClient<'a, TDns : DnsLookup> {
    dns: &'a TDns
}

impl<'a, TDns : DnsLookup> DnsClient<'a, TDns> {
    pub fn new(dns: &'a TDns) -> Self {
        Self { dns }
    }

    pub fn use_dns(&self, hostname: &str) -> Option<&str> {
        self.dns.lookup(hostname)
    }
}

struct Dns {
    lookup_table: HashMap<String, String>
}

impl Dns {
    pub fn from(records: &[(&str, &str)]) -> Self {
        let mut lookup_table = HashMap::new();
        for &(hostname, ip) in records {
            lookup_table.insert(hostname.to_string(), ip.to_string());
        }
        Self { lookup_table }
    }
}

impl DnsLookup for Dns {
    fn lookup(&self, hostname: &str) -> Option<&str> {
        self.lookup_table.get(hostname).map(|s| s.as_str())
    }
}

#[test]
fn test_dns_lookup() {
    let dns = Dns::from(&[("example.com", "67.33.55.128"), ("rust-lang.org", "1.2.3.4")]);

    assert_eq!(dns.lookup("example.com"), Some("67.33.55.128"));
    assert_eq!(dns.lookup("rust-lang.org"), Some("1.2.3.4"));
    assert_eq!(dns.lookup("unknown.com"), None);
}

#[test]
fn test_dns_client() {
    let dns = Dns::from(&[("example.com", "67.33.55.128"), ("rust-lang.org", "1.2.3.4")]);

    let client_1 = DnsClient::new(&dns);
    assert_eq!(client_1.use_dns("example.com"), Some("67.33.55.128"));
    assert_eq!(client_1.use_dns("rust-lang.org"), Some("1.2.3.4"));
    assert_eq!(client_1.use_dns("unknown.com"), None);

    let client_2 = DnsClient::new(&dns);
    assert_eq!(client_2.use_dns("example.com"), Some("67.33.55.128"));
    assert_eq!(client_2.use_dns("rust-lang.org"), Some("1.2.3.4"));
    assert_eq!(client_2.use_dns("unknown.com"), None);
}
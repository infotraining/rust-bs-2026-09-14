pub trait DnsLookup {
    fn lookup(&self, hostname: &str) -> Option<&str>;
}

pub struct DnsClient<'a, TDns: DnsLookup> {
    dns: &'a TDns,
}

impl<'a, TDns: DnsLookup> DnsClient<'a, TDns> {
    pub fn new(dns: &'a TDns) -> Self {
        Self { dns }
    }

    pub fn use_dns(&self, hostname: &str) -> Option<&str> {
        self.dns.lookup(hostname)
    }
}

mod no_shared_mutable_state {
    use super::*;
    use std::collections::HashMap;

    struct Dns {
        lookup_table: HashMap<String, String>,
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
        let dns = Dns::from(&[
            ("example.com", "67.33.55.128"),
            ("rust-lang.org", "1.2.3.4"),
        ]);

        assert_eq!(dns.lookup("example.com"), Some("67.33.55.128"));
        assert_eq!(dns.lookup("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(dns.lookup("unknown.com"), None);
    }

    #[test]
    fn test_dns_client() {
        let dns = Dns::from(&[
            ("example.com", "67.33.55.128"),
            ("rust-lang.org", "1.2.3.4"),
        ]);

        let client_1 = DnsClient::new(&dns);
        assert_eq!(client_1.use_dns("example.com"), Some("67.33.55.128"));
        assert_eq!(client_1.use_dns("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(client_1.use_dns("unknown.com"), None);

        let client_2 = DnsClient::new(&dns);
        assert_eq!(client_2.use_dns("example.com"), Some("67.33.55.128"));
        assert_eq!(client_2.use_dns("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(client_2.use_dns("unknown.com"), None);
    }
}

mod shared_mutable_state_with_cell {
    use super::*;
    use std::cell::Cell;
    use std::collections::HashMap;

    struct Dns {
        lookup_table: HashMap<String, String>,
        pub counter: Cell<u32>,
    }

    impl Dns {
        pub fn from(records: &[(&str, &str)]) -> Self {
            let mut lookup_table = HashMap::new();
            for &(hostname, ip) in records {
                lookup_table.insert(hostname.to_string(), ip.to_string());
            }
            Self {
                lookup_table,
                counter: Cell::new(0),
            }
        }
    }

    impl DnsLookup for Dns {
        fn lookup(&self, hostname: &str) -> Option<&str> {
            let prev_counter = self.counter.get();
            self.counter.set(prev_counter + 1);
            self.lookup_table.get(hostname).map(|s| s.as_str())
        }
    }

    #[test]
    fn test_dns_lookup() {
        let dns = Dns::from(&[
            ("example.com", "67.33.55.128"),
            ("rust-lang.org", "1.2.3.4"),
        ]);

        assert_eq!(dns.lookup("example.com"), Some("67.33.55.128"));
        assert_eq!(dns.lookup("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(dns.lookup("unknown.com"), None);
    }

    #[test]
    fn test_dns_client() {
        let dns = Dns::from(&[
            ("example.com", "67.33.55.128"),
            ("rust-lang.org", "1.2.3.4"),
        ]);

        let client_1 = DnsClient::new(&dns);
        assert_eq!(client_1.use_dns("example.com"), Some("67.33.55.128"));
        assert_eq!(client_1.use_dns("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(client_1.use_dns("unknown.com"), None);

        let client_2 = DnsClient::new(&dns);
        assert_eq!(client_2.use_dns("example.com"), Some("67.33.55.128"));
        assert_eq!(client_2.use_dns("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(client_2.use_dns("unknown.com"), None);

        if let Some(result) = client_1.use_dns("example.com") {
            println!("Result: {result}");
        }
    }
}

mod shared_mutable_state_with_ref_cell {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;

    type LookupLog = Vec<String>;

    struct Dns {
        lookup_table: HashMap<String, String>,
        pub log: RefCell<LookupLog>,
    }

    impl Dns {
        pub fn from(records: &[(&str, &str)]) -> Self {
            let mut lookup_table = HashMap::new();
            for &(hostname, ip) in records {
                lookup_table.insert(hostname.to_string(), ip.to_string());
            }
            Self {
                lookup_table,
                log: RefCell::new(Vec::new()),
            }
        }
    }

    impl DnsLookup for Dns {
        fn lookup(&self, hostname: &str) -> Option<&str> {
            // {
            //     let mut log = self.log.borrow_mut();
            //     log.push(hostname.to_string());
            // }
            self.log.borrow_mut().push(hostname.to_string());

            self.lookup_table.get(hostname).map(|s| s.as_str())
        }
    }

    #[test]
    fn test_dns_lookup() {
        let dns = Dns::from(&[
            ("example.com", "67.33.55.128"),
            ("rust-lang.org", "1.2.3.4"),
        ]);

        assert_eq!(dns.lookup("example.com"), Some("67.33.55.128"));
        assert_eq!(dns.lookup("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(dns.lookup("unknown.com"), None);
    }

    #[test]
    fn test_dns_client() {
        let dns = Dns::from(&[
            ("example.com", "67.33.55.128"),
            ("rust-lang.org", "1.2.3.4"),
        ]);

        let client_1 = DnsClient::new(&dns);
        assert_eq!(client_1.use_dns("example.com"), Some("67.33.55.128"));
        assert_eq!(client_1.use_dns("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(client_1.use_dns("unknown.com"), None);

        let client_2 = DnsClient::new(&dns);
        assert_eq!(client_2.use_dns("example.com"), Some("67.33.55.128"));
        assert_eq!(client_2.use_dns("rust-lang.org"), Some("1.2.3.4"));
        assert_eq!(client_2.use_dns("unknown.com"), None);

        if let Some(result) = client_1.use_dns("example.com") {
            println!("Result: {result}");
        }

        let log = dns.log.borrow();
        println!("Lookup for: {:?}", log);
    }
}

mod shared_mutable_state_with_multithreading {
    use std::collections::HashMap;
    // use std::cell::RefCell;
    use super::*;
    use std::sync::RwLock;

    type LookupLog = Vec<String>;

    struct Dns {
        lookup_table: HashMap<String, String>,
        pub log: RwLock<LookupLog>,
    }

    impl Dns {
        pub fn from(records: &[(&str, &str)]) -> Self {
            let mut lookup_table = HashMap::new();
            for &(hostname, ip) in records {
                lookup_table.insert(hostname.to_string(), ip.to_string());
            }
            Self {
                lookup_table,
                log: RwLock::new(Vec::new()),
            }
        }
    }

    impl DnsLookup for Dns {
        fn lookup(&self, hostname: &str) -> Option<&str> {
            {
                let mut log = self.log.write().unwrap();
                log.push(hostname.to_string());
            }

            self.lookup_table.get(hostname).map(|s| s.as_str())
        }
    }

    #[test]
    fn test_dns_lookup_multithreading() {
        use std::sync::Arc;

        let dns = Arc::new(Dns::from(&[
            ("example.com", "67.33.55.128"),
            ("rust-lang.org", "1.2.3.4"),
        ]));

        let dns_shared = Arc::clone(&dns);
        let thd_1 = std::thread::spawn(move || {
            let result = dns_shared.lookup("example.com");

            println!(
                "Lookup from thread {:?}: {:?}",
                std::thread::current().id(),
                result
            );
        });

        let dns_shared = Arc::clone(&dns);
        let thd_2 = std::thread::spawn(move || {
            let result = dns_shared.lookup("example.com");

            println!(
                "Lookup from thread {:?}: {:?}",
                std::thread::current().id(),
                result
            );
        });

        thd_1.join().unwrap();
        thd_2.join().unwrap();

        println!("Lookup log: {:?}", dns.log.read().unwrap());
    }
}

# AnyMap Crate

[credits](https://zsiciarz.github.io/24daysofrust/book/vol1/day9.html)
[anymap](https://crates.io/crates/anymap)

Map a key to a value where the key is a type and the value is an instance of that type.

- Store configuration data
- Singleton

Create lots of simple types, for example

```rust,editable
extern crate anymap;

use std::net::Ipv4Addr;
use anymap::AnyMap;

#[derive(Debug)]
enum HostAddress {
    DomainName(String),
    Ip(Ipv4Addr),
}

#[derive(Debug)]
struct Port(u32);

#[derive(Debug)]
struct ConnectionLimit(u32);

let mut config = AnyMap::new();
config.insert(HostAddress::DomainName("siciarz.net".to_string()));
config.insert(Port(666));
config.insert(ConnectionLimit(32));
println!("{:?}", config.get::<HostAddress>());
println!("{:?}", config.get::<Port>());
assert!(config.get::<String>().is_none());
assert!(config.get::<u32>().is_none());
```

Here we have types e.g. **Port** instead of having a port being a member of some struct we can have an AnyMap and have Port as one of the keys.

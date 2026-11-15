use super::*;

#[test]
fn server_addresses_default_to_the_minecraft_port() {
    let upstream = |host: &str, port| Upstream { host: host.to_owned(), port };
    assert_eq!(Upstream::parse("play.example.net").unwrap(), upstream("play.example.net", 25565));
    assert_eq!(Upstream::parse("127.0.0.1:25566").unwrap(), upstream("127.0.0.1", 25566));
    assert_eq!(Upstream::parse("[::1]:25566").unwrap(), upstream("::1", 25566));
    assert_eq!(Upstream::parse("::1").unwrap(), upstream("::1", 25565));
    assert!(Upstream::parse("example.net:port").is_err());
}

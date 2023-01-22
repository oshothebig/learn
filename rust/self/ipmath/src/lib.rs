use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub fn add_ipv4(lhs: Ipv4Addr, rhs: u32) -> Ipv4Addr {
    let octets = lhs.octets();
    let num_repr = u32::from_be_bytes(octets).wrapping_add(rhs);
    Ipv4Addr::from(num_repr)
}

#[test]
fn test_add_ipv4_one() {
    let lhs = Ipv4Addr::new(192, 168, 0, 1);
    let actual = add_ipv4(lhs, 1);
    assert_eq!(actual, Ipv4Addr::new(192, 168, 0, 2));
}

#[test]
fn test_add_ipv4_large() {
    let lhs = Ipv4Addr::new(192, 168, 0, 1);
    let actual = add_ipv4(lhs, 256);
    assert_eq!(actual, Ipv4Addr::new(192, 168, 1, 1))
}

#[test]
fn test_add_ipv4_beyond_max() {
    let lhs = Ipv4Addr::new(255, 255, 255, 254);
    let actual = add_ipv4(lhs, 2);
    assert_eq!(actual, Ipv4Addr::new(0, 0, 0, 0));
}

pub fn add_ipv6(lhs: Ipv6Addr, rhs: u128) -> Ipv6Addr {
    let octets = lhs.octets();
    let num_repr = u128::from_be_bytes(octets) + rhs;
    Ipv6Addr::from(num_repr)
}

#[test]
fn test_add_ipv6_one() {
    let lhs: Ipv6Addr = "::1".parse().unwrap();
    let actual = add_ipv6(lhs, 1);
    assert_eq!(actual, "::2".parse::<Ipv6Addr>().unwrap());
}

pub fn add(lhs: IpAddr, rhs: u128) -> IpAddr {
    match lhs {
        IpAddr::V4(addr) => IpAddr::from(add_ipv4(addr, rhs as u32)),
        IpAddr::V6(addr) => IpAddr::from(add_ipv6(addr, rhs)),
    }
}

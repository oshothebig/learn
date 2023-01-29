use std::iter::{IntoIterator, Iterator};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(PartialEq, Debug, Copy, Clone)]
struct Ipv4 {
    ip: u32,
}

struct Ipv4Prefix {
    ip: Ipv4,
    length: u8,
}

impl Ipv4 {
    fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4 {
        Ipv4 {
            ip: u32::from_be_bytes([a, b, c, d]),
        }
    }

    fn next(&self) -> Ipv4 {
        Ipv4 { ip: self.ip + 1 }
    }

    fn prior(&self) -> Ipv4 {
        Ipv4 { ip: self.ip - 1 }
    }
}

#[test]
fn test_ipv4_prior() {
    let net = Ipv4::new(192, 168, 0, 1);
    assert_eq!(net.prior(), Ipv4::new(192, 168, 0, 0));
}

impl Ipv4Prefix {
    fn new(ip: Ipv4, length: u8) -> Ipv4Prefix {
        Ipv4Prefix { ip, length }
    }

    fn last(&self) -> Ipv4 {
        let net_bits = self.length;
        let host_bits = 32 - net_bits;
        let host_mask = (1 << host_bits) - 1;
        let net_mask = !host_mask;
        Ipv4 {
            ip: (self.ip.ip & net_mask) | host_mask,
        }
    }
}

#[test]
fn test_ipv4_prefix_last_24_bits() {
    let net = Ipv4Prefix::new(Ipv4::new(192, 168, 0, 0), 24);
    let actual = net.last();
    assert_eq!(actual, Ipv4::new(192, 168, 0, 255));
}

#[test]
fn test_ipv4_prefix_last_23_bits() {
    let net = Ipv4Prefix::new(Ipv4::new(192, 168, 2, 0), 23);
    let actual = net.last();
    assert_eq!(actual, Ipv4::new(192, 168, 3, 255));
}

#[test]
fn test_ipv4_prefix_last_32_bit() {
    let net = Ipv4Prefix::new(Ipv4::new(192, 168, 0, 1), 32);
    let actual = net.last();
    assert_eq!(actual, Ipv4::new(192, 168, 0, 1));
}

impl From<[u8; 4]> for Ipv4 {
    fn from(octets: [u8; 4]) -> Self {
        Ipv4 {
            ip: u32::from_be_bytes(octets),
        }
    }
}

impl From<Ipv4Addr> for Ipv4 {
    fn from(ipv4: Ipv4Addr) -> Self {
        Ipv4::from(ipv4.octets())
    }
}

impl IntoIterator for Ipv4 {
    type Item = Ipv4;
    type IntoIter = Ipv4Iterator;

    fn into_iter(self) -> Self::IntoIter {
        Ipv4Iterator {
            next: Some(self),
            last: Ipv4::new(255, 255, 255, 255),
        }
    }
}

impl IntoIterator for Ipv4Prefix {
    type Item = Ipv4;
    type IntoIter = Ipv4Iterator;

    fn into_iter(self) -> Self::IntoIter {
        Ipv4Iterator {
            next: Some(self.ip),
            last: self.last(),
        }
    }
}

struct Ipv4Iterator {
    next: Option<Ipv4>,
    last: Ipv4,
}

impl Iterator for Ipv4Iterator {
    type Item = Ipv4;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        match next {
            Some(x) => {
                if x == self.last {
                    self.next = None;
                } else {
                    self.next = Some(x.next());
                }
                next
            }
            None => None,
        }
    }
}

#[test]
fn test_ipv4_iter() {
    let mut iter = Ipv4::new(192, 168, 0, 1).into_iter();
    assert_eq!(iter.next(), Some(Ipv4::new(192, 168, 0, 1)));
    assert_eq!(iter.next(), Some(Ipv4::new(192, 168, 0, 2)));
}

#[test]
fn test_ipv4_iter_final() {
    let mut iter = Ipv4::new(255, 255, 255, 254).into_iter();
    assert_eq!(iter.next(), Some(Ipv4::new(255, 255, 255, 254)));
    assert_eq!(iter.next(), Some(Ipv4::new(255, 255, 255, 255)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_ipv4_prefix_iter() {
    let mut iter = Ipv4Prefix::new(Ipv4::new(192, 168, 0, 0), 24).into_iter();
    assert_eq!(iter.next(), Some(Ipv4::new(192, 168, 0, 0)));
    assert_eq!(iter.next(), Some(Ipv4::new(192, 168, 0, 1)));
}

#[test]
fn test_new_ipv4() {
    let sut = Ipv4::new(192, 168, 0, 1);
    // 0xc0 => 192, 0xa8 => 168, 0x00 => 0, 0x01 => 1
    assert_eq!(sut.ip, 0xc0_a8_00_01);
}

#[test]
fn test_from_byte_array() {
    let sut = Ipv4::from([192, 168, 0, 1]);
    assert_eq!(sut, Ipv4::new(192, 168, 0, 1));
}

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

#[test]
fn test_add_one() {
    let lhs = IpAddr::from([192, 168, 0, 1]);
    let actual = add(lhs, 10);
    assert_eq!(actual, IpAddr::from([192, 168, 0, 11]));
}

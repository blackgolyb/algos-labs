use std::cmp::Ordering;
use std::io::prelude::*;
use std::{fmt, str::FromStr};

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OS {
    #[serde(rename = "Windows")]
    Windows,
    #[serde(rename = "Debian")]
    Debian,
    #[serde(rename = "NixOS")]
    NixOS,
    #[serde(rename = "Ubuntu")]
    Ubuntu,
    #[serde(rename = "Fedora")]
    Fedora,
    #[serde(rename = "CentOS")]
    CentOS,
    #[serde(rename = "Arch")]
    Arch,
    #[serde(rename = "MacOS")]
    MacOS,
    #[serde(rename = "FreeBSD")]
    FreeBSD,
    #[serde(rename = "OpenBSD")]
    OpenBSD,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Protocol {
    #[serde(rename = "TCP")]
    TCP,
    #[serde(rename = "UDP")]
    UDP,
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "QUIC")]
    QUIC,
    #[serde(rename = "SMTP")]
    SMTP,
    #[serde(rename = "FTP")]
    FTP,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Ipv4Address {
    octets: [u8; 4],
}

impl Distribution<OS> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OS {
        match rng.gen_range(0..=10) {
            0 => OS::Windows,
            1 => OS::Debian,
            2 => OS::NixOS,
            3 => OS::Ubuntu,
            4 => OS::Fedora,
            5 => OS::CentOS,
            6 => OS::Arch,
            7 => OS::MacOS,
            8 => OS::FreeBSD,
            _ => OS::OpenBSD,
        }
    }
}

impl Distribution<Protocol> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Protocol {
        match rng.gen_range(0..=6) {
            0 => Protocol::TCP,
            1 => Protocol::UDP,
            2 => Protocol::HTTP,
            3 => Protocol::QUIC,
            4 => Protocol::SMTP,
            _ => Protocol::FTP,
        }
    }
}

impl fmt::Display for Ipv4Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:03}.{:03}.{:03}.{:03}",
            self.octets[0], self.octets[1], self.octets[2], self.octets[3]
        )
    }
}

pub struct ParseIpAdrrError {
    // TODO: add more information
}

impl fmt::Display for ParseIpAdrrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse ip addres")
    }
}

impl FromStr for Ipv4Address {
    type Err = ParseIpAdrrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octets: Result<Vec<u8>, _> = s.split('.').map(|part| part.parse()).collect();

        match octets {
            Ok(octets) if octets.len() == 4 => Ok(Ipv4Address {
                octets: [octets[0], octets[1], octets[2], octets[3]],
            }),
            _ => Err(ParseIpAdrrError {}),
        }
    }
}

// Custom serialization for Ipv4Address
impl Serialize for Ipv4Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// Custom deserialization for Ipv4Address
impl<'de> Deserialize<'de> for Ipv4Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Distribution<Ipv4Address> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ipv4Address {
        Ipv4Address {
            octets: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Data {
    pub ip: Ipv4Address,
    pub os: OS,
    pub protocol: Protocol,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Data {{ ip: {},\tos: {:?},\tprotocol: {:?}\t }}",
            self.ip, self.os, self.protocol
        )
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip
    }
}

impl Eq for Data {}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ip.cmp(&other.ip)
    }
}

impl Distribution<Data> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Data {
        Data {
            ip: rng.gen(),
            os: rng.gen(),
            protocol: rng.gen(),
        }
    }
}

pub fn generate_unique_ip_addresses(n: usize) -> Vec<Ipv4Address> {
    let mut rng = rand::thread_rng();
    let mut res = Vec::new();

    while res.len() < n {
        let addr: Ipv4Address = rng.gen();
        if !res.contains(&addr) {
            res.push(addr);
        }
    }

    res
}

pub fn generate_unique_data(n: usize) -> Vec<Data> {
    let mut rng = rand::thread_rng();
    generate_unique_ip_addresses(n)
        .into_iter()
        .map(|ip| Data {
            ip,
            os: rng.gen(),
            protocol: rng.gen(),
        })
        .collect::<Vec<Data>>()
}

pub fn default_unique_data(n: usize) -> Vec<Data> {
    let mut i = 0;
    let mut vec = Vec::<Data>::new();
    for a in 204..=255 {
        for b in 111..=255 {
            for c in 0..=255 {
                for d in 0..=255 {
                    if i >= n {
                        return vec;
                    }

                    let ip = Ipv4Address {
                        octets: [a, b, c, d],
                    };
                    let data = Data {
                        ip,
                        os: OS::Arch,
                        protocol: Protocol::TCP,
                    };
                    vec.push(data);
                    i += 1;
                }
            }
        }
    }
    vec
}

pub fn generate_test_file(file: &mut std::fs::File, n: usize) {
    let data = generate_unique_data(n);
    let serialized_data = serde_json::to_string(&data).unwrap();

    file.write_all(serialized_data.as_bytes())
        .expect("Unable write into file");
}

pub fn read_data_from_file(file: &mut std::fs::File) -> Vec<Data> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable read from file");

    serde_json::from_str(&contents).unwrap()
}

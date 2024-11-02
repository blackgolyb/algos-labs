use std::cmp::Ordering;
use std::io::prelude::*;
use std::{fmt, str::FromStr};

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::libs::hash_table::HashTable;
use crate::libs::list::double_linked_list::List;
use crate::libs::search::binary_search;
use crate::libs::search::linear_search;
use crate::libs::search::logger::{Logger, Metrics};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum OS {
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
enum Protocol {
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
struct Ipv4Address {
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
struct Data {
    ip: Ipv4Address,
    os: OS,
    protocol: Protocol,
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

fn generate_unique_ip_addresses(n: usize) -> Vec<Ipv4Address> {
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

fn generate_unique_data(n: usize) -> Vec<Data> {
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

fn default_unique_data(n: usize) -> Vec<Data> {
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

fn generate_test_file(file: &mut std::fs::File, n: usize) {
    let data = generate_unique_data(n);
    let serialized_data = serde_json::to_string(&data).unwrap();

    file.write_all(serialized_data.as_bytes())
        .expect("Unable write into file");
}

fn read_data_from_file(file: &mut std::fs::File) -> Vec<Data> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable read from file");

    serde_json::from_str(&contents).unwrap()
}

pub fn main() {
    let n = 10;
    let elem_rate = 0.66666666666;
    let elem_id = (n as f64 * elem_rate) as usize;
    let file_name = format!("data_{n}.json");

    // let mut file = std::fs::File::create(&file_name).expect("Unable to create file");
    // generate_test_file(&mut file, n);

    // let mut file = std::fs::File::open(&file_name).expect("Unable to create file");
    // let mut data_vec = read_data_from_file(&mut file);
    //
    let mut data_vec = generate_unique_data(n);
    // let mut data_vec = default_unique_data(n);
    println!("generated\n");

    let mut hash_table = HashTable::<Ipv4Address, Data>::new_with_capacity(n + 1);
    let mut list = List::<Data>::new();

    data_vec.sort();
    let elem = data_vec[elem_id];

    for data in data_vec {
        hash_table.insert(data.ip, data);
        list.push(data);
    }

    println!("{}", hash_table);

    let mut logger = Logger::new();

    let res1 = linear_search(&mut list, elem, &mut logger);
    let m1 = logger.get_metrics();

    let res2 = binary_search(&mut list, elem, &mut logger);
    let m2 = logger.get_metrics();

    logger.start();
    let res3 = hash_table.get(elem.ip);
    logger.end();
    let m3 = logger.get_metrics();

    println!("{} -- {}", res1.is_some(), m1);
    println!("{} -- {}", res2.is_some(), m2);
    println!("{} -- {}", res3.is_some(), m3);
}

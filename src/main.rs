// An attribute to hide warnings for dead code, unused imports and unused variables
#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]
extern crate pnet;
extern crate rand;
use rand::Rng;
use std::cmp;
use pnet::packet::icmp::{IcmpCode, IcmpTypes, MutableIcmpPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::{self, Packet};
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::{icmp_packet_iter, transport_channel};
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    if env::args().len() < 2 {
        println!("Usage: ping target_address");
        return;
    }
    let target_ip: IpAddr = env::args().nth(1).unwrap().parse().expect("Parse failed");

    let protocol_icmp = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut transmitter, mut receiver) = transport_channel(1024, protocol_icmp).unwrap();

    let mut max_time = 0;
    let mut min_time = u32::MAX;
    let mut average = 0u32;
    let mut num_pings = 20;
    println!("Pinging {} with 32 bytes of data", target_ip);
    for i in 0..num_pings {
        let payload: Vec<u8> = vec![0u8];
        let mut buffer = [0u8; 64]; // Adjust the size as needed
        let mut icmp_packet = MutableIcmpPacket::new(&mut buffer).unwrap();
        let mut payload: Vec<u8> = vec![0u8; 32];
        rand::thread_rng().fill(&mut payload[..]);
        icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
        icmp_packet.set_icmp_code(IcmpCode::new(0));
        icmp_packet.set_checksum(0);
        icmp_packet.set_payload(&payload);
        let checksum: u16 = pnet::packet::icmp::checksum(&icmp_packet.to_immutable());
        icmp_packet.set_checksum(checksum);
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        match transmitter.send_to(icmp_packet, target_ip) {
            Ok(s) => {
                let mut iter = icmp_packet_iter(&mut receiver);
                match iter.next() {
                    Ok((packet, addr)) => {
                        let end = SystemTime::now();
                        let since_start = end.duration_since(start).expect("Time went backwards");
                        let since_start_in_ms = since_start.as_millis();
                        println!("Reply from: {} in {}ms", addr, since_start_in_ms);
                        max_time = cmp::max(max_time, since_start_in_ms as u32);
                        min_time = cmp::min(min_time, since_start_in_ms as u32);
                        average += since_start_in_ms as u32;
                    }
                    Err(e) => panic!("ICMP packet receive error: {}", e),
                }
            }
            Err(e) => println!("Failed to send ICMP Packet, error: {}", e),
        }
    }
    average = average/num_pings;
    println!("Approximate round trip times in milli-seconds: ");
    println!("Max: {}ms, Min: {}ms, Average: {}ms", max_time, min_time, average);
}

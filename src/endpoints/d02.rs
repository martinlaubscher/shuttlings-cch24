use axum::http::StatusCode;
use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Deserialize)]
pub struct ParamsDest {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

#[derive(Deserialize)]
pub struct ParamsKey {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

#[derive(Deserialize)]
pub struct ParamsDestV6 {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

#[derive(Deserialize)]
pub struct ParamsKeyV6 {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

pub(crate) async fn dest(query: Query<ParamsDest>) -> impl IntoResponse {
    let params_dest: ParamsDest = query.0;
    let from_octets = params_dest.from.octets();
    let key_octets = params_dest.key.octets();
    let mut dest_octets: [u8; 4] = [0; 4];

    from_octets
        .iter()
        .zip(key_octets.iter())
        .enumerate()
        .for_each(|(i, (&from, &key))| {
            let mut dest = from as u16 + key as u16;
            while dest > 255 {
                dest -= 256;
            }
            dest_octets[i] = dest as u8;
        });

    (StatusCode::OK, Ipv4Addr::from(dest_octets).to_string())
}

pub(crate) async fn key(query: Query<ParamsKey>) -> impl IntoResponse {
    let params_key: ParamsKey = query.0;
    let from_octets = params_key.from.octets();
    let to_octets = params_key.to.octets();
    let mut key_octets: [u8; 4] = [0; 4];

    from_octets
        .iter()
        .zip(to_octets.iter())
        .enumerate()
        .for_each(|(i, (&from, &to))| {
            let mut key = to as i16 - from as i16;
            while key < 0 {
                key += 256;
            }
            key_octets[i] = key as u8;
        });

    (StatusCode::OK, Ipv4Addr::from(key_octets).to_string())
}

pub(crate) async fn dest_v6(query: Query<ParamsDestV6>) -> impl IntoResponse {
    let params_dest: ParamsDestV6 = query.0;
    let from_segments = params_dest.from.segments();
    let key_segments = params_dest.key.segments();

    (StatusCode::OK, xor(from_segments, key_segments))
}

pub(crate) async fn key_v6(query: Query<ParamsKeyV6>) -> impl IntoResponse {
    let params_dest: ParamsKeyV6 = query.0;
    let from_segments = params_dest.from.segments();
    let to_segments = params_dest.to.segments();

    (StatusCode::OK, xor(from_segments, to_segments))
}

fn xor(segments_1: [u16; 8], segments_2: [u16; 8]) -> String {
    let mut res_segments: [u16; 8] = [0; 8];

    segments_1
        .iter()
        .zip(segments_2.iter())
        .enumerate()
        .for_each(|(i, (&from, &key))| {
            res_segments[i] = from ^ key;
        });
    Ipv6Addr::from(res_segments).to_string()
}

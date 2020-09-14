use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark, Throughput};
use rand::{thread_rng, Rng};
use rand::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use sha2raw::Sha256;
use std::io::{self, Read};
use std::time::Duration;
use sha2raw::sha256_intrinsics;

use sha2raw::consts::H256;
use sha2raw::platform::Implementation;
use std::convert::TryFrom;
lazy_static::lazy_static! {
    static ref IMPL: Implementation = Implementation::detect();
}

fn compress256(sha: &mut Sha256, data: u32) -> [u8; 32] {
    let porep_id = [data as u8, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8,
        9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
    let  replica_id = hex::decode("2e7f22963af99c96a7acef7aa00b4456eda4d684d3f7d8de0c000a7f2c6deff6").unwrap();
    let mut buffer = [0u8; 32];
    buffer[..4].copy_from_slice(&(data).to_be_bytes());
    buffer[4..12].copy_from_slice(&(data as u64).to_be_bytes());
    println!("init:{:x?}", &[AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]][..]);
    sha.input(&[AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]][..]);
    println!("state:{:x?}", sha.state);
    let rng = &mut XorShiftRng::from_seed([
        data as u8, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x33, 0x84, 0x12, 0xdf, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);


    let mut rng_input = vec![0u8; 192];
    //rng.fill_bytes(&mut rng_input);
    let chunked = rng_input.chunks(32).collect::<Vec<_>>();
    println!("chunked:{:x?}", chunked);
    sha.input(chunked.as_slice());
    println!("state:{:x?}", sha.state);
    sha.input(chunked.as_slice());
    println!("state:{:x?}", sha.state);
    sha.input(chunked.as_slice());
    println!("state:{:x?}", sha.state);
    sha.input(chunked.as_slice());
    println!("state:{:x?}", sha.state);
    sha.input(chunked.as_slice());
    println!("state:{:x?}", sha.state);
    sha.input(chunked.as_slice());
    println!("state:{:x?}", sha.state);
    //println!("sha.len:{:?}", sha.len);
    let out = sha.finish_with(&chunked.as_slice()[0]);
    println!("state:{:x?}", sha.state);
    println!("out:{:x?}", out);
    panic!("========================");
    out
}

fn compress256_exp(sha: &mut Sha256, data: u32) -> [u8; 32] {
    let porep_id = [data as u8, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8,
        9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];

    let rng = &mut XorShiftRng::from_seed([
        data as u8, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x33, 0x84, 0x12, 0xdf, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);
    let  replica_id = hex::decode("2e7f22963af99c96a7acef7aa00b4456eda4d684d3f7d8de0c000a7f2c6deff6").unwrap();
    let mut buffer = [0u8; 32];
    buffer[..4].copy_from_slice(&(data).to_be_bytes());
    buffer[4..12].copy_from_slice(&(data as u64).to_be_bytes());
    println!("init:{:x?}", &[AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]][..]);
    sha.input(&[AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]][..]);
    println!("state:{:x?}", sha.state);
    let mut rng_input = vec![0u8; 448];
    //rng.fill_bytes(&mut rng_input);
    let chunked = rng_input.chunks(32).collect::<Vec<_>>();
    //let porep_id = porep_id.chunks(32).collect::<Vec<_>>();
    //println!("chunked:{:?} len:{:?}", chunked.as_slice(), chunked.len());
    println!("chunked:{:x?}", chunked);
    sha.input(chunked.as_slice());
    println!("state:{:x?}", sha.state);
    sha.input(chunked.as_slice());
    println!("state:{:x?}", sha.state);
    sha.input(&chunked.as_slice()[..8]);
    println!("state:{:x?}", sha.state);
    let out = sha.finish_with(&chunked.as_slice()[8]);
    let s = sha.state;
    println!("out:{:x?}", out);
    panic!("______________________________________");
    out
}

fn compress256_benchmark(c: &mut Criterion) {
    c.bench(
        "compress256_benchmark",
        ParameterizedBenchmark::new(
            "compress256_benchmark",
            |b, size| {
                let mut sha = Sha256::new();
                b.iter(|| black_box(
                    {
                        let out =  compress256(&mut sha, H256[4]);
                        compress256(&mut sha, out[9] as u32)
                    }));
            },
            vec![10],
            //vec![128, 256, 512, 256_000, 512_000, 1_024_000, 2_048_000],
        )
            .sample_size(10)
            .throughput(|s| Throughput::Bytes(*s as u64))
            .warm_up_time(Duration::from_secs(1)),
    );
}

criterion_group!(benches, compress256_benchmark);
criterion_main!(benches);

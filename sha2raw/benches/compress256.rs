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

fn compress256(sha: &mut Sha256, data: &u64) -> [u8; 32] {
    let  replica_id = hex::decode("2e7f22963af99c96a7acef7aa00b4456eda4d684d3f7d8de0c000a7f2c6deff6").unwrap();
    //print!("sha.state  ={:x?} ", &sha.state);
    let mut buffer = [0u8; 32];
    buffer[..4].copy_from_slice(&(1 as u32).to_be_bytes());
    buffer[4..12].copy_from_slice(&(*data).to_be_bytes());

    println!("\nthenode[k].layer     =1;");
    print!("thenode[k].repid    ={} ", "{");
    for (i, id) in replica_id.iter().enumerate() {
        if i == replica_id.len()-1 {
            println!("8'h{:x?} {}", id, "};");
        } else {
            print!("8'h{:x?}, ", id);
        }
    }

    print!("thenode[k].layerid    ={} ", "{");
    for (i, layer) in buffer[..4][..].iter().enumerate() {
        if i == 3 {
            println!("8'h{:x?} {}", layer, "};");
        }
        else {
            print!("8'h{:x?}, ", layer);
        }

    }
    print!("thenode[k].nodeid    ={} ", "{");
    for (i, node) in buffer[4..12][..].iter().enumerate() {
        if i == 7 {
            println!("8'h{:x?} {}", node, "};");
        }
        else {
            print!("8'h{:x?}, ", node);
        }
    }

    sha.input(&[AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]][..]);
    println!("");
    // for (_, da) in  [AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]].iter().enumerate() {
    //     for (_, d) in da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    println!("");
    print!("thenode[k].state    ={} ", "{");
    for (i, st) in sha.state.iter().enumerate() {
        if i == sha.state.len()-1 {
            println!("32'h{:08x?} {}", st, "};");
        }
        else {
            print!("32'h{:08x?}, ", st);
        }
    }

    let rng = &mut XorShiftRng::from_seed([
        0x01, 0x67, 0xbe, 0x5d, 0x76, 0x3d, 0x33, 0x84, 0x12, 0xdf, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);


    let mut rng_input = vec![0u8; 192];
    rng.fill_bytes(&mut rng_input);
    let chunked = rng_input.chunks(32).collect::<Vec<_>>();
    //println!("data:{:x?}", chunked);

    for (i, datas) in chunked.iter().enumerate() {
        print!("thenode[k].nodein[{}] ={}", i, "{ ");
        for (i, d) in datas.iter().enumerate() {
            if i == datas.len()-1 {
                println!("8'h{:x?} {}", d,"};");
            } else {
                print!("8'h{:x?}, ", d);
            }
        }
    }

    sha.input(chunked.as_slice());
    // for (_, da) in  chunked.iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    sha.input(chunked.as_slice());
    // for (_, da) in  chunked.iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    sha.input(chunked.as_slice());
    // for (_, da) in  chunked.iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    sha.input(chunked.as_slice());
    // for (_, da) in  chunked.iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    sha.input(chunked.as_slice());
    // for (_, da) in  chunked.iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    sha.input(chunked.as_slice());
    // for (_, da) in  chunked.iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    let out = sha.finish_with(&chunked.as_slice()[0]);
    // for (_, da) in  chunked.as_slice()[0].iter().enumerate() {
    //         print!("{:02x?}", da);
    //
    // }
    print!("\nthenode[k].nodeout     ={}", "{ ");
    for (i, o) in out.iter().enumerate() {
        if i == out.len()-1 {
            println!("8'h{:x?} {}", o, "};");
        } else {
            print!("8'h{:x?}, ", o);
        }
    }
    out
}

fn compress256_exp(sha: &mut Sha256, data: &u64) -> [u8; 32] {
    let rng = &mut XorShiftRng::from_seed([
        0x01, 0x67, 0xbe, 0x5d, 0x76, 0x3d, 0x33, 0x84, 0x12, 0xdf, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);
    //print!("sha.state  ={:x?} ", &sha.state);
    let  replica_id = hex::decode("2e7f22963af99c96a7acef7aa00b4456eda4d684d3f7d8de0c000a7f2c6deff6").unwrap();
    let mut buffer = [0u8; 32];
    buffer[..4].copy_from_slice(&(2 as u32).to_be_bytes());
    buffer[4..12].copy_from_slice(&(*data as u64).to_be_bytes());
    sha.input(&[AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]][..]);

    //println!("\nsha.state:{:x?}", sha.state);
    // for (_, da) in  [AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]].iter().enumerate() {
    //     for (_, d) in da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    let mut rng_input = vec![0u8; 448];
    rng.fill_bytes(&mut rng_input);
    let chunked = rng_input.chunks(32).collect::<Vec<_>>();

    println!("\nthenode[k].layer     =2;");
    print!("thenode[k].repid    ={} ", "{");
    for (i, id) in replica_id.iter().enumerate() {
        if i == replica_id.len()-1 {
            println!("8'h{:x?} {}", id, "};");
        } else {
            print!("8'h{:x?}, ", id);
        }
    }

    print!("thenode[k].layerid    ={} ", "{");
    for (i, layer) in buffer[..4][..].iter().enumerate() {
        if i == 3 {
            println!("8'h{:x?} {}", layer, "};");
        }
        else {
            print!("8'h{:x?}, ", layer);
        }

    }
    print!("thenode[k].nodeid    ={} ", "{");
    for (i, node) in buffer[4..12][..].iter().enumerate() {
        if i == 7 {
            println!("8'h{:x?} {}", node, "};");
        }
        else {
            print!("8'h{:x?}, ", node);
        }
    }

    //sha.input(&[AsRef::<[u8]>::as_ref(&replica_id), &buffer[..]][..]);
    print!("thenode[k].state    ={} ", "{");
    for (i, st) in sha.state.iter().enumerate() {
        if i == sha.state.len()-1 {
            println!("32'h{:08x?} {}", st, "};");
        }
        else {
            print!("32'h{:08x?}, ", st);
        }
    }

    for (i, datas) in chunked.iter().enumerate() {
        print!("thenode[k].nodein[{}] ={}", i, "{ ");
        for (i, d) in datas.iter().enumerate() {
            if i == datas.len()-1 {
                println!("8'h{:x?} {}", d,"};");
            } else {
                print!("8'h{:x?}, ", d);
            }
        }
    }

    sha.input(chunked.as_slice());
    // for (_, da) in  chunked.iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    //println!("state:{:x?}", sha.state);
    sha.input(chunked.as_slice());
    // for (_, da) in  chunked.iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    //println!("state:{:x?}", sha.state);
    sha.input(&chunked.as_slice()[..8]);
    // for (_, da) in  chunked.as_slice()[..8].iter().enumerate() {
    //     for (_, d) in  da.iter().enumerate() {
    //         print!("{:02x?}", d);
    //     }
    // }
    //println!("state:{:x?}", sha.state);
    let out = sha.finish_with(&chunked.as_slice()[8]);
    // for (_, da) in  chunked.as_slice()[8].iter().enumerate() {
    //     print!("{:02x?}", da);
    // }
    let s = sha.state;
    //println!("out:{:x?}", out);
    print!("\nthenode[k].nodeout     ={}", "{ ");
    for (i, o) in out.iter().enumerate() {
        if i == out.len()-1 {
            println!("8'h{:x?} {}", o, "};");
        } else {
            print!("8'h{:x?}, ", o);
        }
    }
    out
}

fn compress256_benchmark(c: &mut Criterion) {
    c.bench(
        "compress256_benchmark",
        ParameterizedBenchmark::new(
            "compress256_benchmark",
            |b, size| {

                let mut node:u64 = 0;
                b.iter(|| black_box(
                    {
                        node = node+1;
                        //println!("node = {:?}", node);

                        let mut sha1 = Sha256::new();
                        let out1 =  compress256(&mut sha1, &node);
                        println!("\nk++;");
                        let mut sha2 = Sha256::new();
                        let out2 =  compress256_exp(&mut sha2, &node);
                        println!("\nk++;");
                        //compress256(&mut sha, out[9] as u32)
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

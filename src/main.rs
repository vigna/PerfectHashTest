use core::hint::black_box;
use std::{collections::HashMap, time::Instant};

use clap::Parser;
use ph::fmph;
use ptr_hash::{PtrHash, PtrHashParams};
use rand::{RngCore, SeedableRng, rngs::SmallRng, seq::SliceRandom};

#[derive(Parser, Debug)]
#[command(about = "Build contributor and origin collaboration graphs")]
struct Args {
    n: usize,
}

fn main() {
    let args = Args::parse();
    let mut r = SmallRng::seed_from_u64(0);

    let n = args.n;

    let mut map = HashMap::with_capacity(n);

    let mut keys = Vec::with_capacity(n);
    keys.resize_with(n, || r.next_u64());

    keys.iter().for_each(|&k| {
        map.insert(k, k);
    });

    keys.shuffle(&mut r);

    let ptr_hash = <PtrHash>::new(keys.as_slice(), PtrHashParams::default());
    let mut values = vec![0; keys.len()];
    map.iter().for_each(|(k, v)| {
        values[ptr_hash.index(k)] = *v;
    });

    let now = Instant::now();
    for i in 0..n {
        let _: _ = black_box(*unsafe { map.get(&keys[i]).unwrap_unchecked() });
    }
    println!(
        "HashMap lookup time {:8.3}µs",
        now.elapsed().as_nanos() as f64 / 1000.0
    );

    let now = Instant::now();
    for i in 0..n {
        debug_assert_eq!(
            values[ptr_hash.index(&keys[i])],
            *map.get(&keys[i]).unwrap()
        );
        _ = black_box(values[ptr_hash.index(&keys[i])]);
    }
    println!(
        "PtrHash lookup time {:8.3}µs",
        now.elapsed().as_nanos() as f64 / 1000.0
    );

    drop(ptr_hash);

    let phast = fmph::Function::from(keys.as_ref());

    let mut values = vec![0; keys.len()];
    map.iter().for_each(|(k, v)| {
        values[phast.get(k).unwrap() as usize] = *v;
    });

    let now = Instant::now();
    for i in 0..n {
        debug_assert_eq!(
            values[phast.get(&keys[i]).unwrap() as usize],
            *map.get(&keys[i]).unwrap()
        );
        _ = black_box(values[unsafe { phast.get(&keys[i]).unwrap_unchecked() } as usize]);
    }
    println!(
        "Phast   lookup time {:8.3}µs",
        now.elapsed().as_nanos() as f64 / 1000.0
    );
}

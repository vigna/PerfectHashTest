use core::hint::black_box;
use std::{collections::HashMap, time::Instant};

use clap::Parser;
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

    for _ in 0..n {
        map.insert(r.next_u64(), r.next_u64());
    }

    let mut keys = map.keys().copied().collect::<Vec<_>>();
    keys.shuffle(&mut r);
    let values = map.values().copied().collect::<Vec<_>>();

    let mphf = <PtrHash>::new(keys.as_slice(), PtrHashParams::default());

    let now = Instant::now();

    for i in 0..n {
        _ = black_box(values[mphf.index(&keys[i])]);
    }
    println!("PtrHash lookup time {}ns", now.elapsed().as_nanos());

    let now = Instant::now();
    for i in 0..n {
        let _: _ = black_box(*unsafe { map.get(&keys[i]).unwrap_unchecked() });
    }
    println!("HashMap lookup time {}ns", now.elapsed().as_nanos());
}

use core::hint::black_box;
use std::time::Instant;

use clap::Parser;
use ph::phast;
use sux::bits::BitVec;
#[cfg(feature = "checks")]
use sux::traits::BitVecOpsMut;

#[derive(Parser, Debug)]
#[command(about = "Build contributor and origin collaboration graphs")]
struct Args {
    n: usize,
}

fn main() {
    let args = Args::parse();

    let n = args.n;
    let keys: Vec<_> = (0..n).collect();
    let now = Instant::now();
    let phast = phast::Function2::from_slice_mt(&keys);
    println!(
        "Phast construction time {:8.3}ns/key",
        now.elapsed().as_nanos() as f64 / n as f64
    );

    let mut _bv = BitVec::new(n);

    let now = Instant::now();
    for i in 0..n {
        let _x = black_box(phast.get(&i));
        #[cfg(feature = "checks")]
        {
            if _bv[_x] {
                panic!("Duplicate key found");
            }
            _bv.set(_x, true);
        }
    }
    println!(
        "Phast   lookup time {:8.3}ns/key",
        now.elapsed().as_nanos() as f64 / n as f64
    );
}

#![feature(test)]

extern crate test;

use columnation::*;
use test::Bencher;

#[bench] fn empty_copy(bencher: &mut Bencher) { _bench_copy(bencher, vec![(); 1024]); }
#[bench] fn u64_copy(bencher: &mut Bencher) { _bench_copy(bencher, vec![0u64; 1024]); }
#[bench] fn u32x2_copy(bencher: &mut Bencher) { _bench_copy(bencher, vec![(0u32,0u32); 1024]); }
#[bench] fn u8_u64_copy(bencher: &mut Bencher) { _bench_copy(bencher, vec![(0u8, 0u64); 512]); }
#[bench] fn string10_copy(bencher: &mut Bencher) { _bench_copy(bencher, vec![format!("grawwwwrr!"); 1024]); }
#[bench] fn string20_copy(bencher: &mut Bencher) { _bench_copy(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]); }
#[bench] fn vec_u_s_copy(bencher: &mut Bencher) { _bench_copy(bencher, vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }
#[bench] fn vec_u_vn_s_copy(bencher: &mut Bencher) { _bench_copy(bencher, vec![vec![(0u64, vec![(); 1 << 40], format!("grawwwwrr!")); 32]; 32]); }

#[bench] fn empty_clone(bencher: &mut Bencher) { _bench_clone(bencher, vec![(); 1024]); }
#[bench] fn u64_clone(bencher: &mut Bencher) { _bench_clone(bencher, vec![0u64; 1024]); }
#[bench] fn u32x2_clone(bencher: &mut Bencher) { _bench_clone(bencher, vec![(0u32,0u32); 1024]); }
#[bench] fn u8_u64_clone(bencher: &mut Bencher) { _bench_clone(bencher, vec![(0u8, 0u64); 512]); }
#[bench] fn string10_clone(bencher: &mut Bencher) { _bench_clone(bencher, vec![format!("grawwwwrr!"); 1024]); }
#[bench] fn string20_clone(bencher: &mut Bencher) { _bench_clone(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]); }
#[bench] fn vec_u_s_clone(bencher: &mut Bencher) { _bench_clone(bencher, vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }
#[bench] fn vec_u_vn_s_clone(bencher: &mut Bencher) { _bench_clone(bencher, vec![vec![(0u64, vec![(); 1 << 40], format!("grawwwwrr!")); 32]; 32]); }

#[bench] fn empty_realloc(bencher: &mut Bencher) { _bench_realloc(bencher, vec![(); 1024]); }
#[bench] fn u64_realloc(bencher: &mut Bencher) { _bench_realloc(bencher, vec![0u64; 1024]); }
#[bench] fn u32x2_realloc(bencher: &mut Bencher) { _bench_realloc(bencher, vec![(0u32,0u32); 1024]); }
#[bench] fn u8_u64_realloc(bencher: &mut Bencher) { _bench_realloc(bencher, vec![(0u8, 0u64); 512]); }
#[bench] fn string10_realloc(bencher: &mut Bencher) { _bench_realloc(bencher, vec![format!("grawwwwrr!"); 1024]); }
#[bench] fn string20_realloc(bencher: &mut Bencher) { _bench_realloc(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]); }
#[bench] fn vec_u_s_realloc(bencher: &mut Bencher) { _bench_realloc(bencher, vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }
#[bench] fn vec_u_vn_s_realloc(bencher: &mut Bencher) { _bench_realloc(bencher, vec![vec![(0u64, vec![(); 1 << 40], format!("grawwwwrr!")); 32]; 32]); }

#[bench] fn empty_prealloc(bencher: &mut Bencher) { _bench_prealloc(bencher, vec![(); 1024]); }
#[bench] fn u64_prealloc(bencher: &mut Bencher) { _bench_prealloc(bencher, vec![0u64; 1024]); }
#[bench] fn u32x2_prealloc(bencher: &mut Bencher) { _bench_prealloc(bencher, vec![(0u32,0u32); 1024]); }
#[bench] fn u8_u64_prealloc(bencher: &mut Bencher) { _bench_prealloc(bencher, vec![(0u8, 0u64); 512]); }
#[bench] fn string10_prealloc(bencher: &mut Bencher) { _bench_prealloc(bencher, vec![format!("grawwwwrr!"); 1024]); }
#[bench] fn string20_prealloc(bencher: &mut Bencher) { _bench_prealloc(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]); }
#[bench] fn vec_u_s_prealloc(bencher: &mut Bencher) { _bench_prealloc(bencher, vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }
#[bench] fn vec_u_vn_s_prealloc(bencher: &mut Bencher) { _bench_prealloc(bencher, vec![vec![(0u64, vec![(); 1 << 40], format!("grawwwwrr!")); 32]; 32]); }

fn _bench_copy<T: Columnation+Eq>(bencher: &mut Bencher, record: T) {

    // prepare encoded data for bencher.bytes
    let mut arena = ColumnStack::<T>::default();

    bencher.iter(|| {
        arena.clear();
        for _ in 0 .. 1024 {
            arena.copy(&record);
        }
    });
}

fn _bench_clone<T: Columnation+Eq+Clone>(bencher: &mut Bencher, record: T) {

    // prepare encoded data for bencher.bytes
    let mut arena = Vec::new();

    bencher.iter(|| {
        arena.clear();
        for _ in 0 .. 1024 {
            arena.push(record.clone());
        }
    });
}

fn _bench_realloc<T: Columnation+Eq>(bencher: &mut Bencher, record: T) {

    bencher.iter(|| {
        // prepare encoded data for bencher.bytes
        let mut arena = ColumnStack::<T>::default();
        for _ in 0 .. 1024 {
            arena.copy(&record);
        }
    });
}

fn _bench_prealloc<T: Columnation+Eq>(bencher: &mut Bencher, record: T) {

    bencher.iter(|| {
        // prepare encoded data for bencher.bytes
        let mut arena = ColumnStack::<T>::default();
        arena.with_capacity_for(std::iter::repeat(&record).take(1024));
        for _ in 0 .. 1024 {
            arena.copy(&record);
        }
    });
}

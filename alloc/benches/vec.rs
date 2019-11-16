use std::iter::{repeat, FromIterator};
use test::{black_box, Bencher};

#[bench]
fn bench_new(b: &mut Bencher) {
    b.iter(|| {
        let v: Vec<u32> = Vec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);
    })
}

fn do_bench_with_capacity(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(|| {
        let v: Vec<u32> = Vec::with_capacity(src_len);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), src_len);
    })
}

#[bench]
fn bench_with_capacity_0000(b: &mut Bencher) {
    do_bench_with_capacity(b, 0)
}

#[bench]
fn bench_with_capacity_0010(b: &mut Bencher) {
    do_bench_with_capacity(b, 10)
}

#[bench]
fn bench_with_capacity_0100(b: &mut Bencher) {
    do_bench_with_capacity(b, 100)
}

#[bench]
fn bench_with_capacity_1000(b: &mut Bencher) {
    do_bench_with_capacity(b, 1000)
}

fn do_bench_from_fn(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(|| {
        let dst = (0..src_len).collect::<Vec<_>>();
        assert_eq!(dst.len(), src_len);
        assert!(dst.iter().enumerate().all(|(i, x)| i == *x));
    })
}

#[bench]
fn bench_from_fn_0000(b: &mut Bencher) {
    do_bench_from_fn(b, 0)
}

#[bench]
fn bench_from_fn_0010(b: &mut Bencher) {
    do_bench_from_fn(b, 10)
}

#[bench]
fn bench_from_fn_0100(b: &mut Bencher) {
    do_bench_from_fn(b, 100)
}

#[bench]
fn bench_from_fn_1000(b: &mut Bencher) {
    do_bench_from_fn(b, 1000)
}

fn do_bench_from_elem(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(|| {
        let dst: Vec<usize> = repeat(5).take(src_len).collect();
        assert_eq!(dst.len(), src_len);
        assert!(dst.iter().all(|x| *x == 5));
    })
}

#[bench]
fn bench_from_elem_0000(b: &mut Bencher) {
    do_bench_from_elem(b, 0)
}

#[bench]
fn bench_from_elem_0010(b: &mut Bencher) {
    do_bench_from_elem(b, 10)
}

#[bench]
fn bench_from_elem_0100(b: &mut Bencher) {
    do_bench_from_elem(b, 100)
}

#[bench]
fn bench_from_elem_1000(b: &mut Bencher) {
    do_bench_from_elem(b, 1000)
}

fn do_bench_from_slice(b: &mut Bencher, src_len: usize) {
    let src: Vec<_> = FromIterator::from_iter(0..src_len);

    b.bytes = src_len as u64;

    b.iter(|| {
        let dst = src.clone()[..].to_vec();
        assert_eq!(dst.len(), src_len);
        assert!(dst.iter().enumerate().all(|(i, x)| i == *x));
    });
}

#[bench]
fn bench_from_slice_0000(b: &mut Bencher) {
    do_bench_from_slice(b, 0)
}

#[bench]
fn bench_from_slice_0010(b: &mut Bencher) {
    do_bench_from_slice(b, 10)
}

#[bench]
fn bench_from_slice_0100(b: &mut Bencher) {
    do_bench_from_slice(b, 100)
}

#[bench]
fn bench_from_slice_1000(b: &mut Bencher) {
    do_bench_from_slice(b, 1000)
}

fn do_bench_from_iter(b: &mut Bencher, src_len: usize) {
    let src: Vec<_> = FromIterator::from_iter(0..src_len);

    b.bytes = src_len as u64;

    b.iter(|| {
        let dst: Vec<_> = FromIterator::from_iter(src.clone());
        assert_eq!(dst.len(), src_len);
        assert!(dst.iter().enumerate().all(|(i, x)| i == *x));
    });
}

#[bench]
fn bench_from_iter_0000(b: &mut Bencher) {
    do_bench_from_iter(b, 0)
}

#[bench]
fn bench_from_iter_0010(b: &mut Bencher) {
    do_bench_from_iter(b, 10)
}

#[bench]
fn bench_from_iter_0100(b: &mut Bencher) {
    do_bench_from_iter(b, 100)
}

#[bench]
fn bench_from_iter_1000(b: &mut Bencher) {
    do_bench_from_iter(b, 1000)
}

fn do_bench_extend(b: &mut Bencher, dst_len: usize, src_len: usize) {
    let dst: Vec<_> = FromIterator::from_iter(0..dst_len);
    let src: Vec<_> = FromIterator::from_iter(dst_len..dst_len + src_len);

    b.bytes = src_len as u64;

    b.iter(|| {
        let mut dst = dst.clone();
        dst.extend(src.clone());
        assert_eq!(dst.len(), dst_len + src_len);
        assert!(dst.iter().enumerate().all(|(i, x)| i == *x));
    });
}

#[bench]
fn bench_extend_0000_0000(b: &mut Bencher) {
    do_bench_extend(b, 0, 0)
}

#[bench]
fn bench_extend_0000_0010(b: &mut Bencher) {
    do_bench_extend(b, 0, 10)
}

#[bench]
fn bench_extend_0000_0100(b: &mut Bencher) {
    do_bench_extend(b, 0, 100)
}

#[bench]
fn bench_extend_0000_1000(b: &mut Bencher) {
    do_bench_extend(b, 0, 1000)
}

#[bench]
fn bench_extend_0010_0010(b: &mut Bencher) {
    do_bench_extend(b, 10, 10)
}

#[bench]
fn bench_extend_0100_0100(b: &mut Bencher) {
    do_bench_extend(b, 100, 100)
}

#[bench]
fn bench_extend_1000_1000(b: &mut Bencher) {
    do_bench_extend(b, 1000, 1000)
}

fn do_bench_extend_from_slice(b: &mut Bencher, dst_len: usize, src_len: usize) {
    let dst: Vec<_> = FromIterator::from_iter(0..dst_len);
    let src: Vec<_> = FromIterator::from_iter(dst_len..dst_len + src_len);

    b.bytes = src_len as u64;

    b.iter(|| {
        let mut dst = dst.clone();
        dst.extend_from_slice(&src);
        assert_eq!(dst.len(), dst_len + src_len);
        assert!(dst.iter().enumerate().all(|(i, x)| i == *x));
    });
}

#[bench]
fn bench_extend_from_slice_0000_0000(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 0, 0)
}

#[bench]
fn bench_extend_from_slice_0000_0010(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 0, 10)
}

#[bench]
fn bench_extend_from_slice_0000_0100(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 0, 100)
}

#[bench]
fn bench_extend_from_slice_0000_1000(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 0, 1000)
}

#[bench]
fn bench_extend_from_slice_0010_0010(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 10, 10)
}

#[bench]
fn bench_extend_from_slice_0100_0100(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 100, 100)
}

#[bench]
fn bench_extend_from_slice_1000_1000(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 1000, 1000)
}

fn do_bench_clone(b: &mut Bencher, src_len: usize) {
    let src: Vec<usize> = FromIterator::from_iter(0..src_len);

    b.bytes = src_len as u64;

    b.iter(|| {
        let dst = src.clone();
        assert_eq!(dst.len(), src_len);
        assert!(dst.iter().enumerate().all(|(i, x)| i == *x));
    });
}

#[bench]
fn bench_clone_0000(b: &mut Bencher) {
    do_bench_clone(b, 0)
}

#[bench]
fn bench_clone_0010(b: &mut Bencher) {
    do_bench_clone(b, 10)
}

#[bench]
fn bench_clone_0100(b: &mut Bencher) {
    do_bench_clone(b, 100)
}

#[bench]
fn bench_clone_1000(b: &mut Bencher) {
    do_bench_clone(b, 1000)
}

fn do_bench_clone_from(b: &mut Bencher, times: usize, dst_len: usize, src_len: usize) {
    let dst: Vec<_> = FromIterator::from_iter(0..src_len);
    let src: Vec<_> = FromIterator::from_iter(dst_len..dst_len + src_len);

    b.bytes = (times * src_len) as u64;

    b.iter(|| {
        let mut dst = dst.clone();

        for _ in 0..times {
            dst.clone_from(&src);

            assert_eq!(dst.len(), src_len);
            assert!(dst.iter().enumerate().all(|(i, x)| dst_len + i == *x));
        }
    });
}

#[bench]
fn bench_clone_from_01_0000_0000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 0, 0)
}

#[bench]
fn bench_clone_from_01_0000_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 0, 10)
}

#[bench]
fn bench_clone_from_01_0000_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 0, 100)
}

#[bench]
fn bench_clone_from_01_0000_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 0, 1000)
}

#[bench]
fn bench_clone_from_01_0010_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 10, 10)
}

#[bench]
fn bench_clone_from_01_0100_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 100, 100)
}

#[bench]
fn bench_clone_from_01_1000_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 1000, 1000)
}

#[bench]
fn bench_clone_from_01_0010_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 10, 100)
}

#[bench]
fn bench_clone_from_01_0100_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 100, 1000)
}

#[bench]
fn bench_clone_from_01_0010_0000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 10, 0)
}

#[bench]
fn bench_clone_from_01_0100_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 100, 10)
}

#[bench]
fn bench_clone_from_01_1000_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 1000, 100)
}

#[bench]
fn bench_clone_from_10_0000_0000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 0, 0)
}

#[bench]
fn bench_clone_from_10_0000_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 0, 10)
}

#[bench]
fn bench_clone_from_10_0000_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 0, 100)
}

#[bench]
fn bench_clone_from_10_0000_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 0, 1000)
}

#[bench]
fn bench_clone_from_10_0010_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 10, 10)
}

#[bench]
fn bench_clone_from_10_0100_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 100, 100)
}

#[bench]
fn bench_clone_from_10_1000_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 1000, 1000)
}

#[bench]
fn bench_clone_from_10_0010_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 10, 100)
}

#[bench]
fn bench_clone_from_10_0100_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 100, 1000)
}

#[bench]
fn bench_clone_from_10_0010_0000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 10, 0)
}

#[bench]
fn bench_clone_from_10_0100_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 100, 10)
}

#[bench]
fn bench_clone_from_10_1000_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 1000, 100)
}

macro_rules! bench_in_place {
    (
        $($fname:ident, $type:ty , $count:expr, $init: expr);*
    ) => {
        $(
            #[bench]
            fn $fname(b: &mut Bencher) {
                b.iter(|| {
                    let src: Vec<$type> = black_box(vec![$init; $count]);
                    let mut sink = src.into_iter()
                        .enumerate()
                        .map(|(idx, e)| { (idx as $type) ^ e }).collect::<Vec<$type>>();
                    black_box(sink.as_mut_ptr())
                });
            }
        )+
    };
}

bench_in_place![
    bench_in_place_xxu8_i0_0010,     u8,     10, 0;
    bench_in_place_xxu8_i0_0100,     u8,    100, 0;
    bench_in_place_xxu8_i0_1000,     u8,   1000, 0;
    bench_in_place_xxu8_i1_0010,     u8,     10, 1;
    bench_in_place_xxu8_i1_0100,     u8,    100, 1;
    bench_in_place_xxu8_i1_1000,     u8,   1000, 1;
    bench_in_place_xu32_i0_0010,    u32,     10, 0;
    bench_in_place_xu32_i0_0100,    u32,    100, 0;
    bench_in_place_xu32_i0_1000,    u32,   1000, 0;
    bench_in_place_xu32_i1_0010,    u32,     10, 1;
    bench_in_place_xu32_i1_0100,    u32,    100, 1;
    bench_in_place_xu32_i1_1000,    u32,   1000, 1;
    bench_in_place_u128_i0_0010,   u128,     10, 0;
    bench_in_place_u128_i0_0100,   u128,    100, 0;
    bench_in_place_u128_i0_1000,   u128,   1000, 0;
    bench_in_place_u128_i1_0010,   u128,     10, 1;
    bench_in_place_u128_i1_0100,   u128,    100, 1;
    bench_in_place_u128_i1_1000,   u128,   1000, 1
];

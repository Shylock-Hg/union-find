#![feature(test)]
#![feature(uniform_paths)]

#[cfg(test)] 
mod bench_tests {

    extern crate test;
    use test::Bencher;
    extern crate rand;
    use rand::Rng;

    use union_find;

    #[bench]
    fn large_bench_find(b: &mut Bencher) {
        const COUNT: usize = 1024*1024*32;
        // const COUNT: usize = 1024*1024;
        // const COUNT: usize = 1024;
        use union_find::UnionFind;
        let mut u: union_find::UnionWeighted<i32> =
            union_find::UnionWeighted::new_full_category(COUNT);

        // union all categories
        for i in 0..COUNT-1 {  // union all categories
                u.union(i, i+1);
        }

        let mut rng = rand::thread_rng();
        let r1: usize = rng.gen_range(0 as usize, COUNT);
        let r2: usize = rng.gen_range(0 as usize, COUNT);

        b.iter(|| {
                u.find(r1, r2);
        });
    }

/*  // not very property
    #[bench]
    fn large_bench_union(b: &mut Bencher) {
        const COUNT: usize = 1024*1024*32;
        // const COUNT: usize = 1024*1024;
        // const COUNT: usize = 1024;
        use union_find::UnionFind;
        let mut u: union_find::UnionWeighted<i32> =
            union_find::UnionWeighted::new_full_category(COUNT);

        // union all categories except last
        for i in 0..COUNT-2 {  // union all categories
            u.union(i, i+1);
        }

        // bench
        b.iter(|| {
            // \NOTE this will perform diffrence
            //       when input unioned and ununioned item
            u.union(COUNT - 1, COUNT - 2);
        });
    }
*/
}

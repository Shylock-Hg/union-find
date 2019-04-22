#[cfg(test)]
mod simple_tests {

    use union_find;
    use union_find::UnionFind; // must specify

    #[test]
    fn basic() {
        const COUNT: usize = 4;
        let mut u: union_find::UnionWeighted<i32> =
            union_find::UnionWeighted::new_full_category(4);

        // check initializing
        assert_eq!(u.capacity(), COUNT);
        assert_eq!(u.len(), COUNT);
        for i in 0..COUNT {
            assert_eq!(i, u.get_root(i));
        }
        for i in 0..COUNT {
            assert_eq!((i, 0), u.get_root_with_depth(i));
        }
        for i in 0..COUNT - 1 {
            assert_eq!(false, u.find(i, i + 1));
        }
        // visualize the initialized union
        println!("The initialized union is:\n {:?}\n", u);

        // check union
        u.union(2, 3);
        assert!(u.find(2, 3));
        u.union(1, 2);
        assert!(u.find(1, 3));
        // visualize the result of union(2, 3, 1)
        println!("The result of union(2, 3, 1) is:\n {:?}\n", u);

        // check find with different union input
        assert_eq!(false, u.find(0, 3));
        u.union(0, 3);
        assert!(u.find(0, 2));
        // visualize the result of union(2, 3, 1, 0)
        println!("The result of union(2, 3, 1, 0) is:\n {:?}\n", u);

        // check repeat union
        u.union(0, 1);
        u.union(1, 2);
        u.union(2, 3);
        assert!(u.find(0, 1));
        assert!(u.find(1, 2));
        assert!(u.find(2, 3));
        // visualize the union structure information
        println!("The result of repeat union is:\n {:?}\n", u);

        // union all categories
        for i in 0..COUNT - 1 {
            // union all categories
            u.union(i, i + 1);
        }
        for i in 0..COUNT - 1 {
            // check
            assert_eq!(true, u.find(i, i + 1));
        }
        for i in 0..COUNT {
            // flat check
            let (_, depth) = u.get_root_with_depth(i);
            assert!(depth < 3);
        }
        // visulize the the all unioned result
        println!("The result of all unioned is:\n {:?}\n", u);

        // testing push
        u.push(3);
        assert_eq!(4, u.get_root(4));
        assert_eq!((4, 0), u.get_root_with_depth(4));
        assert_eq!(false, u.find(3, 4));

        u.union(1, 4);
        assert_eq!(true, u.find(3, 4));

        // testing insert
        u.insert(2, 5);
        assert_eq!(true, u.find(2, 5));
        assert_eq!(true, u.find(0, 5));

        u.insert(5, 6);
        assert_eq!(true, u.find(1, 6));
        assert_eq!(true, u.find(2, 6));

        // visualize the result of insert new item
        println!("The simple union test final result is:\n {:?}\n", u);
    }

    #[test]
    fn large() {
        const COUNT: usize = 1024 * 1024 * 32;
        // const COUNT: usize = 1024*1024;
        // const COUNT: usize = 1024;
        use union_find::UnionFind;
        let mut u: union_find::UnionWeighted<i32> =
            union_find::UnionWeighted::new_full_category(COUNT);
        // initiliaze check
        assert_eq!(u.capacity(), COUNT);
        assert_eq!(u.len(), COUNT);
        for i in 0..COUNT {
            assert_eq!(i, u.get_root(i));
            assert_eq!((i, 0), u.get_root_with_depth(i));
        }
        for i in 0..COUNT - 1 {
            assert_eq!(false, u.find(i, i + 1));
        }
        for i in 0..COUNT {
            assert_eq!(true, u.find(i, i));
        }

        // union all categories
        for i in 0..COUNT - 1 {
            // union all categories
            u.union(i, i + 1);
        }
        for i in 0..COUNT - 1 {
            // check
            assert_eq!(true, u.find(i, i + 1));
        }
        for i in 0..COUNT {
            // flat check
            let (_, depth) = u.get_root_with_depth(i);
            assert!(depth < 3);
        }

        // back union again
        for i in COUNT - 1..0 {
            u.union(i, i + 1);
        }
        for i in 0..COUNT - 1 {
            // check
            assert_eq!(true, u.find(i, i + 1));
        }
        for i in 0..COUNT {
            // flat check
            let (_, depth) = u.get_root_with_depth(i);
            assert!(depth < 3);
        }
    }
}

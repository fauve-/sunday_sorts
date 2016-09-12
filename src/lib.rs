#![feature(test)]
extern crate rand;
extern crate test;

// bog standard quicksort with d's partitioning and a few modifications for rust's region checking
mod qs {
    fn partition<T: PartialOrd>(v: &mut [T], lo: usize, hi: usize) -> usize {
        let mut i = lo;
        let mut j = hi;
        let mut piv = lo + (hi - lo) / 2;
        loop {
            while v[i] < v[piv] {
                i += 1;
            }
            while v[j] > v[piv] {
                j -= 1;
            }
            if i >= j {
                return j;
            }

            // follow the value of piv
            if i == piv {
                piv = j;
            } else if j == piv {
                piv = i;
            }
            v.swap(i, j);
        }
    }


    fn qs<T: PartialOrd>(v: &mut [T], lo: usize, hi: usize) {
        if lo < hi {
            let p = partition(v, lo, hi);
            qs(v, lo, p);
            qs(v, p + 1, hi);
        }
    }

    pub fn quicksort<T: PartialOrd>(v: &mut [T]) {
        let max = v.len() - 1;
        qs(v, 0, max);
    }
}

mod selection {
    pub fn selection<T: PartialOrd>(v: &mut [T]) {
        for i in 0..(v.len() - 1) {
            let mut j_min = i;
            for j in i..(v.len()) {
                if v[j] < v[j_min] {
                    j_min = j;
                }
            }
            if j_min != i {
                v.swap(i, j_min);
            }
        }
    }
}

mod gnome {
    pub fn gnome<T: PartialOrd>(v: &mut [T]) {
        let mut cur = 0;
        loop {
            if cur >= v.len() {
                return;
            } else if cur == 0 || v[cur] >= v[cur - 1] {
                cur += 1;
            } else {
                v.swap(cur, cur - 1);
                cur -= 1;
            }
        }
    }
}

mod insertion {
    pub fn insertion<T: PartialOrd>(v: &mut [T]) {
        // swaps for the swap god
        for i in 1..v.len() {
            let mut j = i;
            while j > 0 && v[j - 1] > v[j] {
                v.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}

mod bubble {
    pub fn bubble<T: PartialOrd>(v: &mut [T]) {
        let mut n = v.len();
        loop {
            let mut swapped = false;
            for i in 1..n {
                if v[i - 1] > v[i] {
                    v.swap(i - 1, i);
                    swapped = true;
                }
            }
            n -= 1;
            if !swapped {
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use insertion::insertion;
    use selection::selection;
    use qs::quicksort;
    use bubble::bubble;
    use gnome::gnome;
    use rand;
    use rand::Rng;
    use std::vec::Vec;
    use test::Bencher;

    macro_rules! make_test_vec{
        ($siz:expr,$t:ty) => {{
            let mut v:Vec<$t> = vec![];
            let mut rng = rand::thread_rng();
            for _ in 0..$siz{
                v.push(rng.gen::<$t>());
            }
            v
        }}
    }

    macro_rules! verify_sorted{
        ($v:expr) =>{{
            loop{
                if $v.len() == 0{
                    break;
                }
                let a = $v.pop().unwrap();
                let b = $v.pop().unwrap();
                assert!(a >= b);
            }
        }}
    }


    #[test]
    fn test_quicksort() {
        let mut v = make_test_vec!(10000, isize);
        quicksort(v.as_mut_slice());
        verify_sorted!(v);
    }



    #[test]
    fn test_bubble_random() {
        let mut v = make_test_vec!(10000, isize);
        bubble(v.as_mut_slice());
        verify_sorted!(v);
    }


    #[test]
    fn test_selection_sort() {
        let mut v = make_test_vec!(10000, isize);
        selection(v.as_mut_slice());
        verify_sorted!(v);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = make_test_vec!(10000, isize);
        insertion(v.as_mut_slice());
        verify_sorted!(v);
    }



    #[test]
    fn test_gnome_sort() {
        let mut v = make_test_vec!(10000, isize);
        gnome(v.as_mut_slice());
        verify_sorted!(v);
    }


    #[bench]
    fn bench_quicksort_random(b: &mut Bencher) {
        let mut v = make_test_vec!(10000, isize);
        b.iter(|| {
            // don't bother to validate. test will validate
            let mut v = v.clone();
            quicksort(v.as_mut_slice());
        })
    }

    #[bench]
    fn bench_selection_sort_random(b: &mut Bencher) {
        let mut v = make_test_vec!(10000, isize);
        b.iter(|| {
            // don't bother to validate. test will validate
            let mut v = v.clone();
            selection(v.as_mut_slice());
        })
    }


    #[bench]
    fn bench_insertion_random(b: &mut Bencher) {
        let mut v = make_test_vec!(10000, isize);
        b.iter(|| {
            // don't bother to validate. test will validate
            let mut v = v.clone();
            insertion(v.as_mut_slice());
        })
    }

    #[bench]
    fn bench_library_sort_random(b: &mut Bencher) {
        let mut v = make_test_vec!(10000, isize);
        b.iter(|| {
            let mut v = v.clone();
            v.as_mut_slice().sort()
        });
    }

    #[bench]
    fn bench_gnome_sort_random(b: &mut Bencher) {
        let mut v = make_test_vec!(10000, isize);
        b.iter(|| {
            let mut v = v.clone();
            gnome(v.as_mut_slice());
        });
    }


    #[bench]
    fn bench_bubble_sort_random(b: &mut Bencher) {
        let mut v = make_test_vec!(10000, isize);
        b.iter(|| {
            let mut v = v.clone();
            bubble(v.as_mut_slice());
        });
    }
}

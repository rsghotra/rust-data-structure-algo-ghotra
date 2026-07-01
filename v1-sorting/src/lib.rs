use std::fmt::Debug;

//slice implements debug, if its internal type implements Debug

pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len(){
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i+1] {
                v.swap(i, i+1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

#[cfg(test)] //tells the compiler this is a test cases do not include in the build
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 12];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 4, 6, 12]);
    }
}

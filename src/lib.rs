// Start with a function which accepts a sorted array and splits it
// into two pieces at a specified point.

use num::Num;

fn split_arr<T: Num + PartialOrd>(arr: Vec<T>, at: T) -> (Vec<T>, Vec<T>) {
    let mut left = vec![];
    let mut right = vec![];

    for element in arr {
        if element < at {
            left.push(element);
        } else {
            right.push(element);
        }
    }
    (left, right)
}

pub fn sorted_squares<T: Num + PartialOrd + Clone>(arr: Vec<T>) -> Vec<T> {
    let (left, right) = split_arr(arr, num::zero());

    let square = |v: Vec<T>| v.into_iter().map(|x| x.clone() * x).collect();

    let mut left: Vec<_> = square(left);
    let mut right: Vec<_> = square(right);

    if left.is_empty() {
        return right;
    }

    right.reverse(); // descending order like left

    let mut result = vec![];
    loop {
        let l = left.last();
        let r = right.last();
        if let (Some(l), Some(r)) = (l, r) {
            if l < r {
                result.push(left.pop().unwrap());
            } else {
                result.push(right.pop().unwrap());
            }
        } else if r.is_some() {
            result.push(right.pop().unwrap());
        } else if l.is_some() {
            result.push(left.pop().unwrap());
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial() {
        assert_eq!(
            (vec![1, 2], vec![3, 4, 5]),
            split_arr(vec![1, 2, 3, 4, 5], 3)
        );
        assert_eq!(vec![1, 4, 9, 16, 25], sorted_squares(vec![1, 2, 3, 4, 5]))
    }

    #[test]
    fn negative() {
        assert_eq!(
            (vec![-2, -1], vec![3, 4, 5]),
            split_arr(vec![-2, -1, 3, 4, 5], 3)
        );
        assert_eq!(vec![1, 4, 9, 16, 25], sorted_squares(vec![-2, -1, 3, 4, 5]));
        assert_eq!(
            vec![1, 9, 16, 25, 1000000],
            sorted_squares(vec![-1000, -1, 3, 4, 5])
        );
        assert_eq!(
            vec![1, 4, 9, 16, 25, 1000000],
            sorted_squares(vec![-2, -1, 3, 4, 5, 1000])
        )
    }

    #[test]
    fn empty() {
        assert_eq!((vec![], vec![]), split_arr(vec![], 3))
    }

    #[test]
    fn unsorted() {
        assert_eq!((vec![1], vec![4, 3, 5]), split_arr(vec![4, 3, 5, 1], 3))
    }
}

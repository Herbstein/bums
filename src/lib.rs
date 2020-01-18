use std::iter;
use std::ops::Add;

// const D: u64 = (std::u64::MAX / 2) - 1;
const D: u64 = 2;

pub struct Num {
    sign: bool,
    num: Vec<u64>,
}

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        let mut left = self.num;
        let mut right = rhs.num;

        if left.len() < right.len() {
            let diff = right.len() - left.len();
            left.extend(iter::repeat(0).take(diff));
        } else if left.len() > right.len() {
            let diff = left.len() - right.len();
            right.extend(iter::repeat(0).take(diff));
        }

        assert_eq!(left.len(), right.len());

        let mut new_num = Vec::with_capacity(left.len());
        let mut carry = 0;

        for (left, right) in left.into_iter().zip(right) {
            let mut sum = left + right + carry;
            carry = 0;

            while sum >= D {
                carry = 1;
                sum %= D;
            }

            new_num.push(sum);
        }

        if carry > 0 {
            new_num.push(carry);
        }

        let new_sign = self.sign != rhs.sign;

        Num {
            sign: new_sign,
            num: new_num,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Num;

    #[test]
    fn it_works() {
        let one = Num {
            sign: false,
            num: vec![1],
        };

        let two = Num {
            sign: false,
            num: vec![1, 1, 1, 1, 1],
        };

        let added = one + two;

        assert_eq!(added.sign, false);
        assert_eq!(added.num, vec![0, 0, 0, 0, 0, 1]);
        //println!("{:?}", added.num);
    }
}

use num_traits::{NumOps, Num, PrimInt, Unsigned};

fn gcd<T: NumOps + Num + PrimInt + Unsigned>(a: T, b: T) -> T {
    if b == T::zero() {
        return a
    }

    return gcd(b, a % b)
}


pub fn lcm<T: NumOps + Num + PrimInt + Unsigned>(a: T, b: T) -> T {
    if (a > b) {
        return (a / gcd(a, b)) * b
    } else {
        return (b / gcd(a, b)) * a
    }
}

pub fn are_coprime<T: NumOps + Num + PrimInt + Unsigned>(a: T, b: T) -> bool {
    let mut a_clone = a.clone();
    let mut b_clone = b.clone();

    if ((a_clone | b_clone) & T::one()) == T::zero() {
        return false;
    }

    while a_clone & T::one() == T::zero() {
        a_clone = a_clone >> 1;
    }
    if (a_clone == T::zero()) {
        return true;
    }

    loop {
        if b_clone == T::zero() {
            break;
        }

        while ((b_clone & T::one()) == T::zero()) {
            b_clone = b_clone >> 1;
        }
        if (b_clone == T::one()) {
            return true;
        }

        if (a_clone > b_clone) {
            let mut t = b_clone.clone();
            b_clone = a_clone;
            a_clone = t;
        }

        b_clone = b_clone - a_clone;
    }


    false

}


pub fn mod_inverse(a: u128, m: u128) -> u128 {
    for x in 1..m {
        if (((a % m) * (x % m)) % m == 1) {
            return x;
        }
    }

    return 0
}

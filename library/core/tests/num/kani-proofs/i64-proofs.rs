#[cfg(kani)]
mod verification {
    // use super::*;

    #[kani::proof]
    fn verify_i64_unchecked_add() {
        let num1: i64 = kani::any::<i64>();
        let num2: i64 = kani::any::<i64>();

        // Safety preconditions:
        // - Positive number addition won't overflow
        // - Negative number addition won't underflow
        // Addition of two integers with different signs never overflows
        // Undefined behavior occurs when overflow or underflow happens
        kani::assume(
            (num1 > 0 && num2 > 0 && num1 < i64::MAX - num2)
                || (num1 < 0 && num2 < 0 && num1 > i64::MIN - num2),
        );

        unsafe {
            let result = num1.unchecked_add(num2);
            assert_eq!(Some(result), num1.checked_add(num2));
        }
    }

    #[kani::proof]
    fn verify_i64_unchecked_sub() {
        // TODO
    }

    #[kani::proof]
    fn verify_i64_unchecked_mul() {
        // TODO
    }

    #[kani::proof]
    fn verify_i64_unchecked_shl() {
        // TODO
    }

    #[kani::proof]
    fn verify_i64_unchecked_shr() {
        // TODO
    }

    #[kani::proof]
    fn verify_i64_unchecked_neg() {
        // TODO
    }
}

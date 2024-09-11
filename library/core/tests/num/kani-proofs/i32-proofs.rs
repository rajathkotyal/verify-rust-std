#![feature(unchecked_shifts)]

#[cfg(kani)]
mod verification {
    // use super::*;

    #[kani::proof]
    fn verify_i32_unchecked_add() {
        let num1: i32 = kani::any::<i32>();
        let num2: i32 = kani::any::<i32>();

        // Safety preconditions:
        // - Positive number addition won't overflow
        // - Negative number addition won't underflow
        // Addition of two integers with different signs never overflows
        // Undefined behavior occurs when overflow or underflow happens
        kani::assume(
            (num1 > 0 && num2 > 0 && num1 < i32::MAX - num2)
                || (num1 < 0 && num2 < 0 && num1 > i32::MIN - num2),
        );

        unsafe {
            let result = num1.unchecked_add(num2);
            assert_eq!(Some(result), num1.checked_add(num2));
        }
    }

    #[kani::proof]
    fn verify_i32_unchecked_sub() {
        // TODO
    }

    #[kani::proof]
    fn verify_i32_unchecked_mul() {
        let num: i32 = kani::any::<i32>(); // Any value in type i32
        let shift_amount: u32 = kani::any::<u32>(); // Any shift amount in type u32
    
        // Assume the shift value is smaller than 32 because i32 only has 32 bits
        kani::assume(shift_amount < 32);
    
        unsafe {
            let result = num.unchecked_shl(shift_amount);
            assert_eq!(Some(result), num.checked_shl(shift_amount));
        }
    }

    #[kani::proof]
    fn verify_i32_unchecked_shl() {
        // TODO
    }

    #[kani::proof]
    fn verify_i32_unchecked_shr() {
        // TODO
    }

    #[kani::proof]
    fn verify_i32_unchecked_neg() {
        // TODO
    }
}

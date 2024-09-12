#[cfg(kani)]
mod verification {
    // use super::*;

    #[kani::proof]
    fn verify_i8_unchecked_add() {
        let num1: i8 = kani::any::<i8>();
        let num2: i8 = kani::any::<i8>();

        // Safety preconditions:
        // - Positive number addition won't overflow
        // - Negative number addition won't underflow
        // Addition of two integers with different signs never overflows
        // Undefined behavior occurs when overflow or underflow happens
        kani::assume(
            (num1 > 0 && num2 > 0 && num1 < i8::MAX - num2)
                || (num1 < 0 && num2 < 0 && num1 > i8::MIN - num2),
        );

        unsafe {
            let result = num1.unchecked_add(num2);
            assert_eq!(Some(result), num1.checked_add(num2));
        }
    }

    #[kani::proof]
    fn verify_i8_unchecked_sub() {
        let num1: i8 = kani::any::<i8>();
        let num2: i8 = kani::any::<i8>();
    
        // Safety preconditions:
        // - Subtraction won't underflow when num1 is positive and num2 is negative
        // - Subtraction won't overflow when num1 is negative and num2 is positive
        // Undefined behavior occurs when overflow or underflow happens
        kani::assume(
            (num1 >= 0 && num2 <= 0 && num1 <= i8::MAX + num2)
                || (num1 <= 0 && num2 >= 0 && num1 >= i8::MIN + num2),
        );
    
        unsafe {
            let result = num1.unchecked_sub(num2);
            assert_eq!(Some(result), num1.checked_sub(num2));
        }   
    }

    #[kani::proof]
    fn verify_i8_unchecked_mul() {
        // TODO
    }

    #[kani::proof]
    fn verify_i8_unchecked_shl() {
        // TODO
    }

    #[kani::proof]
    fn verify_i8_unchecked_shr() {
        // TODO
    }

    #[kani::proof]
    fn verify_i8_unchecked_neg() {
        // TODO
    }
}

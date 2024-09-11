#[cfg(kani)]
mod verification {
    // use super::*;

    #[kani::proof]
    fn verify_u32_unchecked_add() {
        let num1: u32 = kani::any::<u32>();
        let num2: u32 = kani::any::<u32>();

        // Safety preconditions:
        // - Addition won't overflow
        // Unsigned integers are always positive, so underflow won't happen
        // Undefined behavior occurs when overflow happens
        kani::assume(num1 < u32::MAX - num2);

        unsafe {
            let result = num1.unchecked_add(num2);
            assert_eq!(Some(result), num1.checked_add(num2));
        }
    }

    #[kani::proof]
    fn verify_u32_unchecked_sub() {
        // TODO
    }

    #[kani::proof]
    fn verify_u32_unchecked_mul() {
        // TODO
    }

    #[kani::proof]
    fn verify_u32_unchecked_shl() {
        // TODO
    }

    #[kani::proof]
    fn verify_u32_unchecked_shr() {
        // TODO
    }

    #[kani::proof]
    fn verify_u32_unchecked_neg() {
        // TODO
    }
}

#[cfg(kani)]
mod verification {
   // use super::*;

   #[kani::proof]
   fn verify_u128_unchecked_add() {
       let num1: u128 = kani::any::<u128>();
       let num2: u128 = kani::any::<u128>();

       // Safety preconditions:
       // - Addition won't overflow
       // Unsigned integers are always positive, so underflow won't happen
       // Undefined behavior occurs when overflow happens
       kani::assume(num1 < u128::MAX - num2);

        unsafe {
            let result = num1.unchecked_add(num2);
            assert_eq!(Some(result), num1.checked_add(num2));
        }
   }
}
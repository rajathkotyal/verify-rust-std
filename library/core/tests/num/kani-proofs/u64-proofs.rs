#[cfg(kani)]
mod verification {
   // use super::*;

   #[kani::proof]
   fn verify_u64_unchecked_add() {
       let num1: u64 = kani::any::<u64>();
       let num2: u64 = kani::any::<u64>();

       // Safety preconditions:
       // - Addition won't overflow
       // Unsigned integers are always positive, so underflow won't happen
       // Undefined behavior occurs when overflow happens
       kani::assume(num1 < u64::MAX - num2);

        unsafe {
            let result = num1.unchecked_add(num2);
            assert_eq!(Some(result), num1.checked_add(num2));
        }
   }
}
#[cfg(kani)]
mod verification {
   // use super::*;

   #[kani::proof]
   fn verify_u16_unchecked_add() {
       let num1: u16 = kani::any::<u16>();
       let num2: u16 = kani::any::<u16>();

       // Safety preconditions:
       // - Addition won't overflow
       // Unsigned integers are always positive, so underflow won't happen
       // Undefined behavior occurs when overflow happens
       kani::assume(num1 < u16::MAX - num2);

        unsafe {
            let result = num1.unchecked_add(num2);
            assert_eq!(Some(result), num1.checked_add(num2));
        }
   }
}
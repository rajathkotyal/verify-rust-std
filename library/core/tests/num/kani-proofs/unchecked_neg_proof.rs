#![feature(unchecked_neg)]

// ANCHOR: kani
#[cfg(kani)]
#[kani::proof]
//#[kani::unwind(100)] // deliberately too low
fn verify_unchecked_neg() {
    let x: i8 = kani::any();
    kani::assume(x != i8::MIN);

    let result: i8 = unsafe{x.unchecked_neg()};
    assert_eq!(result, -x);
}

// ANCHOR_END: kani
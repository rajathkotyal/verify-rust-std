// pp-exact

fn main() {
    let x = 1;
    let y = 2;
    let z = 3;
    let l1 = fn@[copy x]() -> int { x + y };
    let l2 = fn@[copy x; move y]() -> int { x + y };
    let l3 = fn@[move z]() -> int { z };

    let x = 1;
    let y = 2;
    let z = 3;
    let s1 = sendfn[copy x]() -> int { x + y };
    let s2 = sendfn[copy x; move y]() -> int { x + y };
    let s3 = sendfn[move z]() -> int { z };
}

#![no_std]

/// A helper for TCO
pub enum TCO<REC, RET> {
    /// Recall function
    Rec(REC),
    /// Return a value
    Ret(RET),
}

/// Construct a fn(A) -> B from a function that returns Fn(A) -> TCO<A,B>
pub fn tco<A, B>(fun: fn(A) -> TCO<A, B>) -> impl Fn(A) -> B {
    move |p: A| {
        let mut c: TCO<A, B> = TCO::Rec(p);
        loop {
            match c {
                TCO::Rec(i) => c = fun(i),
                TCO::Ret(b) => return b,
            }
        }
    }
}

/// use tail calls to compute factorial.
/// converted to optimized function with tco
/// this avoids stack overflows
#[test]
fn test() {
    // y is an accumulator for result
    let f = tco(|(x, y): (i32, f64)| {
        if (x == 0) {
            // if we have reached 0 return computed value
            TCO::Ret(y)
        } else {
            // reduce x by 1, and multiplyx value by x
            TCO::Rec((x - 1, y * (x as f64)))
        }
    });
    assert_eq!(f((3, 1.0)), 6.0);
    f((100000, 1.0));
}
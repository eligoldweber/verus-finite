use vstd::*;
use vstd::prelude::*;

verus! {



spec fn triangle(n: nat) -> nat
    decreases n
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat)
    }
}
// 


fn main() {
    let n = 5;
    
    assert(triangle(n as nat) == n*((n as nat+1nat)/2nat));
}


}
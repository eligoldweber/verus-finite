#[allow(unused_imports)]
use vstd::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use seq::*;
#[allow(unused_imports)]
use prelude::*;

verus! {

struct Vals {
    x: nat,
    y: nat,
}

struct State {
    v: Vals,
    id: int,
}

spec fn divides(factor: nat, candidate: nat) -> bool
    recommends 1 <= factor
{
    candidate % factor == 0
}

spec fn is_prime(candidate: State) -> bool {
    &&& 1 < candidate.v.x
    &&& forall|factor: nat| 1 < factor < candidate.v.x ==> !divides(factor, candidate.v.x)
}




fn main() {
    // assert(!is_prime());
    // assert(is_prime());
}

} // verus!

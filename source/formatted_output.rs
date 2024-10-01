#[allow(unused_imports)]
use prelude::*;
#[allow(unused_imports)]
use seq::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use vstd::*;
verus! {

spec fn divides(factor: nat, candidate: nat) -> bool
    recommends
        1 <= factor,
{
    candidate % factor == 0
}

spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}

spec fn is_three_prime() -> bool {
    &&& 1 < 3
    &&& !divides(2, 3)
}

spec fn is_valid(factor: nat) -> bool {
    factor == 2
}

spec fn test() -> bool {
    &&& forall|factor: nat| 1 <= factor <= 3 ==> #[trigger] is_valid(factor)
}

fn main() {
    assert(is_valid(2));
    assert(test());
}

} // verus!

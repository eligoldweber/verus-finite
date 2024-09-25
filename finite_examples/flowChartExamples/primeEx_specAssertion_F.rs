#[allow(unused_imports)]
use vstd::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use seq::*;
#[allow(unused_imports)]
use prelude::*;

verus! {

spec fn divides(factor: nat, candidate: nat) -> bool
    recommends 1 <= factor
{
    candidate % factor == 0
}

spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}





spec fn is_prime_F3() -> bool {
    &&& 1 < 3
    &&& !divides(2, 3)  
}

spec fn is_prime_F4() -> bool {
    &&& 1 < 4
    &&& !divides(2, 4)  
    &&& !divides(3, 4)

}


fn main() {

    // assert(is_prime(4));
    // assert(is_prime_F4());
    // assert(false);
    assert(2 < 1);
   
}

} // verus!

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

spec fn divides_2_3() -> bool
    recommends
        1 <= 2,
{
    3nat % 2nat == 0
}
//...

spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}

spec fn is_prime_F2() -> bool {
    &&& 1 < 2
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


fn test_prime_finite_2() -> (result: bool)
    requires
        1 < 2,
    ensures
        result == is_prime_F2(),
{
    let mut factor: u64 = 2;
    while factor < 2
    invariant
        1 < factor,
        // forall|smallerfactor: nat|
        //     1 < smallerfactor < factor ==> !divides(smallerfactor, 2 as nat), 
            {
                if 2 % factor == 0 {
                        // assert(divides(factor as nat, candidate as nat));
                        return false;
                    }
                    factor = factor + 1;
            }
            true
}

fn test_prime_finite_3() -> (result: bool)
    requires
        1 < 3,
    ensures
        result == is_prime_F2(),
{
    let mut factor: u64 = 2;
    if(factor < 3){
        assert(1 < factor);
        // assert(!divides(2, 3 as nat));
        if 3 % factor == 0 {
            // assert(divides(factor as nat, candidate as nat));
            return false;
        }
        factor = factor + 1;
        assert(1 < factor);
        assert(!divides(2, 3 as nat));
    }
    if(factor < 3){
        assert(1 < factor);
        assert(!divides(2, 3 as nat));
        if 3 % factor == 0 {
            // assert(divides(factor as nat, candidate as nat));
            return false;
        }
        factor = factor + 1;
        assert(1 < factor);
        assert(!divides(2, 3 as nat));
        assert(!divides(3, 3 as nat));
    }
    true
}




fn test_prime_finite_4() -> (result: bool)
    requires
        1 < 4,
    ensures
        result == is_prime_F4()
{
    let mut factor: u64 = 2;
    if(factor < 4){
        assert(1 < factor);
        // assert(!divides(2, 3 as nat));
        if 4 % factor == 0 {
            // assert(divides(factor as nat, candidate as nat));
            return false;
        }
        factor = factor + 1;
        assert(1 < factor);
        assert(!divides(2, 4 as nat));
    }
    if(factor < 4){
        assert(1 < factor);
        assert(!divides(2, 4 as nat));
        if 4 % factor == 0 {
            // assert(divides(factor as nat, candidate as nat));
            return false;
        }
        factor = factor + 1;
        assert(1 < factor);
        assert(!divides(2, 4 as nat));
        assert(!divides(3, 4 as nat));
    }
    if(factor < 4){
        assert(1 < factor);
        assert(!divides(2, 4 as nat));
        assert(!divides(3, 4 as nat));
        if 4 % factor == 0 {
            // assert(divides(factor as nat, candidate as nat));
            return false;
        }
        factor = factor + 1;
        assert(1 < factor);
        assert(!divides(2, 4 as nat));
        assert(!divides(3, 4 as nat));
        assert(!divides(4, 4 as nat));
    }
    true
}


fn main() {
}

} // verus!

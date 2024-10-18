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

// spec fn is_prime(candidate: nat) -> bool {
//     &&& 1 < candidate
//     &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
// }

spec fn is_prime_F0() -> bool {
    &&& 1 < 0
}


spec fn is_prime_F1() -> bool {
    &&& 1 < 1
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



spec fn is_prime_F5() -> bool {
    &&& 1 < 5
    &&& !divides(2, 5)  
    &&& !divides(3, 5)
    &&& !divides(3, 5)
}


spec fn is_prime_F6() -> bool {
    &&& 1 < 6
    &&& !divides(2, 6)  
    &&& !divides(3, 6)
    &&& !divides(3, 6)
}


// proof fn is_prime(candidate: nat) -> bool 
//     requires candidate <= 6
// {
//     if(candidate == 0)
//     {
//         return is_prime_F0();
//     }
//     if(candidate == 1)
//     {
//         return is_prime_F1();
//     }
//     if(candidate == 2)
//     {
//         return is_prime_F2();
//     }
//     if(candidate == 3)
//     {
//         return is_prime_F3();
//     }
//     if(candidate == 4)
//     {
//         return is_prime_F4();
//     }
//     if(candidate == 5)
//     {
//         return is_prime_F5();
//     }
//     if(candidate == 6)
//     {
//         return is_prime_F6();
//     }
//     false
// }

spec fn is_prime(candidate:nat) -> bool
{
    &&& (candidate == 0) ==> is_prime_F0()
    &&& (candidate == 1) ==> is_prime_F1()
    &&& (candidate == 2) ==> is_prime_F2()
    &&& (candidate == 3) ==> is_prime_F3()
    &&& (candidate == 4) ==> is_prime_F4()
    &&& (candidate == 5) ==> is_prime_F5()
    &&& (candidate == 6) ==> is_prime_F6()
    &&& (candidate > 6) ==> false

    
}

proof fn test(){
    // let v = is_prime(3);
    // assert(is_prime(3));
    assert(is_prime(5));
    // assert(is_prime(10));

}

fn main() {
   

    // assert(is_prime_F5());
    // assert(is_prime_F4()); // still fails (expexted)

    // assert(!is_prime_F8()); 
   
}

} // verus!

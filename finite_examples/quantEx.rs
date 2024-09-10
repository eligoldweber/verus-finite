#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, seq::*};

verus! {

//----------------------------------------------------------
//                   --- EX 1 ----
//----------------------------------------------------------

    spec fn is_even(i: int) -> bool {
        i % 2 == 0
    }

    // Verus can automatically find this trigger! 
    proof fn test_use_forall(s: Seq<int>)
        requires
            5 <= s.len(),
            forall|i: int| 0 <= i < s.len() ==> #[trigger] is_even(s[i]),
    {
        assert(is_even(s[3]));
    }


    proof fn test_use_forall_fail(s: Seq<int>)
        requires
            5 <= s.len(),
            forall|i: int| 0 <= i < s.len() ==> #[trigger] is_even(s[i]),
    {
        // assert(s[3] % 2 == 0); // FAILS: doesn't trigger is_even(s[i])
    }

//----------------------------------------------------------
//                   --- EX 2 ----
//----------------------------------------------------------
    spec fn nonnegative(i: int) -> bool {
        0 <= i
    }
    
    proof fn test_use_forall_bad2(s: Seq<int>)
        requires
            5 <= s.len(),
            forall|i: int| #[trigger] nonnegative(i) && i < s.len() ==> is_even(s[i]),
    {
        // assert(is_even(s[3])); // FAILS: doesn't trigger nonnegative(i)
    }


//----------------------------------------------------------
//                   --- EX 3 ----
//----------------------------------------------------------

    // proof fn test_distinct_fail1(s: Seq<int>)
    // requires
    //     5 <= s.len(),
    //     forall|i: int, j: int|
    //         0 <= i < j < s.len() ==> s[i] != #[trigger] s[j], // error: trigger fails to mention i
    // {
    //     assert(s[4] != s[2]);
    // }


//----------------------------------------------------------
//                   --- EX 4 ----
//----------------------------------------------------------

    proof fn test_exists_succeeds() {
        // assert(exists|i: int| #[trigger] is_even(i));  // succeeds with witness i = 4 or i = 6
    }


//----------------------------------------------------------
//                   --- EX 5 ----
//----------------------------------------------------------

    spec fn less_than(x: int, y: int) -> bool {
        x < y
    }

    proof fn test_choose_succeeds2() {
        assert(less_than(3, 7));  // promote i = 3, i = 7 as a witness
        let (x, y) = choose|i: int, j: int| less_than(i, j);
        assert(x < y);
    }


//----------------------------------------------------------
//                   --- EX 6 (WIP) ----
//----------------------------------------------------------

    spec fn f(i: int) -> bool;

    proof fn test_choose_fails() {
        let i_witness = choose|i: int| f(i);
        assert(i_witness < 0 || i_witness >= 0); // i_witness is some integer
        // assert(f(i_witness)); // FAILS because we don't know exists|i: int| f(i)
    }    




    fn main()

    {

    }
}
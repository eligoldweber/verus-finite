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

    proof fn test_use_forall_fail_finite(s: Seq<int>)
    requires
        5 == s.len(),
        s[0] % 2 == 0,
        s[1] % 2 == 0,
        s[2] % 2 == 0,
        s[3] % 2 == 0,
        s[4] % 2 == 0,
        // forall|i: int| 0 <= i < s.len() ==> #[trigger] is_even(s[i]),
    {
        assert(s[3] % 2 == 0); 
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


    proof fn test_use_forall_bad2_finite(s: Seq<int>)
    requires
        5 == s.len(),
        nonnegative(0) ==>  s[0] % 2 == 0,
        nonnegative(1) ==>  s[1] % 2 == 0,
        nonnegative(2) ==>  s[2] % 2 == 0,
        nonnegative(3) ==>  s[3] % 2 == 0,
        nonnegative(4) ==>  s[4] % 2 == 0,
        // forall|i: int| #[trigger] nonnegative(i) && i < s.len() ==> is_even(s[i]),
    {
        assert(is_even(s[3])); 
    }

//----------------------------------------------------------
//                   --- EX 3 ----
//----------------------------------------------------------

    // proof fn test_distinct_fail1(s: Seq<int>)
    //     requires
    //         5 <= s.len(),
    //         forall|i: int, j: int|
    //             0 <= i < j < s.len() ==> s[i] != #[trigger] s[j], // error: trigger fails to mention i
    // {
    //     // assert(s[4] != s[2]);
    // }


    proof fn test_distinct_fail1_finite(s: Seq<int>)
        requires
            5 == s.len(),
            s[0] != s[1],
            s[0] != s[2],
            s[0] != s[3],
            s[0] != s[4],
            s[1] != s[2],
            s[1] != s[3],
            s[1] != s[4],
            s[2] != s[3],
            s[2] != s[4],
            s[3] != s[4],
            // forall|i: int, j: int|
            //     0 <= i < j < s.len() ==> s[i] != #[trigger] s[j], // error: trigger fails to mention i
    {
        assert(s[4] != s[2]);
    }


//----------------------------------------------------------
//                   --- EX 4 ----
//----------------------------------------------------------

    // proof fn test_exists_fails() {
    //     assert(exists|i: int| #[trigger] is_even(i));  // succeeds with witness i = 4 or i = 6
    // }


    proof fn test_exists_finite() {
        assert(is_even(0) || is_even(1) || is_even(2));
        // assert(exists|i: int| #[trigger] is_even(i));  // succeeds with witness i = 4 or i = 6
    }
    


    spec fn less_than(x: int, y: int) -> bool {
        x < y
    }

    proof fn test_choose_succeeds2() {
        // assert(less_than(3, 7));  // promote i = 3, i = 7 as a witness
        let (x, y) = choose|i: int, j: int| less_than(i, j);
        assert(less_than(0, 1) || less_than(0, 2) || less_than(1, 0) || less_than(1, 2));

        assert(x < y);
    }


    spec fn f(i: int) -> bool;

    proof fn test_choose_fails() {
        let i_witness = choose|i: int| f(i);
        assert(i_witness < 0 || i_witness >= 0); // i_witness is some integer
        // assert(f(0) || f(1) || f(2) || f(3));
        // assert(f(i_witness)); // FAILS because we don't know exists|i: int| f(i)
    }    


    spec fn g() -> int;


    proof fn test_exists_skol() {
        // assert(is_even(0) || is_even(1) || is_even(2));
        // assert(is_even(g()));
        // assert(exists|i: int| #[trigger] is_even(i));  // succeeds with witness i = 4 or i = 6
    }
        
    fn main()

    {

    }
}

#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, seq::*};

verus! {



fn binary_search_f(v: &Vec<u64>, k: u64) -> (r: usize)
    requires
        v.len() > 0,
        v.len() <=  x,
        forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
        exists|i: int| 0 <= i < v.len() && k == v[i],
    ensures
        r < v.len(),
        k == v[r as int],
{
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;

  
    i1
}




    fn main()

    {

    }

}


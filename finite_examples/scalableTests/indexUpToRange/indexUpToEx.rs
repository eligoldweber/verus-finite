#[allow(unused_imports)]
use vstd::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use seq::*;
use set::*;
#[allow(unused_imports)]
use prelude::*;
use multiset::*;
verus! {


fn indexUpTo(n:u32) -> (f: Vec<u32>)
    requires n > 1,
            n < x
    ensures f.len() == n, 
             f[0] == 0,
             f[n-1] != 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = 1;

    return v;
}



fn main()

{

}

}
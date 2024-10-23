
#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, seq::*};

verus! {

// fn binary_search(v: &Vec<u64>, k: u64) -> (r: usize)
//     requires
//         forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
//         exists|i: int| 0 <= i < v.len() && k == v[i],
//     ensures
//          r == v.len(),,
//         k == v[r as int], //failed postcondition
// {
//     let mut i1: usize = 0;
//     let mut i2: usize = v.len() - 1;
//     while i1 != i2
//         invariant
//             i2 < v.len(),
//             exists|i: int| i1 <= i <= i2 && k == v[i],
//             forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
//     {
//         let ix = i1 + (i2 - i1) / 2;
//         if v[ix] < k {
//             i1 = ix + 1;
//         } else {
//             i2 = ix;
//         }
//     }
//     i1
// }
fn fullFinite_k40()

{
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    // assert(v[3] == 30);  // needed to trigger exists|i: int| ... k == v[i]
    // inline
    // assert forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
    let k = 40;
    assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    // exists|i: int| 0 <= i < v.len() && k == v[i],
    assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    //
    let r = i1;
    assert(r == v.len()-1); // This Passes! 
    assert(k == v[r as int]); 

}



fn fullFinite_k30()

{
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    // assert(v[3] == 30);  // needed to trigger exists|i: int| ... k == v[i]
    // inline
    // assert forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
    let k = 30;
    assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    // exists|i: int| 0 <= i < v.len() && k == v[i],
    assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(k == v[0] || k == v[1] || k == v[2] || k == v[3] || k == v[4]);
        assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    }
    //
    let r = i1;
    assert(r == v.len()-1);
    assert(k == v[r as int]); 

}


    fn main()

    {
    }

}
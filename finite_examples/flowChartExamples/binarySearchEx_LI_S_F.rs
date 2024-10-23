
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
//         r < v.len(),
//         k == v[r as int],
// {
//     let mut i1: usize = 0;
//     let mut i2: usize = v.len() - 1;
//     while i1 != i2
//         invariant
//             i2 < v.len(),
//             exists|i: int| i1 <= i < i2 && k == v[i], // fails (changed to be incorrect!  i <= i2 to i < i2)
//             // forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j], //commented out
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



fn bSearchFinite_checkImpl() -> (r: usize)
   
{
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    let k = 30;
    assert(v[0] <= v[0] && v[0] <= v[1] && v[0] <= v[2] && v[0] <= v[3] && v[0] <= v[4]
            && v[1] <= v[1] && v[1] <= v[2] && v[1] <= v[3] && v[1] <= v[4]
            && v[2] <= v[2] && v[2] <= v[3] && v[2] <= v[4]
            && v[3] <= v[3] && v[3] <= v[4]
            && v[4] <= v[4]);
    assert(30 == v[0] || 30 == v[1] || 30 == v[2] || 30 == v[3] || 30 == v[4]);
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    if(i1 != i2){
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if(i1 != i2){
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if(i1 != i2){
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if(i1 != i2){
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if(i1 != i2){
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    //
    let r = i1;
    // ensures
    assert(r < v.len());
    assert(k == v[r as int]);   
    r
}


fn bSearchFinite_checkLoopInv()-> (r: usize)

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
    assert(30 == v[0] || 30 == v[1] || 30 == v[2] || 30 == v[3] || 30 == v[4]);
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    if(i1 != i2){ //i2 == 4
        assert(i2 < v.len()); 
        assert(30 == v[0] || 30 == v[1] || 30 == v[2] || 30 == v[3]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2] || 30 == v[3] ); //i2 == 4
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2] || 30 == v[3] );
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2] ); // i2 == 3
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2] );
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2] );
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2]);
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2] );
    }
    if(i1 != i2){
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2] );
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
        assert(i2 < v.len());
        assert(30 == v[0] || 30 == v[1] || 30 == v[2]);
    }
    //
    let r = i1;
    r
    // assert(r == 3);
}



    fn main()

    {

    }

}
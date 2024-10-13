
#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, seq::*};

verus! {

/*
fn binary_search(v: &Vec<u64>, k: u64) -> (r: usize)
    requires
        forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
        exists|i: int| 0 <= i < v.len() && k == v[i],
    ensures
        r < v.len(),
        k == v[r as int],
{
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    while i1 != i2
        invariant
            i2 < v.len(),
            exists|i: int| i1 <= i <= i2 && k == v[i],
            forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
    {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    i1
}*/

fn binary_search_finite_0() {
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    v.push(3);
    let k: u64 = 0;
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    let r = i1;
    assert(r == k);
}

fn binary_search_finite_1() {
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    v.push(3);
    let k: u64 = 1;
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    let r = i1;
    assert(r == k);
}

fn binary_search_finite_2() {
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    v.push(3);
    let k: u64 = 2;
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    let r = i1;
    assert(r == k);
}

fn binary_search_finite_3() {
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    v.push(3);
    let k: u64 = 3;
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    if i1 != i2 {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    let r = i1;
    assert(r == k);
}

fn binary_search(v: &Vec<u64>, k: u64) -> (r: usize)
    requires
        forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
        exists|i: int| 0 <= i < v.len() && k == v[i],
    ensures
        r < v.len(),
        k == v[r as int],
{
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    while i1 != i2
        invariant
            i2 < v.len(),
            exists|i: int| i1 <= i <= i2 && k == v[i],
            forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
    {
        let ix = i1 + (i2 - i1) / 2;
        if v[ix] < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }
    }
    i1
}


    fn main()
    {
    }

}
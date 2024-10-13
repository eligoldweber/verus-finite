#![allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, seq::*};

verus! {

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

}

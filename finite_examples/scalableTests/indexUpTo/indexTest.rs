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




fn indexUpTo_finite_50() -> (f: Vec<u32>)
    ensures f.len() == 50, 
             f[0] == 0,
             f[50-1] != 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = 1;
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    if(i < 50) {
        v.push(i);
        i = i + 1;
    }
    return v;
}

fn main()
{
}
}

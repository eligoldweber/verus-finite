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


fn indexUpTo5(n:u32) -> (f: Vec<u32>)
    requires n > 1,
            n > 5
    ensures f.len() == 5, 
             f[0] == 0,
             f[5-1] != 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = 1;
    let mut m:u32 = 5; // Added
    if(i < m){
        v.push(i);
        i = i + 1;
    }
    if(i < m){
        v.push(i);
        i = i + 1;
    }
    if(i < m){
        v.push(i);
        i = i + 1;
    }
    if(i < m){
        v.push(i);
        i = i + 1;
    }
    if(i < m){
        v.push(i);
        i = i + 1;
    }
    if(i < m){
        v.push(i);
        i = i + 1;
    }

    return v;
}

fn indexUpTo_finite_rest(n:u32) -> (f: Vec<u32>)
        requires n > 5,
                 n < 10
    ensures f.len() == n, 
             f[0] == 0,
             f[n-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo5(n);
    let mut i:u32 = v.len() as u32;
    if(i < n){
        v.push(i);
        i = i + 1;
    }
    if(i < n){
        v.push(i);
        i = i + 1;
    }
    if(i < n){
        v.push(i);
        i = i + 1;
    }
    if(i < n){
        v.push(i);
        i = i + 1;
    }
    if(i < n){
        v.push(i);
        i = i + 1;
    }
    if(i < n){
        v.push(i);
        i = i + 1;
    }
    return v;
}



fn main()

{

}

}
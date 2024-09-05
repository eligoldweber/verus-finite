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
    requires n == 3,
    ensures f.len() == n, 
             f[0] == 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = 1;
    assert(v.len() == 1);
    assert(v[0] == 0);
    //
    if(i < n){
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= n);
        v.push(i);
        i = i + 1;  
    }
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= n);
    if(i < n){
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= n);
        v.push(i);
        i = i + 1;  
        }
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= n);
    if(i < n){
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= n);
        v.push(i);
        i = i + 1;  
    }
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= n);
    
    assert(v[n-1]!= 0);
    return v;
}


fn main()

{

}

}
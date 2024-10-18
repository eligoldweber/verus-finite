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




// fn indexUpTo(n:u32) -> (f: Vec<u32>)
//     requires n > 0,
//     ensures f.len() == n, 
//              f[0] == 0,
// {
//     let mut v: Vec<u32> = Vec::new();
//     v.push(0);
//     let mut i:u32 = 1;
//     assert(v.len() == 1);
//     assert(v[0] == 0);
//     while(i < n)
//         invariant i > 0,
//             v.len() == i,
//             v[0] == 0,
//             i <= n,
            
//     {
//         v.push(i);
//         i = i + 1; 
        
//     }
//     assert(v[n-1]!= 0);
//     return v;
// }


fn indexUpTo(n:u32) -> (f: Vec<u32>)
    requires n > 1,
            n < 3
    ensures f.len() == n, 
             f[0] == 0,
             f[n-1] != 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = 1;
    // assert(v.len() == 1);
    // assert(v[0] == 0);
    //
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
    // assert(v[n-1]!= 0);
    return v;
}



fn main()

{

}

}
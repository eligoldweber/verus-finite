use vstd::*;
use vstd::prelude::*;

verus! {



spec fn triangle(n: nat) -> (val:nat)
    decreases n
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat)
    }
}
// 


fn rec_triangle(n: u32) -> (sum: u32)
    requires
        triangle(n as nat) < 0x1_0000_0000,
    ensures
        sum == triangle(n as nat),
        // sum == n*((n+1)/2),
{
    if n == 0 {
        0
    } else {
        n + rec_triangle(n - 1)
    }
}

spec fn triangleF0() -> nat
{
    let n:nat = 0;
    if n == 0 {
        0
    } else {
        n
    }   
}


spec fn triangleF1() -> nat
{
    let n:nat = 1;
    if n == 0 {
        0
    } else {
        n + triangleF0()
    }   
}

spec fn triangleF2() -> nat
{
    let n:nat = 2;
    if n == 0 {
        0
    } else {
        n + triangleF1()
    }   
}

spec fn triangleF3() -> nat
{
    let n:nat = 3;
    if n == 0 {
        0
    } else {
        n + triangleF2()
    }   
}

spec fn triangleF4() -> nat
{
    let n:nat = 4;
    if n == 0 {
        0
    } else {
        n + triangleF3()
    }   
}

spec fn triangleF5() -> nat
{
    let n:nat = 5;
    if n == 0 {
        0
    } else {
        n + triangleF4()
    }   
}


fn main() {
    let n = 5;
    assert(triangleF5() == n*((n as nat+1nat)/2nat));
}


}
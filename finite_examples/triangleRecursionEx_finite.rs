use vstd::*;
use vstd::prelude::*;

verus! {



spec fn triangle(n: nat) -> nat
    decreases n
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat)
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
// 

fn main() {
    // assert(triangle(0) == 0);
    // assert(triangle(1) == 1);
    // assert(triangle(2) == 3);

    // assert(triangle(1) == (if 1 == 0  {0} else {triangle((1-1)as nat) + 1nat}));
    // assert(triangle(2) == (if 2 == 0  {0} else {triangle((2-1)as nat) + 2nat}));
    // assert(triangle(3) == (if 3 == 0  {0} else {triangle((3-1)as nat) + 3nat}));
    // assert(triangle(4) == (if 4 == 0  {0} else {triangle((4-1)as nat) + 4nat}));

    // assert(triangle(5) == 15);
    assert(triangleF5() == 15);
}


/// EX 2
fn triangleInf(n:nat){
    // proof {
    //     reveal_with_fuel(triangle, 11);
    // }

    // assert(triangle(n) == n*((n+1)/2));

    // assert(triangleF5() == 5*((5+1)/2));
}


fn triangleInf5(){
    let n = 5;

    assert(triangleF5() == n*((n as nat+1nat)/2nat));

    // assert(triangleF5() == 5*((5+1)/2));
}



}
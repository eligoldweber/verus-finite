#[allow(unused_imports)]
use vstd::*;
use vstd::prelude::*;
#[allow(unused_imports)]
use seq::*;
#[allow(unused_imports)]
use prelude::*;

verus! {

struct Vals {
x: nat,
y: nat,
}

struct State {
v: Vals,
id: int,
}

spec fn divides(factor: nat, candidate: nat) -> bool
recommends 1 <= factor
{
candidate % factor == 0
}

spec fn is_prime() -> bool {
&&& 1 < State{v: Vals{x: 50, y: 0}, id: 0}.v.x
&&& !divides(2, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(3, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(4, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(5, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(6, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(7, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(8, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(9, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(10, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(11, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(12, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(13, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(14, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(15, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(16, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(17, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(18, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(19, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(20, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(21, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(22, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(23, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(24, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(25, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(26, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(27, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(28, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(29, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(30, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(31, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(32, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(33, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(34, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(35, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(36, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(37, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(38, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(39, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(40, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(41, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(42, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(43, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(44, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(45, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(46, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(47, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(48, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)&&& !divides(49, State{v: Vals{x: 50, y: 0}, id: 0}.v.x)
}




fn main() {
assert(!is_prime());
// assert(is_prime());
}

} // verus!

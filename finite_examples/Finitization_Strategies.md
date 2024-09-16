# Heuristics for fintizing failed infinite Verus queries 

[WIP] Manual heuristics for finitizing failed queries. These are meant to finitize failed queries, using the failed assertions as a reference to perform the finitization. 

### Function Parameters
    
Parameters become encoded with implicit universial quantifiers. To finitize function parameters, they should be **pushed down**.


Example:
##### Function Parameters Infinite Example
```
spec fn divides(factor: nat, candidate: nat) -> bool
    recommends 1 <= factor
{
    candidate % factor == 0
}

spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}
fn main() { assert(is_prime(3));  //failing assertion}
```

This example contains 2 predicates, to **push down** the parameters in the context of the failing assertion  `assert(is_prime(3));` the example becomes: 

```
spec fn is_prime_3() -> bool {
    &&& 1 < 3
    &&& !divides(2, 3)
}
```

This works if the failed assertion is based on a specific value. If the proof failure is due to a general pre/post condition, then a ***harness*** is needed to create the finite values. 

Consider the following example:

```
fn binary_search(v: &Vec<u64>, k: u64) -> (r: usize)
    requires
        forall|i: int, j: int| 0 <= i <= j < v.len() ==> v[i] <= v[j],
        exists|i: int| 0 <= i < v.len() && k == v[i],
    ensures
        r == v.len(), //failed postcondition
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
```

This binary search function is **incorrect** because of the post condition r == v.len(). This only holds true when the element that is being searched for is at the end of the vector. In this case the "general" lemma has failed with no specific case to base the finitization off of. To acomplish this, the finitization needs to be able to generate values `v: &Vec<u64>, k: u64` such that these meet the pre-condition. This can be acomplished with a fuzzer[TBD]. Then in order to get decent coverage, the same finite query will be need to run with a configurable amount of different values for `v: &Vec<u64>, k: u64`. 

For example with a vector `v == [1,2,3,4,5], k == 5` the finite query would return true. But with a different configuration `v == [1,2,3,4,5], k == 4` the query would fail.

 

### Quantifiers

##### Universal 
Finitizing Universal quantifiers becomes just a conjunction expression. 

Consider:
```
spec fn divides(factor: nat, candidate: nat) -> bool
    recommends 1 <= factor
{
    candidate % factor == 0
}

spec fn is_prime(candidate: nat) -> bool {
    &&& 1 < candidate
    &&& forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)
}
fn main() { assert(is_prime(11));  //failing assertion}
```

To finitize `forall|factor: nat| 1 < factor < candidate ==> !divides(factor, candidate)` for `assert(is_prime(11))` the expression would be an enumeration of all possible combinations of the forall in a conjunction. 

```
&&& !divides(2, 11) 
&&& !divides(3, 11)
&&& !divides(4, 11)
&&& !divides(5, 11)
&&& !divides(6, 11)
&&& !divides(7, 11)
&&& !divides(8, 11)
&&& !divides(9, 11)
&&& !divides(10, 11)
```

##### Existential 

Finitizing Universal quantifiers becomes just a disjunction expression. 

Consider:
```
    proof fn test_exists_fails() {
        assert(exists|i: int| #[trigger] is_even(i)); // fails
    }
```

Finitizing around the type if `int i`, this expression can be quantified as:

```
assert(is_even(-1) || is_even(0) || is_even(1) || is_even(2));
```


### Loops

Loops are unrolled -- the loop condition is checked with an if-statement, and the loop invariants (if there are any) are checked with assertions before and after each loop iteration. 

```
fn indexUpTo(n:u32) -> (f: Vec<u32>)
    requires n > 0,
    ensures f.len() == n, 
             f[0] == 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = 1;
    assert(v.len() == 1);
    assert(v[0] == 0);
    while(i < n)
        invariant i > 0,
            v.len() == i,
            v[0] == 0,
            i <= n,
            
    {
        v.push(i);
        i = i + 1; 
        
    }
    assert(v[n-1]!= 0);
    return v;
}
```

Becomes (for n == 3): 

```
fn indexUpTo() -> (f: Vec<u32>)
    ensures f.len() == 3, 
             f[0] == 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = 1;
    assert(v.len() == 1);
    assert(v[0] == 0);
    //
    if(i < 3){
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= 3);
        v.push(i);
        i = i + 1;  
    }
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= 3);
    if(i < 3){
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= 3);
        v.push(i);
        i = i + 1;  
        }
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= 3);
    if(i < 3){
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= 3);
        v.push(i);
        i = i + 1;  
    }
        assert(i > 0);
        assert(v.len() == i);
        assert(v[0] == 0);
        assert(i <= 3);
    
    assert(v[3-1]!= 0);
    return v;
}

```

### Recursive Definitions

Recursive definitions, like loops, are also unrolled into individial functions. 

```
spec fn triangle(n: nat) -> nat
    decreases n
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat)
    }
}
fn main() { assert(triangle(5) == 15); // assertion failed}
```

Becomes:

```
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

    assert(triangleF5() == 15);
}
```
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




fn indexUpTo_finite_1() -> (f: Vec<u32>)
    ensures f.len() == 1, 
             f[0] == 0,
{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    let mut i:u32 = v.len() as u32;
    if (i < 1) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_2() -> (f: Vec<u32>)
    ensures f.len() == 2, 
             f[0] == 0,
             f[2-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_1();
    let mut i:u32 = v.len() as u32;
    if (i < 2) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_3() -> (f: Vec<u32>)
    ensures f.len() == 3, 
             f[0] == 0,
             f[3-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_2();
    let mut i:u32 = v.len() as u32;
    if (i < 3) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_4() -> (f: Vec<u32>)
    ensures f.len() == 4, 
             f[0] == 0,
             f[4-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_3();
    let mut i:u32 = v.len() as u32;
    if (i < 4) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_5() -> (f: Vec<u32>)
    ensures f.len() == 5, 
             f[0] == 0,
             f[5-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_4();
    let mut i:u32 = v.len() as u32;
    if (i < 5) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_6() -> (f: Vec<u32>)
    ensures f.len() == 6, 
             f[0] == 0,
             f[6-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_5();
    let mut i:u32 = v.len() as u32;
    if (i < 6) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_7() -> (f: Vec<u32>)
    ensures f.len() == 7, 
             f[0] == 0,
             f[7-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_6();
    let mut i:u32 = v.len() as u32;
    if (i < 7) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_8() -> (f: Vec<u32>)
    ensures f.len() == 8, 
             f[0] == 0,
             f[8-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_7();
    let mut i:u32 = v.len() as u32;
    if (i < 8) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_9() -> (f: Vec<u32>)
    ensures f.len() == 9, 
             f[0] == 0,
             f[9-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_8();
    let mut i:u32 = v.len() as u32;
    if (i < 9) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_10() -> (f: Vec<u32>)
    ensures f.len() == 10, 
             f[0] == 0,
             f[10-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_9();
    let mut i:u32 = v.len() as u32;
    if (i < 10) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_11() -> (f: Vec<u32>)
    ensures f.len() == 11, 
             f[0] == 0,
             f[11-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_10();
    let mut i:u32 = v.len() as u32;
    if (i < 11) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_12() -> (f: Vec<u32>)
    ensures f.len() == 12, 
             f[0] == 0,
             f[12-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_11();
    let mut i:u32 = v.len() as u32;
    if (i < 12) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_13() -> (f: Vec<u32>)
    ensures f.len() == 13, 
             f[0] == 0,
             f[13-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_12();
    let mut i:u32 = v.len() as u32;
    if (i < 13) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_14() -> (f: Vec<u32>)
    ensures f.len() == 14, 
             f[0] == 0,
             f[14-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_13();
    let mut i:u32 = v.len() as u32;
    if (i < 14) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_15() -> (f: Vec<u32>)
    ensures f.len() == 15, 
             f[0] == 0,
             f[15-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_14();
    let mut i:u32 = v.len() as u32;
    if (i < 15) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_16() -> (f: Vec<u32>)
    ensures f.len() == 16, 
             f[0] == 0,
             f[16-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_15();
    let mut i:u32 = v.len() as u32;
    if (i < 16) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_17() -> (f: Vec<u32>)
    ensures f.len() == 17, 
             f[0] == 0,
             f[17-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_16();
    let mut i:u32 = v.len() as u32;
    if (i < 17) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_18() -> (f: Vec<u32>)
    ensures f.len() == 18, 
             f[0] == 0,
             f[18-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_17();
    let mut i:u32 = v.len() as u32;
    if (i < 18) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_19() -> (f: Vec<u32>)
    ensures f.len() == 19, 
             f[0] == 0,
             f[19-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_18();
    let mut i:u32 = v.len() as u32;
    if (i < 19) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_20() -> (f: Vec<u32>)
    ensures f.len() == 20, 
             f[0] == 0,
             f[20-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_19();
    let mut i:u32 = v.len() as u32;
    if (i < 20) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_21() -> (f: Vec<u32>)
    ensures f.len() == 21, 
             f[0] == 0,
             f[21-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_20();
    let mut i:u32 = v.len() as u32;
    if (i < 21) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_22() -> (f: Vec<u32>)
    ensures f.len() == 22, 
             f[0] == 0,
             f[22-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_21();
    let mut i:u32 = v.len() as u32;
    if (i < 22) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_23() -> (f: Vec<u32>)
    ensures f.len() == 23, 
             f[0] == 0,
             f[23-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_22();
    let mut i:u32 = v.len() as u32;
    if (i < 23) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_24() -> (f: Vec<u32>)
    ensures f.len() == 24, 
             f[0] == 0,
             f[24-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_23();
    let mut i:u32 = v.len() as u32;
    if (i < 24) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_25() -> (f: Vec<u32>)
    ensures f.len() == 25, 
             f[0] == 0,
             f[25-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_24();
    let mut i:u32 = v.len() as u32;
    if (i < 25) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_26() -> (f: Vec<u32>)
    ensures f.len() == 26, 
             f[0] == 0,
             f[26-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_25();
    let mut i:u32 = v.len() as u32;
    if (i < 26) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_27() -> (f: Vec<u32>)
    ensures f.len() == 27, 
             f[0] == 0,
             f[27-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_26();
    let mut i:u32 = v.len() as u32;
    if (i < 27) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_28() -> (f: Vec<u32>)
    ensures f.len() == 28, 
             f[0] == 0,
             f[28-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_27();
    let mut i:u32 = v.len() as u32;
    if (i < 28) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_29() -> (f: Vec<u32>)
    ensures f.len() == 29, 
             f[0] == 0,
             f[29-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_28();
    let mut i:u32 = v.len() as u32;
    if (i < 29) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_30() -> (f: Vec<u32>)
    ensures f.len() == 30, 
             f[0] == 0,
             f[30-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_29();
    let mut i:u32 = v.len() as u32;
    if (i < 30) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_31() -> (f: Vec<u32>)
    ensures f.len() == 31, 
             f[0] == 0,
             f[31-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_30();
    let mut i:u32 = v.len() as u32;
    if (i < 31) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_32() -> (f: Vec<u32>)
    ensures f.len() == 32, 
             f[0] == 0,
             f[32-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_31();
    let mut i:u32 = v.len() as u32;
    if (i < 32) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_33() -> (f: Vec<u32>)
    ensures f.len() == 33, 
             f[0] == 0,
             f[33-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_32();
    let mut i:u32 = v.len() as u32;
    if (i < 33) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_34() -> (f: Vec<u32>)
    ensures f.len() == 34, 
             f[0] == 0,
             f[34-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_33();
    let mut i:u32 = v.len() as u32;
    if (i < 34) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_35() -> (f: Vec<u32>)
    ensures f.len() == 35, 
             f[0] == 0,
             f[35-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_34();
    let mut i:u32 = v.len() as u32;
    if (i < 35) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_36() -> (f: Vec<u32>)
    ensures f.len() == 36, 
             f[0] == 0,
             f[36-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_35();
    let mut i:u32 = v.len() as u32;
    if (i < 36) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_37() -> (f: Vec<u32>)
    ensures f.len() == 37, 
             f[0] == 0,
             f[37-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_36();
    let mut i:u32 = v.len() as u32;
    if (i < 37) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_38() -> (f: Vec<u32>)
    ensures f.len() == 38, 
             f[0] == 0,
             f[38-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_37();
    let mut i:u32 = v.len() as u32;
    if (i < 38) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_39() -> (f: Vec<u32>)
    ensures f.len() == 39, 
             f[0] == 0,
             f[39-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_38();
    let mut i:u32 = v.len() as u32;
    if (i < 39) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_40() -> (f: Vec<u32>)
    ensures f.len() == 40, 
             f[0] == 0,
             f[40-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_39();
    let mut i:u32 = v.len() as u32;
    if (i < 40) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_41() -> (f: Vec<u32>)
    ensures f.len() == 41, 
             f[0] == 0,
             f[41-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_40();
    let mut i:u32 = v.len() as u32;
    if (i < 41) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_42() -> (f: Vec<u32>)
    ensures f.len() == 42, 
             f[0] == 0,
             f[42-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_41();
    let mut i:u32 = v.len() as u32;
    if (i < 42) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_43() -> (f: Vec<u32>)
    ensures f.len() == 43, 
             f[0] == 0,
             f[43-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_42();
    let mut i:u32 = v.len() as u32;
    if (i < 43) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_44() -> (f: Vec<u32>)
    ensures f.len() == 44, 
             f[0] == 0,
             f[44-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_43();
    let mut i:u32 = v.len() as u32;
    if (i < 44) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_45() -> (f: Vec<u32>)
    ensures f.len() == 45, 
             f[0] == 0,
             f[45-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_44();
    let mut i:u32 = v.len() as u32;
    if (i < 45) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_46() -> (f: Vec<u32>)
    ensures f.len() == 46, 
             f[0] == 0,
             f[46-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_45();
    let mut i:u32 = v.len() as u32;
    if (i < 46) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_47() -> (f: Vec<u32>)
    ensures f.len() == 47, 
             f[0] == 0,
             f[47-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_46();
    let mut i:u32 = v.len() as u32;
    if (i < 47) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_48() -> (f: Vec<u32>)
    ensures f.len() == 48, 
             f[0] == 0,
             f[48-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_47();
    let mut i:u32 = v.len() as u32;
    if (i < 48) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_49() -> (f: Vec<u32>)
    ensures f.len() == 49, 
             f[0] == 0,
             f[49-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_48();
    let mut i:u32 = v.len() as u32;
    if (i < 49) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_50() -> (f: Vec<u32>)
    ensures f.len() == 50, 
             f[0] == 0,
             f[50-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_49();
    let mut i:u32 = v.len() as u32;
    if (i < 50) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_51() -> (f: Vec<u32>)
    ensures f.len() == 51, 
             f[0] == 0,
             f[51-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_50();
    let mut i:u32 = v.len() as u32;
    if (i < 51) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_52() -> (f: Vec<u32>)
    ensures f.len() == 52, 
             f[0] == 0,
             f[52-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_51();
    let mut i:u32 = v.len() as u32;
    if (i < 52) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_53() -> (f: Vec<u32>)
    ensures f.len() == 53, 
             f[0] == 0,
             f[53-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_52();
    let mut i:u32 = v.len() as u32;
    if (i < 53) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_54() -> (f: Vec<u32>)
    ensures f.len() == 54, 
             f[0] == 0,
             f[54-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_53();
    let mut i:u32 = v.len() as u32;
    if (i < 54) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_55() -> (f: Vec<u32>)
    ensures f.len() == 55, 
             f[0] == 0,
             f[55-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_54();
    let mut i:u32 = v.len() as u32;
    if (i < 55) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_56() -> (f: Vec<u32>)
    ensures f.len() == 56, 
             f[0] == 0,
             f[56-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_55();
    let mut i:u32 = v.len() as u32;
    if (i < 56) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_57() -> (f: Vec<u32>)
    ensures f.len() == 57, 
             f[0] == 0,
             f[57-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_56();
    let mut i:u32 = v.len() as u32;
    if (i < 57) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_58() -> (f: Vec<u32>)
    ensures f.len() == 58, 
             f[0] == 0,
             f[58-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_57();
    let mut i:u32 = v.len() as u32;
    if (i < 58) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_59() -> (f: Vec<u32>)
    ensures f.len() == 59, 
             f[0] == 0,
             f[59-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_58();
    let mut i:u32 = v.len() as u32;
    if (i < 59) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_60() -> (f: Vec<u32>)
    ensures f.len() == 60, 
             f[0] == 0,
             f[60-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_59();
    let mut i:u32 = v.len() as u32;
    if (i < 60) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_61() -> (f: Vec<u32>)
    ensures f.len() == 61, 
             f[0] == 0,
             f[61-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_60();
    let mut i:u32 = v.len() as u32;
    if (i < 61) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_62() -> (f: Vec<u32>)
    ensures f.len() == 62, 
             f[0] == 0,
             f[62-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_61();
    let mut i:u32 = v.len() as u32;
    if (i < 62) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_63() -> (f: Vec<u32>)
    ensures f.len() == 63, 
             f[0] == 0,
             f[63-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_62();
    let mut i:u32 = v.len() as u32;
    if (i < 63) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_64() -> (f: Vec<u32>)
    ensures f.len() == 64, 
             f[0] == 0,
             f[64-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_63();
    let mut i:u32 = v.len() as u32;
    if (i < 64) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_65() -> (f: Vec<u32>)
    ensures f.len() == 65, 
             f[0] == 0,
             f[65-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_64();
    let mut i:u32 = v.len() as u32;
    if (i < 65) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_66() -> (f: Vec<u32>)
    ensures f.len() == 66, 
             f[0] == 0,
             f[66-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_65();
    let mut i:u32 = v.len() as u32;
    if (i < 66) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_67() -> (f: Vec<u32>)
    ensures f.len() == 67, 
             f[0] == 0,
             f[67-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_66();
    let mut i:u32 = v.len() as u32;
    if (i < 67) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_68() -> (f: Vec<u32>)
    ensures f.len() == 68, 
             f[0] == 0,
             f[68-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_67();
    let mut i:u32 = v.len() as u32;
    if (i < 68) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_69() -> (f: Vec<u32>)
    ensures f.len() == 69, 
             f[0] == 0,
             f[69-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_68();
    let mut i:u32 = v.len() as u32;
    if (i < 69) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_70() -> (f: Vec<u32>)
    ensures f.len() == 70, 
             f[0] == 0,
             f[70-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_69();
    let mut i:u32 = v.len() as u32;
    if (i < 70) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_71() -> (f: Vec<u32>)
    ensures f.len() == 71, 
             f[0] == 0,
             f[71-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_70();
    let mut i:u32 = v.len() as u32;
    if (i < 71) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_72() -> (f: Vec<u32>)
    ensures f.len() == 72, 
             f[0] == 0,
             f[72-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_71();
    let mut i:u32 = v.len() as u32;
    if (i < 72) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_73() -> (f: Vec<u32>)
    ensures f.len() == 73, 
             f[0] == 0,
             f[73-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_72();
    let mut i:u32 = v.len() as u32;
    if (i < 73) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_74() -> (f: Vec<u32>)
    ensures f.len() == 74, 
             f[0] == 0,
             f[74-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_73();
    let mut i:u32 = v.len() as u32;
    if (i < 74) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_75() -> (f: Vec<u32>)
    ensures f.len() == 75, 
             f[0] == 0,
             f[75-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_74();
    let mut i:u32 = v.len() as u32;
    if (i < 75) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_76() -> (f: Vec<u32>)
    ensures f.len() == 76, 
             f[0] == 0,
             f[76-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_75();
    let mut i:u32 = v.len() as u32;
    if (i < 76) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_77() -> (f: Vec<u32>)
    ensures f.len() == 77, 
             f[0] == 0,
             f[77-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_76();
    let mut i:u32 = v.len() as u32;
    if (i < 77) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_78() -> (f: Vec<u32>)
    ensures f.len() == 78, 
             f[0] == 0,
             f[78-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_77();
    let mut i:u32 = v.len() as u32;
    if (i < 78) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_79() -> (f: Vec<u32>)
    ensures f.len() == 79, 
             f[0] == 0,
             f[79-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_78();
    let mut i:u32 = v.len() as u32;
    if (i < 79) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_80() -> (f: Vec<u32>)
    ensures f.len() == 80, 
             f[0] == 0,
             f[80-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_79();
    let mut i:u32 = v.len() as u32;
    if (i < 80) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_81() -> (f: Vec<u32>)
    ensures f.len() == 81, 
             f[0] == 0,
             f[81-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_80();
    let mut i:u32 = v.len() as u32;
    if (i < 81) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_82() -> (f: Vec<u32>)
    ensures f.len() == 82, 
             f[0] == 0,
             f[82-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_81();
    let mut i:u32 = v.len() as u32;
    if (i < 82) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_83() -> (f: Vec<u32>)
    ensures f.len() == 83, 
             f[0] == 0,
             f[83-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_82();
    let mut i:u32 = v.len() as u32;
    if (i < 83) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_84() -> (f: Vec<u32>)
    ensures f.len() == 84, 
             f[0] == 0,
             f[84-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_83();
    let mut i:u32 = v.len() as u32;
    if (i < 84) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_85() -> (f: Vec<u32>)
    ensures f.len() == 85, 
             f[0] == 0,
             f[85-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_84();
    let mut i:u32 = v.len() as u32;
    if (i < 85) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_86() -> (f: Vec<u32>)
    ensures f.len() == 86, 
             f[0] == 0,
             f[86-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_85();
    let mut i:u32 = v.len() as u32;
    if (i < 86) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_87() -> (f: Vec<u32>)
    ensures f.len() == 87, 
             f[0] == 0,
             f[87-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_86();
    let mut i:u32 = v.len() as u32;
    if (i < 87) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_88() -> (f: Vec<u32>)
    ensures f.len() == 88, 
             f[0] == 0,
             f[88-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_87();
    let mut i:u32 = v.len() as u32;
    if (i < 88) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_89() -> (f: Vec<u32>)
    ensures f.len() == 89, 
             f[0] == 0,
             f[89-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_88();
    let mut i:u32 = v.len() as u32;
    if (i < 89) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_90() -> (f: Vec<u32>)
    ensures f.len() == 90, 
             f[0] == 0,
             f[90-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_89();
    let mut i:u32 = v.len() as u32;
    if (i < 90) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_91() -> (f: Vec<u32>)
    ensures f.len() == 91, 
             f[0] == 0,
             f[91-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_90();
    let mut i:u32 = v.len() as u32;
    if (i < 91) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_92() -> (f: Vec<u32>)
    ensures f.len() == 92, 
             f[0] == 0,
             f[92-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_91();
    let mut i:u32 = v.len() as u32;
    if (i < 92) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_93() -> (f: Vec<u32>)
    ensures f.len() == 93, 
             f[0] == 0,
             f[93-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_92();
    let mut i:u32 = v.len() as u32;
    if (i < 93) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_94() -> (f: Vec<u32>)
    ensures f.len() == 94, 
             f[0] == 0,
             f[94-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_93();
    let mut i:u32 = v.len() as u32;
    if (i < 94) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_95() -> (f: Vec<u32>)
    ensures f.len() == 95, 
             f[0] == 0,
             f[95-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_94();
    let mut i:u32 = v.len() as u32;
    if (i < 95) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_96() -> (f: Vec<u32>)
    ensures f.len() == 96, 
             f[0] == 0,
             f[96-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_95();
    let mut i:u32 = v.len() as u32;
    if (i < 96) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_97() -> (f: Vec<u32>)
    ensures f.len() == 97, 
             f[0] == 0,
             f[97-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_96();
    let mut i:u32 = v.len() as u32;
    if (i < 97) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_98() -> (f: Vec<u32>)
    ensures f.len() == 98, 
             f[0] == 0,
             f[98-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_97();
    let mut i:u32 = v.len() as u32;
    if (i < 98) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_99() -> (f: Vec<u32>)
    ensures f.len() == 99, 
             f[0] == 0,
             f[99-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_98();
    let mut i:u32 = v.len() as u32;
    if (i < 99) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_100() -> (f: Vec<u32>)
    ensures f.len() == 100, 
             f[0] == 0,
             f[100-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_99();
    let mut i:u32 = v.len() as u32;
    if (i < 100) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_101() -> (f: Vec<u32>)
    ensures f.len() == 101, 
             f[0] == 0,
             f[101-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_100();
    let mut i:u32 = v.len() as u32;
    if (i < 101) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_102() -> (f: Vec<u32>)
    ensures f.len() == 102, 
             f[0] == 0,
             f[102-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_101();
    let mut i:u32 = v.len() as u32;
    if (i < 102) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_103() -> (f: Vec<u32>)
    ensures f.len() == 103, 
             f[0] == 0,
             f[103-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_102();
    let mut i:u32 = v.len() as u32;
    if (i < 103) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_104() -> (f: Vec<u32>)
    ensures f.len() == 104, 
             f[0] == 0,
             f[104-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_103();
    let mut i:u32 = v.len() as u32;
    if (i < 104) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_105() -> (f: Vec<u32>)
    ensures f.len() == 105, 
             f[0] == 0,
             f[105-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_104();
    let mut i:u32 = v.len() as u32;
    if (i < 105) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_106() -> (f: Vec<u32>)
    ensures f.len() == 106, 
             f[0] == 0,
             f[106-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_105();
    let mut i:u32 = v.len() as u32;
    if (i < 106) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_107() -> (f: Vec<u32>)
    ensures f.len() == 107, 
             f[0] == 0,
             f[107-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_106();
    let mut i:u32 = v.len() as u32;
    if (i < 107) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_108() -> (f: Vec<u32>)
    ensures f.len() == 108, 
             f[0] == 0,
             f[108-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_107();
    let mut i:u32 = v.len() as u32;
    if (i < 108) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_109() -> (f: Vec<u32>)
    ensures f.len() == 109, 
             f[0] == 0,
             f[109-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_108();
    let mut i:u32 = v.len() as u32;
    if (i < 109) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_110() -> (f: Vec<u32>)
    ensures f.len() == 110, 
             f[0] == 0,
             f[110-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_109();
    let mut i:u32 = v.len() as u32;
    if (i < 110) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_111() -> (f: Vec<u32>)
    ensures f.len() == 111, 
             f[0] == 0,
             f[111-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_110();
    let mut i:u32 = v.len() as u32;
    if (i < 111) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_112() -> (f: Vec<u32>)
    ensures f.len() == 112, 
             f[0] == 0,
             f[112-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_111();
    let mut i:u32 = v.len() as u32;
    if (i < 112) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_113() -> (f: Vec<u32>)
    ensures f.len() == 113, 
             f[0] == 0,
             f[113-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_112();
    let mut i:u32 = v.len() as u32;
    if (i < 113) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_114() -> (f: Vec<u32>)
    ensures f.len() == 114, 
             f[0] == 0,
             f[114-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_113();
    let mut i:u32 = v.len() as u32;
    if (i < 114) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_115() -> (f: Vec<u32>)
    ensures f.len() == 115, 
             f[0] == 0,
             f[115-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_114();
    let mut i:u32 = v.len() as u32;
    if (i < 115) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_116() -> (f: Vec<u32>)
    ensures f.len() == 116, 
             f[0] == 0,
             f[116-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_115();
    let mut i:u32 = v.len() as u32;
    if (i < 116) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_117() -> (f: Vec<u32>)
    ensures f.len() == 117, 
             f[0] == 0,
             f[117-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_116();
    let mut i:u32 = v.len() as u32;
    if (i < 117) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_118() -> (f: Vec<u32>)
    ensures f.len() == 118, 
             f[0] == 0,
             f[118-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_117();
    let mut i:u32 = v.len() as u32;
    if (i < 118) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_119() -> (f: Vec<u32>)
    ensures f.len() == 119, 
             f[0] == 0,
             f[119-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_118();
    let mut i:u32 = v.len() as u32;
    if (i < 119) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_120() -> (f: Vec<u32>)
    ensures f.len() == 120, 
             f[0] == 0,
             f[120-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_119();
    let mut i:u32 = v.len() as u32;
    if (i < 120) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_121() -> (f: Vec<u32>)
    ensures f.len() == 121, 
             f[0] == 0,
             f[121-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_120();
    let mut i:u32 = v.len() as u32;
    if (i < 121) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_122() -> (f: Vec<u32>)
    ensures f.len() == 122, 
             f[0] == 0,
             f[122-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_121();
    let mut i:u32 = v.len() as u32;
    if (i < 122) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_123() -> (f: Vec<u32>)
    ensures f.len() == 123, 
             f[0] == 0,
             f[123-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_122();
    let mut i:u32 = v.len() as u32;
    if (i < 123) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_124() -> (f: Vec<u32>)
    ensures f.len() == 124, 
             f[0] == 0,
             f[124-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_123();
    let mut i:u32 = v.len() as u32;
    if (i < 124) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_125() -> (f: Vec<u32>)
    ensures f.len() == 125, 
             f[0] == 0,
             f[125-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_124();
    let mut i:u32 = v.len() as u32;
    if (i < 125) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_126() -> (f: Vec<u32>)
    ensures f.len() == 126, 
             f[0] == 0,
             f[126-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_125();
    let mut i:u32 = v.len() as u32;
    if (i < 126) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_127() -> (f: Vec<u32>)
    ensures f.len() == 127, 
             f[0] == 0,
             f[127-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_126();
    let mut i:u32 = v.len() as u32;
    if (i < 127) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_128() -> (f: Vec<u32>)
    ensures f.len() == 128, 
             f[0] == 0,
             f[128-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_127();
    let mut i:u32 = v.len() as u32;
    if (i < 128) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_129() -> (f: Vec<u32>)
    ensures f.len() == 129, 
             f[0] == 0,
             f[129-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_128();
    let mut i:u32 = v.len() as u32;
    if (i < 129) {
        v.push(i);
    }
    return v;
}

fn indexUpTo_finite_130() -> (f: Vec<u32>)
    ensures f.len() == 130, 
             f[0] == 0,
             f[130-1] != 0,
{
    let mut v: Vec<u32> = indexUpTo_finite_129();
    let mut i:u32 = v.len() as u32;
    if (i < 130) {
        v.push(i);
    }
    return v;
}

fn main()
{
}
}

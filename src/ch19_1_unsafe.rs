// NOTE: raw pointer implemented `Copy` trait

static mut HELLO_WORLD: &str = "hello world"; // global variable, access is unsafe

#[test]
fn test() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = 0x012345usize as *const i32;

    match std::mem::size_of_val(&r3) {
        x if x == 8 => assert_eq!("0x0000000000012345", format!("{:#?}", r3)),
        x if x == 4 => assert_eq!("0x00012345", format!("{:#?}", r3)),
        x => panic!("I'am {}-bit system!", x),
    } // 0-F is 4 bits

    unsafe {
        *r2 += 5;
        assert_eq!(10, *r1);
    }
    assert_eq!(10, num);

    let num = 42;
    let r4 = &num as *const i32 as usize as *mut i32; // is this safe?
    unsafe {
        *r4 -= 5;
    }
    assert_eq!(37, num);

    extern "C" {
        fn abs(inputx: i32) -> u32;
        // function name must match, but parameter name not
        // application binary interface (ABI)
        // call at assembly level 汇编级
    }
    unsafe {
        assert_eq!(3, abs(-3));

        assert_eq!("hello world", HELLO_WORLD);
        HELLO_WORLD = "hello unsafe";
        assert_eq!("hello unsafe", HELLO_WORLD);
    }
}

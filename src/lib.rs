#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("../bindings.rs");

pub fn connect_lj_u6() ->i32{
    let address_c_string = CString::new("1").unwrap();
    let mut handle :i32 = 0;
    let ptr: *mut i32 = &mut handle;
    unsafe {
        let e_code = OpenLabJack(LJ_dtU6, LJ_ctUSB, address_c_string.as_ptr() as *const i8, 1, ptr);
        if e_code == 0{
            println!("Labjack connected successfully!");
        }else if e_code == 1007{
            println!("No Labjack device found");
        }else{
            println!("Labjack returned error code {:?}", e_code);
        }
    };
    handle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = connect_lj_u6();
    }
}

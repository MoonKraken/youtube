use std::rc::Rc;

trait SomeTrait {}
fn main() {
    // Rust Pizza and Slices

    let arr: [i32; 5] = [1,2,3,4,5];
    let slc: &[i32] = &[1,2,3];
    // let trt: SomeTrait;

    let slc_smart: Box<[i32]> = Box::new([1,2,3]);

    let slc2: &[i32] = &arr[0..2];

    let slc3: &[i32] = &arr;

    let a_string: String = String::new();

    let a_slice: &str = &a_string;
    // let bytes: &[u8];

    let mut b_string = String::with_capacity(10);
    b_string.push_str("pizza");
    b_string.push_str(" time");
    println!("{}", b_string);

    let str_slice: &str = &b_string[0..3];

    let str_slice2: &str = &b_string;

    let str_borrow: &String = &b_string;
    println!("{}", str_borrow.capacity());

    let smart_ptr: Rc<str> = Rc::from(b_string);
    println!("{}", &smart_ptr);
}


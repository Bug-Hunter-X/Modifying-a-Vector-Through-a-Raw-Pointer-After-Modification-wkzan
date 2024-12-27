fn main() {
    let mut v = vec![1, 2, 3];
    let mut ptr = v.as_mut_ptr();
    unsafe{
        *ptr = 10; // This is still unsafe, but it's safe if there is a guarantee that v is not modified after
        // This is where the bug is likely to happen
    }
    println!(" {:?}", v);
}
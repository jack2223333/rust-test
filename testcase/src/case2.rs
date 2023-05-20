///在rust中调用c语言的malloc函数,实现内存拷贝功能，T是泛型
///val是需要拷贝的内容，size是开辟空间的大小，当size的大小小于所需空间的大小，或者size为不合法的数值，就会报错
pub struct Cat{
    pub height: u32,
    pub width:u32,
    pub weight:u64,
    pub name:String
}

pub fn copy<T>(val: T,size: usize) -> &'static mut T {
    unsafe {
        let ptr: *mut T = libc::malloc(size).cast();
        if ptr.is_null() {
            std::process::abort();
        }
        ptr.write(val);
        &mut *ptr
    }
}
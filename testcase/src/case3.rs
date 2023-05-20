pub struct Buf {
    header: String,
    body: [u8; 1024],
}
/*
constructor传入的裸指针p可能未被初始化，不能对其取引用
*/
impl Buf {
    
    pub unsafe fn constructor(p: *mut Buf) {
        let header: *mut String = &mut (*p).header;
        header.write(String::default());

        let body: *mut [u8; 1024] = &mut (*p).body;
        body.write_bytes(0, 1);
    }
}
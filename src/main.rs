use std::os::raw::{c_int, c_uchar};

#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
struct zmq_msg_t {
    data: [c_uchar; 64],
}

extern "C" {
    fn zmq_msg_init(msg: *mut zmq_msg_t) -> c_int;
}

fn main() {
    unsafe {
        let mut msg = zmq_msg_t { data: [0; 64] };
        zmq_msg_init(&mut msg);
    }
}

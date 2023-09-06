use std::ops::Deref;

struct MyRc<T> {
    data: *const T,
    cnt: *mut usize,
}
impl<T> MyRc<T> {
    fn new(data: T) -> Self {
        let tmp = Box::new(data);
        let datat = Box::into_raw(tmp);
        let cnt = Box::into_raw(Box::new(1));
        MyRc {
            data: datat,
            cnt,
        }
    }
    fn clone(&self) -> Self {
        unsafe {
            *self.cnt += 1;
        }
        MyRc {
            data: self.data,
            cnt: self.cnt,
        }
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            if *self.cnt > 1 {
                *self.cnt -= 1;
            } else {
                Box::from_raw(self.data as *mut T);
                Box::from_raw(self.cnt);
            }
        }
    }
}

fn main() {
    let my_rc1 = MyRc::new(5);
    let my_rc2 = my_rc1.clone();

    println!("rc1: {}", *my_rc1);
    println!("rc2: {}", *my_rc2);
}

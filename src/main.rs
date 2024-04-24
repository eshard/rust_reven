use std::mem;

trait CallbackData {}

#[derive(Debug)]
struct MyCallbackData<'a> {
    data: &'a [u8],
}

impl<'a> CallbackData for MyCallbackData<'a> {}

struct MyCallback<'a, T: CallbackData + 'a> {
    callback: Box<dyn Fn(&T) + 'a>,
}

trait MyTrait<'a, T: CallbackData + 'a> {
    fn set_callback(&mut self, cb: MyCallback<'a, T>);
    fn do_something(&self);
}

struct MyStruct<'a, T: CallbackData + 'a> {
    callbacks: Vec<MyCallback<'a, T>>,
    data: &'a [u8],
}

impl<'a> MyStruct<'a, MyCallbackData<'a>> {
    //Usage of FnOnce as a bound to accept a parameter of function-like type and only need to call it once
    fn with_scope<F>(data: [u8; 3], f: F)
    where
        F: for<'b> FnOnce(&mut MyStruct<'b, MyCallbackData<'b>>),
    {
        let mut s = MyStruct {
            callbacks: Vec::new(),
            data: &data,
        };
        f(&mut s);
        // Ensure the borrow drop explicitly
        mem::drop(s);
    }
}

impl<'a, 'b: 'a> MyTrait<'a, MyCallbackData<'a>> for MyStruct<'b, MyCallbackData<'a>> {
    fn set_callback(&mut self, cb: MyCallback<'a, MyCallbackData<'a>>) {
        self.callbacks.push(cb);
    }

    fn do_something(&self) {
        for cb in &self.callbacks {
            let cb_data = MyCallbackData { data: self.data };
            (cb.callback)(&cb_data);
        }
    }
}

fn main() {
    let data = [1, 2, 3];

    MyStruct::with_scope(data, |s| {
        s.set_callback(MyCallback {
            callback: Box::new(|data: &MyCallbackData| {
                println!("Callback called with data {:?}", data);
            }),
        });

        s.do_something();
    });
}
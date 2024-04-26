trait CallbackData {}

#[derive(Debug)]
struct MyCallbackData<'a> {
    data: &'a [u8],
}

impl<'a> CallbackData for MyCallbackData<'a> {}

struct MyCallback<T: CallbackData> {
    callback: Box<dyn Fn(&T)>,
}

trait MyTrait<'a, T: CallbackData> {
    fn set_callback(&mut self, cb: MyCallback<T>);
    fn do_something(&self);
}

struct MyStruct<'a, T: CallbackData> {
    callbacks: Vec<MyCallback<T>>,
    data: &'a [u8; 3],
}

impl<'a> MyTrait<'a, MyCallbackData<'a>> for MyStruct<'a, MyCallbackData<'a>> {
    fn set_callback(&mut self, cb: MyCallback<MyCallbackData<'a>>) {
        self.callbacks.push(cb);
    }

    fn do_something(&self) {

        for cb in &self.callbacks {
            let cb_data = MyCallbackData {
                data: self.data,
            };

            (cb.callback)(&cb_data);
        }
    }
}

fn main() {
    let mut s = MyStruct {
        callbacks: Vec::new(),
        data: &[1, 2, 3],
    };

    s.set_callback(MyCallback {
        callback: Box::new(|data: &MyCallbackData| {
            println!("Callback called with data {:?}", data);
        }),
    });

    s.do_something();
}

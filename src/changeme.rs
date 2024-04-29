pub trait CallbackData {}

#[derive(Debug)]
pub struct MyCallbackData<'a> {
    data: &'a [u8],
}

impl<'a> CallbackData for MyCallbackData<'a> {}

pub struct MyCallback<T: CallbackData> {
    callback: Box<dyn Fn(&T)>,
}

pub trait MyTrait<T: CallbackData> {
    fn set_callback(&mut self, cb: MyCallback<T>);
    fn do_something(&self);
}

pub struct MyStruct<T: CallbackData> {
    callbacks: Vec<MyCallback<T>>,
    data: [u8; 3],
}

impl<'a> MyTrait<MyCallbackData<'a>> for MyStruct<MyCallbackData<'a>> {
    fn set_callback(&mut self, cb: MyCallback<MyCallbackData<'a>>) {
        self.callbacks.push(cb);
    }

    fn do_something(&self) {

        for cb in &self.callbacks {
            let cb_data = MyCallbackData {
                data: &self.data,
            };

            (cb.callback)(&cb_data);
        }
    }
}
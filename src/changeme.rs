pub trait CallbackData {}

#[derive(Debug)]
pub struct MyCallbackData {
    data: [u8; 3], 
}

impl CallbackData for MyCallbackData {}

pub struct MyCallback<T: CallbackData> {
    pub callback: Box<dyn Fn(&T)>,
}

pub trait MyTrait<T: CallbackData> {
    fn set_callback(&mut self, cb: MyCallback<T>);
    fn do_something(&self);
}

pub struct MyStruct< T: CallbackData > {
    pub callbacks: Vec<MyCallback<T>>,
    pub data: [u8; 3], 
}

impl<'a> MyTrait<MyCallbackData> for MyStruct< MyCallbackData> {
    fn set_callback(&mut self, cb: MyCallback<MyCallbackData>) {
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

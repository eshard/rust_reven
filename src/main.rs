/*
You are trying to store multiple callbacks in MyStruct and call them with a concrete type in the do_something method.
Specializing MyStruct (which is parametrized with the argument type of the closure) introduces the following lifetime issue :

cannot infer an appropriate lifetime for borrow expression due to conflicting
requirements

note: ...so that reference does not outlive borrowed content
note: expected `&MyCallbackData<'a>`
	  found `&MyCallbackData<'_>`

Please detail what is the problem, and how you would deal with it.
*/


trait CallbackData {}

#[derive(Debug)]
struct MyCallbackData<'a> {
    data: &'a [u8],
}

impl<'a> CallbackData for MyCallbackData<'a> {}

struct MyCallback<T: CallbackData> {
    callback: Box<dyn Fn(&T)>,
}

trait MyTrait<T: CallbackData> {
    fn set_callback(&mut self, cb: MyCallback<T>);
    fn do_something(&self);
}

struct MyStruct<T: CallbackData> {
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

fn main() {
    let mut s = MyStruct {
        callbacks: Vec::new(),
        data: [1, 2, 3],
    };

    s.set_callback(MyCallback {
        callback: Box::new(|data: &MyCallbackData| {
            println!("Callback called with data {:?}", data);
        }),
    });

    s.do_something();
}

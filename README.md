Mission
~~~~~~~

You are trying to store multiple callbacks in MyStruct and call them with a concrete type in the do_something method.
Specializing MyStruct (which is parametrized with the argument type of the closure) introduces the following lifetime issue :

cannot infer an appropriate lifetime for borrow expression due to conflicting requirements

note: ...so that reference does not outlive borrowed content
note: expected `&MyCallbackData<'a>`
	  found `&MyCallbackData<'_>`

Goal:
~~~~~

- Please detail what is the problem in a specific text file
- Provide a solution that you would propose to fix it
- Add the patched code and documentation to the repository
- you are only allowed to modify the file src/changeme.rs
- Provide us a git patch or pull request with your work

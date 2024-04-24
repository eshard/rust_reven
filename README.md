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
- Provide us a git patch or pull request with your work


Solution proposé:

Il faut dans un premier temps révisé les déclarations de lifetime pour les aligner correctement, 
Cela implique d'introduire un paramètre de lifetime dans MyTrait et de modifier les implémentations en conséquence. 

Dans un second temps, il faut introduire un autre paramètre de lifetime dans la méthode set_callback 
pour garantir que la lifetime de MyCallback soit au moins aussi longue que celle de MyStruct.

Enfin, il faut introduire une fonction with_scope pour gérer la lifetime de MyStruct. 
Cette fonction garantit que MyStruct reste actif pendant la durée de son utilisation dans un scope donné. 
On notera l'utilisation du bound FnONce, nous permettant de liberer la reférence, une fois que la closure est appliquée.
En gérant explicitement la lifetime de MyStruct, nous évitons les problèmes liés à l'emprunt et à la suppression trop précoce.
Ce patch résout efficacement les problèmes de lifetime et permet au code de se compiler avec succès.
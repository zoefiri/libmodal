# libmodal
_pending name change_

the basic idea for this is a framework for making modal interfaces. Fundamentally, it maps user inputs to handlers, dependent upon a currently selected mode. to facilitate this, resources are stored in a concurrent hashmap _(one resource per resource type)_ and can be concurrently accessed in bind handlers.
This mostly just exists as a really abstracted backbone for drop-in vim/helix/etc style modal editing.

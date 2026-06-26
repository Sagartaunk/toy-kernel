## NOTE:
This project is under construction and may fail to compile
or function properly. Please ignore this repository till this
readme exists.

## NOTE: 
This is supposed to be a learning project and I will only be
implementing a basic toy-kernel for it. This is not supposed 
to become a production grade kernel at any point in time.

## NOTE: 
Any damages that may occur from running or testing this project 
are not my responsibility. This is my first time making a kernel
and I don't know how it works and will be learning as I go.
Please, use or test at your own risk.


## Learning Notes:

This field contains stuff I think is important and will be writing
here for later reference.

-> `#[unsafe(no_mangle)]` This macro is used to tell rust not to 
change the name of the function. Otherwise, `Rust` will convert it
to some arbitrary string to differentiate between the functions.
-> `#[repr(X)]` is used to tell rust to follow a specific semantic 
for a struct. For example: 
  `#[repr(u8)]` formats the fields as `u8` by default.
  `#[repr(transparent)]` formats the fields exactly as written in 
  binary.

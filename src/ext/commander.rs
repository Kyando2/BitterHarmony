use crate::ext::extension::Extension;
use crate::ext::context::Context;


struct Commander {
    listener: fn(Context, Vec<String>)
}

impl Extension for Commander {
    
}
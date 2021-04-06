use crate::Ext::extension::Extension;
use crate::Ext::context::Context;


struct Commander {
    listener: fn(Context, Vec<String>)
}

impl Extension for Commander {
    
}
// https://stackoverflow.com/a/77835585
#[macro_export]
macro_rules! apply_attrib {
    { #!$attr:tt $($it:item)* } => {
        $(
            #$attr
            $it
        )*
    }
}

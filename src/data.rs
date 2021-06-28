#[macro_export]
macro_rules! ctx_data {
    ($($name:ident => $value:ty),* $(,)?) => {
        $(
            pub struct $name;

            impl serenity::prelude::TypeMapKey for $name {
                type Value = $value;
            }
        )*
    };
}

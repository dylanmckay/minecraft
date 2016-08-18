
macro_rules! define_composite_type {
    ($name:ident => [
         $( $field:ident: $ty:ty),+ ]) => {
        #[derive(Clone, Debug)]
        pub struct $name
        {
            $(
                pub $field: $ty,
            )*
        }

        impl ::protocol::ReadableType for $name
        {
            fn read(read: &mut ::std::io::Read)
                -> Result<Self, ::protocol::Error> {
                Ok($name {
                    $(
                        $field: ::protocol::types::ReadableType::read(read)?,
                    )*
                })
            }
        }

        impl ::protocol::WritableType for $name
        {
            fn write(&self, write: &mut ::std::io::Write)
                -> Result<(), ::protocol::Error> {

                $( self.$field.write(write)?; )*

                Ok(())
            }
        }

        impl ::protocol::Type for $name { }
    }
}


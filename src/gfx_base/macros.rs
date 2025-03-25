#[macro_export]
macro_rules! define_atomic_id {
    ($atomic_id_type:ident) => {
        #[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
        pub struct $atomic_id_type(core::num::NonZero<u32>);

        impl $atomic_id_type {
            #[expect(
                clippy::new_without_default,
                reason = "Implementing the `Default` trait on atomic IDs would imply that two `<AtomicIdType>::default()` equal each other. By only implementing `new()`, we indicate that each atomic ID created will be unique."
            )]
            pub fn new() -> Self {
                use core::sync::atomic::{AtomicU32, Ordering};

                static COUNTER: AtomicU32 = AtomicU32::new(1);

                let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
                Self(core::num::NonZero::<u32>::new(counter).unwrap_or_else(|| {
                    panic!(
                        "The system ran out of unique `{}`s.",
                        stringify!($atomic_id_type)
                    );
                }))
            }
        }

        impl From<$atomic_id_type> for core::num::NonZero<u32> {
            fn from(value: $atomic_id_type) -> Self {
                value.0
            }
        }

        impl From<core::num::NonZero<u32>> for $atomic_id_type {
            fn from(value: core::num::NonZero<u32>) -> Self {
                Self(value)
            }
        }
    };
}

#[macro_export]
macro_rules! define_gfx_type {
    ($downcast_type:ident, $downcast_type_id: ident, $downcast_type_trait: ident) => {
        use downcast::downcast;

        #[derive(Debug)]
        pub struct $downcast_type {
            id: $downcast_type_id,
            instance: Box<dyn $downcast_type_trait>,
        }

        downcast!(dyn $downcast_type_trait);

        impl PartialEq for $downcast_type {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl $downcast_type {
            pub fn new<T: $downcast_type_trait>(instance: T) -> Self {
                $downcast_type {
                    instance: Box::new(instance),
                    id: $downcast_type_id::new(),
                }
            }

            pub fn downcast<T: $downcast_type_trait>(self) -> Option<Box<T>> {
                let value: Option<Box<T>> = self.instance.downcast::<T>().ok();
                value
            }

            pub fn downcast_ref<T: $downcast_type_trait>(&self) -> Option<&T> {
                let value: Option<&T> = self.instance.downcast_ref::<T>().ok();
                value
            }
        }
    };
}

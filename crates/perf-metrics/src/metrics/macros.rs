#[allow(unused_macros)]
macro_rules! define_record_with_elapsed_time_function {
    ($name:ident, $field:ident, $time_counter:ident) => {
        pub(super) fn $name(&mut self) -> Instant {
            // Calculate duration and reset time_counter.
            let now = Instant::now();
            let cycles = now.checked_cycles_since(self.$time_counter).unwrap_or(0);
            self.$time_counter = now;
            // Record duration.
            self.$field = self.$field.checked_add(cycles).expect("overflow");
            now
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_record_time_function {
    ($name:ident, $field:ident, $time_counter:ident) => {
        pub(super) fn $name(&mut self) {
            // Calculate duration.
            let cycles = Instant::now().checked_cycles_since(self.$time_counter).expect("overflow");
            // Record duration.
            self.$field = self.$field.checked_add(cycles).expect("overflow");
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_record_size_function {
    ($name:ident, $field:ident) => {
        pub(super) fn $name(&mut self, size: usize) {
            self.$field = self.$field.checked_add(size).expect("overflow");
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_start_functions {
    ($start_fn:ident, $start_field:ident) => {
        pub(super) fn $start_fn(&mut self) {
            self.$start_field = Instant::now();
        }
    };
}

// #[cfg(feature = "enable_execution_duration_record")]
#[allow(unused_macros)]
macro_rules! impl_write_macro {
    ($struct_name:ident, $start_record_fn:ident, $record_upsert_time_fn:ident, $record_size_fn:ident, $append_time_fn:ident, $write_size_fn:ident) => {
        #[cfg(feature = "enable_execution_duration_record")]
        pub struct $struct_name(usize);

        #[cfg(feature = "enable_execution_duration_record")]
        impl $struct_name {
            pub fn new(size: usize) -> Self {
                $start_record_fn();
                Self(size)
            }
        }

        #[cfg(feature = "enable_execution_duration_record")]
        impl Drop for $struct_name {
            fn drop(&mut self) {
                $record_upsert_time_fn();
                $record_size_fn(self.0);
            }
        }
    };
}

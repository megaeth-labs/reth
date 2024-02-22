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

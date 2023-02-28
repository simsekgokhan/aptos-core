/// Module providing debug functionality.
module std::ol_debug {
    native public fun print<T>(x: &T);

    native public fun print_stack_trace();
}
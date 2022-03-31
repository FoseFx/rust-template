#![deny(clippy::all)]
#![deny(
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::filetype_is_file,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::inline_asm_x86_att_syntax,
    clippy::integer_division,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::string_to_string,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
)]
#![warn(clippy::pedantic)]

fn main() {
    println!("Hello, {}!", sample());
}

fn sample() -> String {
    "sample".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(sample(), "sample".to_string());
    }
}

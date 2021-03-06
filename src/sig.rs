/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
extern "C" {
    pub fn signal_name_to_number(name: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
    pub fn signal_number_to_name(signo: ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
    pub fn print_given_signals(argc: ::std::os::raw::c_int,
                               argv: *const *const ::std::os::raw::c_char,
                               max_line: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn strtosig(s: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
    pub fn pretty_print_signals();
    pub fn unix_print_signals();
}

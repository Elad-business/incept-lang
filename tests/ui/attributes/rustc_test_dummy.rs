//@ compile-flags: --test

fn main() {}

#[rustc_test_dummy]
//~^ ERROR use of an internal attribute
//~| NOTE the `#[rustc_test_dummy]` attribute is an internal implementation detail that will never be stable
//~| NOTE `#[rustc_test_dummy]` attribute is used internally to track tests
fn foo() {}


#[test]
#[rustc_test_dummy]
//~^ ERROR use of an internal attribute
//~| NOTE the `#[rustc_test_dummy]` attribute is an internal implementation detail that will never be stable
//~| NOTE `#[rustc_test_dummy]` attribute is used internally to track tests
fn bar() {}

#[test]
#[rustc_test_dummy]
//~^ ERROR use of an internal attribute
//~| NOTE the `#[rustc_test_dummy]` attribute is an internal implementation detail that will never be stable
//~| NOTE `#[rustc_test_dummy]` attribute is used internally to track tests
#[test]
//~^ WARN duplicated attribute
//~| NOTE `#[warn(duplicate_macro_attributes)]` on by default
fn bazz() {}

fn main() {}

#[defmt_test_macros::tests]
mod tests {
    #[teardown]
    fn hello(a: i32, b: i32) -> i32 {
        a + b
    }
}

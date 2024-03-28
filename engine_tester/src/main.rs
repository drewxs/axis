use axis::custom_errors::Errors;

fn main() {
    axis::logger::init();
    error_test(1);
}

fn error_test(num: i32) -> Result<(), Errors> {
    if num == 1 {
        Err(Errors::TestError.into());
    }
    Ok(())
}

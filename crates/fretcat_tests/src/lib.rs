#[cfg(test)]
mod tests {
    use fretcat_derive::Control;

    #[test]
    fn control_macro() {
        #[derive(Control)]
        struct TestStruct {}

        TestStructControl::print();
    }
}

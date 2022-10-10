#[macro_export]
macro_rules! val {
    (B => $bval:expr) => {{
        AttributeValue::Bool($bval)
    }};
    (L => $val:expr) => {{
        AttributeValue::L($val)
    }};
    (S => $val:expr) => {{
        AttributeValue::S($val)
    }};
    (M => $val:expr) => {{
        AttributeValue::M($val)
    }};
}

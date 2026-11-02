fn real() -> i32 { 1 }

#[cfg(test)]
mod tests {
    fn helper() -> i32 { 2 }
}

#[cfg(test)]
fn only_test() {}

fn main() {}

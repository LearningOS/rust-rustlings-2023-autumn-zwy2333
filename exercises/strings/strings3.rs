// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let start=get_start(input);
    let end  =get_end(input)+1;
    input[start..end].to_string()
    //input.to_string().trim().to_string()
}
fn get_start(input: &str)->usize{
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            return i;
        }
    }
    input.len()
}
fn get_end(input: &str)->usize{
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate().rev() {
            if item != b' ' {
                return i;
            }
    }
    input.len()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let app=String::from(" world!");
    input.to_owned()+&app
    
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars","balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}

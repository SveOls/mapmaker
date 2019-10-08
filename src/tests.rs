#[cfg(test)]
mod tests {
    use super::super::*;
    #[test]
    fn test_open_provinces() {
        if let Err(e) = open_file("provinces".to_string()) { 
            panic!("Error: {}", e) 
        }
    }
    #[test]
    fn test_open_sea() {
        if let Err(e) = open_file("states".to_string()) { 
            panic!("Error: {}", e) 
        }
    }
}
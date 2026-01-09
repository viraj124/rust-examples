pub fn search(file: &str, query: &str) -> Vec<String> {
    // let mut results = Vec::new();
    file.lines().filter(|line| line.contains(query)).map(|line| line.to_string()).collect()
}

pub fn search_case_insensitive(file: &str, query: &str) -> Vec<String> {
    // let mut results = Vec::new();

    file.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).map(|line| line.to_string()).collect()

    // let query = query.to_lowercase();
    // for line in file.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line.to_string());
    //     }
    // }
    // results
}
    // vec![]

    // file.lines()
    //     .filter(|line| line.contains(query))
    //     .map(|line| line.to_string())
    //     .collect()
// pub fn search<'a>(file: &'a str, query: &str) -> Vec<&'a str> {


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        let file = "hello world\nhello rust\nhello\n";
        let query = "hello";
        assert_eq!(vec!["hello world", "hello rust", "hello"], search(file, query));
    }
}
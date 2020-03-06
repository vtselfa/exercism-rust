pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mid = |a:&str, b:&str| format!("For want of a {} the {} was lost.", a, b);
    let end = |a:&str | format!("And all for the want of a {}.", a);

    let mut result: Vec<String> = vec![];
    for (a, b) in list.iter().zip(list[1..].iter())
    {
        result.push(mid(a, b));
    }

    result.push(end(list[0]));
    result.join("\n")    
}

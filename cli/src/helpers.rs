pub fn convert_str_usize(input: Option<&str>) -> Result<usize, String> {
    Ok(match input {
        Some(num) => match num.parse() {
            Ok(num_usize) => num_usize,
            Err(err) => return Err(err.to_string())
        },
        None => return Err("please enter the lernsetId".to_string())
    })
}

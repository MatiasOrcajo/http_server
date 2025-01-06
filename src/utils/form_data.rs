
pub fn extract_form_data_from_http_request(mut formatted_http_request: Vec<String>) -> Result<String, String> {
    let mut sliced_vector = Vec::new();

    if let Some(index) = formatted_http_request.iter().position(|content| content == "Content-Length:") {
        // Modifica el vector original directamente
        sliced_vector = formatted_http_request.drain(index + 2..).collect();
    }

    let mut form_data_in_string = String::new();
    
    for value in sliced_vector.iter() {
        form_data_in_string += &format!("{} ", value);
    }

    println!("{}", form_data_in_string);

    Ok(form_data_in_string)
}

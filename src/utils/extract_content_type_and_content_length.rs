pub fn extract_content_type_and_content_length_from_http_request (formatted_http_request: Vec<String>) -> (String, String) {

    let mut content_type: String = String::new();
    let mut content_length: String = String::new(); 


    for (index, content) in formatted_http_request.iter().enumerate() {
        if content == "Content-Type:" {
            
            content_type = format!(
                "{} {}",
                &formatted_http_request[index + 1],
                &formatted_http_request[index + 2]
            );
        }

        if content == "Content-Length:" {
            
            content_length = format!("{}", &formatted_http_request[index + 1]);
        }
    }

    (content_type, content_length)
        
}
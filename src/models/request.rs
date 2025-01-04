use crate::utils::form_data::extract_form_data_from_http_message;

#[allow(dead_code)]
#[derive(Debug)]

pub struct Request {
    pub method: Result<String, String>,
    pub request_target: Result<String, String>,
    
}

/// Creates a vector from a given TCP connection content
/// `content` - TCP connection content
/// 
/// Example of given content is in request_example.txt
/// 
fn convert_tcp_to_vector(content: &str) -> Vec<String> {
    
    content.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
}


/// Returns an specific index from convert_tcp_to_vector,
/// throws specific error message
/// 
/// `index` - index to search in vector
/// `error_message` - specific message to be throw if failing
/// `content` - TCP connection content
/// 
fn extract_from_vector(index: u8, error_message: &str, content: &str) -> Result<String, String> {
    
    let tcp_connection = convert_tcp_to_vector(content);

        tcp_connection.get(index as usize)
            .map(|s| s.to_string())
            .ok_or_else(|| error_message.to_string())
}

impl Request {
    
    pub fn new(content: String) -> Result<Request, String> {
        
        Request::extract_form_data(&content);
        
        Ok(
            Request {
                method: Request::extract_method(&content),
                request_target: Request::extract_request_target(&content)
            }
        )
    }


    pub fn extract_method(content: &str) -> Result<String, String> {
        
        extract_from_vector(0, "Method not found in content", content)
    }


    pub fn extract_request_target(content: &str) -> Result<String, String> {
        
        extract_from_vector(1, "Request target not found in content", content)
    }


    pub fn extract_form_data (content: &str) -> Result<String, String>{
        
        let formatted_content = convert_tcp_to_vector(content);
        extract_form_data_from_http_message(formatted_content);
        Ok("".to_string())

    }
    
}
use std::collections::HashMap;

use crate::utils::{extract_content_type_and_content_length::extract_content_type_and_content_length_from_http_request, form_data::extract_form_data_from_http_request};

#[allow(dead_code)]
#[derive(Debug)]

pub struct Request {
    pub method: Result<String, String>,
    pub request_target: Result<String, String>,
    pub form_data: HashMap<String, String>,
    pub http_user_agent: Result<String, String>,
    pub content_type: String,
    pub content_length: String
    
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

        let (content_type, content_length) = extract_content_type_and_content_length_from_http_request(convert_tcp_to_vector(&content));

        Ok(
            Request {
                method: Request::extract_method(&content),
                request_target: Request::extract_request_target(&content),
                form_data: Request::extract_form_data(&content),
                http_user_agent: Request::extract_http_user_agent(&content),
                content_type: content_type,
                content_length: content_length
            }
        )
    }


    pub fn extract_method(content: &str) -> Result<String, String> {
        
        extract_from_vector(0, "Method not found in content", content)
    }


    pub fn extract_request_target(content: &str) -> Result<String, String> {
        
        extract_from_vector(1, "Request target not found in content", content)
    }


    pub fn extract_form_data (content: &str) -> HashMap<String, String>{
        
        let formatted_content = convert_tcp_to_vector(content);
        
        return extract_form_data_from_http_request(formatted_content);

    }


    pub fn extract_http_user_agent (content: &str) -> Result<String, String> {

        extract_from_vector(4, "HTTP user agent not found in content", content)
    }

    
}
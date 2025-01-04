use std::collections::HashMap;

pub fn extract_form_data_from_http_request(formatted_http_request: Vec<String>) -> HashMap<String, String>{
    
    let mut form_data: HashMap<String, String> = HashMap::new();
    

    for (index, content) in  formatted_http_request.iter().enumerate(){
        if content == "form-data;" {

            let input: &str = &formatted_http_request[index + 1];
            
            if let Some((_, rest)) = input.split_once("name=\"") {
                if let Some((extracted, _)) = rest.split_once('"') {
                    
                    form_data.insert( String::from(extracted), String::from(&formatted_http_request[index + 2]));

                }
            }   
        }
    }


    return form_data

}
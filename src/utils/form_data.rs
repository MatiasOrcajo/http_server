use std::collections::HashMap;

pub fn extract_form_data_from_http_message(formatted_content: Vec<String>) -> HashMap<String, String>{
    
    let mut form_data: HashMap<String, String> = HashMap::new();
    

    for (index, content) in  formatted_content.iter().enumerate(){
        if content == "form-data;" {

            let input: &str = &formatted_content[index + 1];
            
            if let Some((_, rest)) = input.split_once("name=\"") {
                if let Some((extracted, _)) = rest.split_once('"') {
                    
                    form_data.insert( String::from(extracted), String::from(&formatted_content[index + 2]));

                }
            }   
        }
    }


    form_data

}
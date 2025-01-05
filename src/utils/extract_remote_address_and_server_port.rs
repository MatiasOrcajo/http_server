pub fn extract_remote_address_and_server_port(formatted_http_request: &Vec<String>) -> (String, String) {

    let mut client_ip_address = String::new();
    let mut server_port = String::new();

    for (index, content) in  formatted_http_request.iter().enumerate() {
        if content == "Host:" {
            
            client_ip_address = String::from(&formatted_http_request[index + 1]);
            let aux = &client_ip_address;

            if let Some((_, rest)) = aux.split_once(":") {
                server_port = String::from(rest);    
            }
        }
    }

    (client_ip_address, server_port)
}
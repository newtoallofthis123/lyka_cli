use crate::types::AddCode;

const URL: &str = "https://www.noobscience.rocks/api/code";

fn get_client()-> reqwest::Client{
    let client = reqwest::Client::new();
    return client;   
}

//* Simple Function to convert the response text to JSON */
fn convert_to_json(txt: &str)-> serde_json::Value{
    //Use serde_json to convert the response to JSON
    let json = serde_json::from_str(txt).unwrap();
    return json;
}

pub async fn add_code(code: AddCode) -> String{
    let client = get_client();

    // a hashmap of the data we want to send
    let mut form_data = std::collections::HashMap::new();
    form_data.insert("title", code.title);
    form_data.insert("author", code.author);
    form_data.insert("content", code.content);
    form_data.insert("lang", code.lang);

    // send the data to the server

    let pb = crate::cli::spinner();
    let response = client.post(URL.clone().to_string() + "/add").json(&form_data).send().await.unwrap();
    pb.finish_with_message("Done!");

    // get the response text
    let json = convert_to_json(&response.text().await.unwrap());
    let returned_data = json.as_object().unwrap();

    return returned_data.get("hash").unwrap().as_str().unwrap().to_string();
}
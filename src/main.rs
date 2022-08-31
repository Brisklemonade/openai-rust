#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_token = fetch_api_key(".env");

    let client = openai_api::Client::new(&api_token);
    let prompt = "Once apon a time";
    let completion_args = openai_api::api::CompletionArgs::builder()
        .prompt(prompt)
        .engine("davinci")
        .max_tokens(20)
        .temperature(0.7)
        .top_p(0.9)
        .stop(vec!["\n".into()])
        .build()
        .expect("Failed to create CompletionArgs struct");

    let completion = client.complete_prompt(completion_args).await?;

    println!("{} {}", prompt, completion);

    Ok(())
}

fn fetch_api_key(file_name: &str) -> String {
    let err = format!("Couldn't read file: {}", file_name);
    let contents = std::fs::read_to_string(".env").expect(err.as_str());

    let contents_vec = contents.split_ascii_whitespace().collect::<Vec<_>>();
    let api_key = match contents_vec.get(2) {
        Some(key) => key.to_owned(),
        None => panic!("Failed to retrieve api key from file"),
    };

    api_key.to_owned()
}

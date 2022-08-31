# OpenAI Rust Sandbox

If you have access to open api and would like to use this, you will need to provide your api key. To do so, create a `.env` file at the root of the project and paste in the following:

```toml
OPENAI_API_KEY = sk-YOUR_API_KEY
```

The helper function will then retrieve your key and use it for the client builder.

Refer [here](https://github.com/deontologician/openai-api-rust) for documentation on the **openai_api**

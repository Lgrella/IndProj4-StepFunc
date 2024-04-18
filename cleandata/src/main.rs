// Import necessary modules
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

// Define a struct to represent the input data
#[derive(Deserialize, Serialize)]
struct Input {
    data: String, // Field to hold the input data
}

// Define a struct to represent the output data
#[derive(Deserialize, Serialize)]
struct Output {
    capitalized: String, // Field to hold the processed result
}

// Main function, entry point of the program
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(process_data); // Create a handler function from the process_data function
    lambda_runtime::run(func).await?; // Run the Lambda function
    Ok(()) // Return Ok if everything executed successfully
}


// Function to process the input data
async fn process_data(event: Input, _ctx: Context) -> Result<Output, Error> {
    let capitalized = capitalize_words(&event.data); // Capitalize the first letter of every word
    Ok(Output { capitalized }) // Return the processed result
}

// Function to capitalize the first letter of every word in a sentence
fn capitalize_words(sentence: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in sentence.chars() {
        if capitalize_next && ch.is_alphabetic() {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }

        if ch.is_whitespace() {
            capitalize_next = true;
        }
    }

    result
}

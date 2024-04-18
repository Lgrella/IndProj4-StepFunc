// Import necessary modules
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

// Define a struct to represent the input data
#[derive(Deserialize, Serialize)]
struct Input {
    capitalized: String, // Field to hold the input data
}

// Define a struct to represent the output data
#[derive(Deserialize, Serialize)]
struct Output {
    count: i32, // Field to hold the processed result
}

// Main function, entry point of the program
#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(count_names); // Create a handler function from the process_data function
    lambda_runtime::run(func).await?; // Run the Lambda function
    Ok(()) // Return Ok if everything executed successfully
}

// Function to process the input data
async fn count_names(event: Input, _ctx: Context) -> Result<Output, Error> {
    let count = count_names_input(&event.capitalized); // Capitalize the first letter of every word
    Ok(Output { count }) // Return the processed result
}

// Function to count words in a sentence return a number
fn count_names_input(sentence: &str) -> i32 {
    let words = sentence.split_whitespace(); // Split the sentence into words
    let mut count = 0; // Initialize a counter to count the words

    for _word in words {
        count += 1; // Increment the counter if the first letter is uppercase
    }

    count // Return the count of words with an uppercase first letter
}

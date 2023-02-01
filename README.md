# rust-mini-week2:
A Rust Mini Project command-line tool that draws a random Tarot card and tells fortune. 

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [rust-marco-polo-example](https://github.com/noahgift/rust-mlops-template/tree/main/MarcoPolo)

## Usage
<code>cargo run -- tarot</code>
 
 The command line tool uses the subcommand "<code>tarot</code>" and generates a random number between 0-57, representing a random card from the 58 cards deck. Then, the tool parses the json for tarot cards, finds its name and selects a random fortune-telling for the specific card.

## Learning
Learned json processing for Rust (using serde_json to unwrap in each level), as well as random number generator. 

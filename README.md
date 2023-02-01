# rust-mini-week1:
A Rust Mini Project command-line tool that converts an integer to Roman literals. 

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [rust-marco-polo-example](https://github.com/noahgift/rust-mlops-template/tree/main/MarcoPolo)

## Usage
<code>cargo run -- convert --num 34</code>
 
 The command line tool uses the subcommand "<code>convert</code>" and takes in one argument, a <code>u32</code> integer named "<code>num</code>," as the above examples shows. The result should be outputted as "XXXIV"

## Test
Run <code>make test</code> and the four tests written under <code>/tests</code> directory will run. 

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[grammar = "fun_grammar.pest"]
pub struct FunParser;

fn main() {
    let indentation = "  "; // 2 spaces

    let fun_code =
        r#"
fun hello() {
  foo = "bar"
  bar mutable = "chan"

  .log "nice www"
  .error "not-nice www"
}
    "#;

    let parsed = FunParser::parse(Rule::fun_file, &fun_code).unwrap();
    // let parsed = FunParser::parse(Rule::fun_file, &fun_code);
    //   match parsed {
    //       Ok(p) => p,
    //       Err(e) => {
    //           eprintln!("Parsing error: {:?}", e);
    //           std::process::exit(1);
    //       }
    // };

    // println!("{:#?}", parsed);
    // for pair in parsed {
    //     println!("Rule: {:?}, Content: {}", pair.as_rule(), pair.as_str());
    // }
    let mut js_code = String::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::fun_declaration => {
                let mut inner_pairs: Vec<_> = pair.into_inner().collect();
                let function_name = inner_pairs[0].as_str();
                let mut function_body = String::new();

                for inner_pair in &inner_pairs[1..] {
                    // println!("Processing inner pair: {:?}", inner_pair);  // Debug print

                    match inner_pair.as_rule() {
                        Rule::statement => {
                            for statement_inner_pair in inner_pair.clone().into_inner() {
                                match statement_inner_pair.as_rule() {
                                    Rule::console_log => {
                                        let log_content = statement_inner_pair
                                            .clone()
                                            .into_inner()
                                            .next()
                                            .unwrap()
                                            .as_str();
                                        println!("Found console_log with content: {}", log_content); // Debug print
                                        function_body.push_str(
                                            &format!(
                                                "{}console.log({})\n",
                                                indentation,
                                                log_content
                                            )
                                        );
                                    }
                                    Rule::console_error => {
                                        let error_content = statement_inner_pair
                                            .clone()
                                            .into_inner()
                                            .next()
                                            .unwrap()
                                            .as_str();
                                        println!("Found console_error with content: {}", error_content); // Debug print
                                        function_body.push_str(
                                            &format!(
                                                "{}console.error({})\n",
                                                indentation,
                                                error_content
                                            )
                                        );
                                    }
                                    Rule::line_jump => {
                                        println!("Found line jump"); // Debug print
                                        function_body.push_str("\n");
                                    }
                                    Rule::const_ass => {
                                        println!(
                                            "statement_inner_pair {}",
                                            statement_inner_pair.clone().into_inner().as_str()
                                        );
                                        // let mut inner_pairs_c: Vec<_> = statement_inner_pair;
                                        let const_name = statement_inner_pair
                                            .clone()
                                            .into_inner()
                                            .as_str();

                                        // let mut const_body = String::new();

                                        // let error_content = statement_inner_pair
                                        //     .clone()
                                        //     .into_inner()
                                        //     .next()
                                        //     .unwrap()
                                        //     .as_str();
                                        println!("Found const with content: "); // Debug print
                                        function_body.push_str(
                                            &format!("{}const {}\n", indentation, const_name)
                                        );
                                    }
                                    Rule::let_ass => {
                                        let mut let_inner = statement_inner_pair
                                            .clone()
                                            .into_inner();
                                        let var_name = let_inner.next().unwrap().as_str();
                                        let var_value = let_inner.next().unwrap().as_str();
                                        println!(
                                            "Found let assignment: {} = {}",
                                            var_name,
                                            var_value
                                        ); // Debug print
                                        function_body.push_str(
                                            &format!(
                                                "{}let {} = {}\n",
                                                indentation,
                                                var_name,
                                                var_value
                                            )
                                        );
                                    }

                                    _ => {
                                        println!(
                                            "Skipping statement inner pair: {:?}",
                                            statement_inner_pair
                                        ); // Debug print
                                    }
                                }
                            }
                        }
                        _ => {
                            println!("Skipping inner pair: {:?}", inner_pair); // Debug print
                        }
                    }
                }

                js_code.push_str(
                    &format!("function {}() {{\n{}}}\n", function_name, function_body)
                );
            }
            _ => {}
        }
    }

    println!("Generated JS Code:\n{}", js_code);

    // Save the transformed code to a .js file
    let mut file = File::create("output.js").expect("Unable to create file");
    file.write_all(js_code.as_bytes()).expect("Unable to write data");
    println!("JavaScript code saved to output.js");
}

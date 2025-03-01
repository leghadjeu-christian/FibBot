use fib::fib;
use get_pr::get_pr_body;
use parse_numbers::extract_numbers;
use post::post_comment;
use std::env::{self, args};

mod fib;
mod get_pr;
mod parse_numbers;
mod post;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    println!("{:?}", args);
    if args.is_empty() {
        println!("No arguments supplied.");
        return;
        // } else if args.len() != 2 {
        //     println!("FibBot requires exactly two parameters.");
        //     return;
    }

    let enable_fib = args[0].to_lowercase() == "true";
    let max_threshold = args[1].parse().expect("Failed parsing the max_threshold");
    let owner = &args[4];
    let pr_number = args[3].parse().expect("Failed to parse pr_number");
    let token = &args[2];
    let repo = &args[5];
    let repo = repo.split("/").collect::<Vec<&str>>();
    let repo = repo[1];

    let pr_content = get_pr_body(&owner, &repo);
    // println!("{:?}", result);
    let result: &str = &pr_content.unwrap();
    let vec_of_nums = extract_numbers(result);
    let mut comments = String::new();

    for num in vec_of_nums {
        if num > max_threshold {
            continue;
        } else {
            let fib_numb = fib(num);
             let comment = format!("The fibonnaci number of {} is  : {}", num,fib_numb);

            comments.push_str(format!("{}\n", comment).as_str());

            println!("{:#?}", result);
        }
    }
    let result = post_comment(
        &owner.to_string(),
        &repo.to_string(),
        pr_number,
        token.to_string(),
        comments,
    );
}

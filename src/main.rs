use regex::Regex;

fn main() {
   let re = Regex::new(r"^/demo/[^/]+$").unwrap();
   let message = "/demo/as";

   dbg!(re.is_match(message));
}

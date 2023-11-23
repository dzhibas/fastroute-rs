use regex::Regex;

fn main() {
   let re = Regex::new(r"(?x)^(?:
                (?<r1>/foo/([^/]+)/(\d+))
              | (?<r2>/bar/(?<id>\d+))
              | (?<r3>/foi/([^/]+))
            )$").unwrap();
   let route: &str = "/foi/33";
   let cap = re.captures(route).unwrap();
   dbg!(&cap);
   for c in cap.iter().skip(1) {
      if c.is_some() {
         let b = c.unwrap();
         dbg!(b);
      }
   }
   let matches: Vec<_> = re.find_iter(route).map(|m| m.as_str()).collect();
   dbg!(matches);
}

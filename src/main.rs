use regex::Regex;

fn main() {
   /// idea is to build one single regex pattern 
   /// and find route in one shot
   ///
   /// we need to compare it with axum-router, RegexSet as both of them regex based
   /// and fo speed check with matchit as that one fastest
   let re = Regex::new(r"(?sxm)^(?:
                (?<r1>/foo/([^/]+)/(\d+))
              | (?<r2>/bar/(?<id>\d+))
              | (?<r3>/foi/([^/]+))
            )$").unwrap();
   let route: &str = "/foi/33";
   let cap = re.captures(route).unwrap();
   let cap_names = re.capture_names();
   dbg!(cap_names);
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

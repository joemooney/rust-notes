use regex::Regex;
use std::fs;

fn main() {
    println!("Generating Table of Contents graph");
    let path = "md/SUMMARY.md";
    let contents = fs::read_to_string(path).expect(&format!("Failed to find {}", path));

    //let re = Regex::new(r#"(?P<spaces>.*)- \[(?P<name>.*)\]\((?P<link>.*)\)"#).unwrap();
    let re = Regex::new(r#"([ ]*)- \[(.*)\]\((.*).md\)"#).unwrap();

    println!("{}", contents);
    println!("* [[../index.html Book]]");
    for line in contents.lines() {
      if re.is_match(line) {
         let caps = re.captures(line).unwrap();
         let spaces = &caps[1];
         let name = &caps[2];
         let hlink = &caps[3];
         //println!("<{}><{}><{}>", spaces, name, hlink);
         let level = "*".repeat(spaces.len() + 2);
                                                                                                                      
         // line.replace(&line, format)
         println!("{} [[.{}.html {}]]", level, hlink, name);
      }
    }
}

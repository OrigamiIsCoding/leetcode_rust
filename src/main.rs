use tera::Tera;

fn main() {
    let tera = match Tera::new("templates/solution_template.rs") {
        Err(e) => {
            panic!("Parsing error(s): {}",e)
        },
        Ok(t) => t,
    };
}

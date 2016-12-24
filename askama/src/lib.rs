#![feature(proc_macro)]

#[macro_use]
extern crate askama_codegen;

pub trait Template {
    fn render(&self) -> String;
}

#[cfg(test)]
mod tests {

    extern crate askama;

    #[derive(Template)]
    struct TestTemplate {
        var: String,
    }

    #[test]
    fn it_works() {
        let s = TestTemplate { var: "foo".to_string() }.render();
        assert_eq!(s, "hello world, foo");
    }

}
extern crate di;
extern crate metafactory;

fn main() {
    let mut registry = di::registry::Registry::new();

    registry.insert_one("a", 5i);
    registry.insert_one("b", 4i);

    registry
        .one(
            "sum",
            |a: int, b: int|
            a + b
        )
        .with_args(&["a", "b"])
        .insert();

    registry
        .one(
            "difference",
            |a: int, b: int|
            a - b
        )
        .with_args(&["a", "b"])
        .insert();

    registry
        .one_of(
            "results",
            |sum: int, difference: int| {
                vec![sum, difference]
            }
        )
        .with_args(&["sum", "difference"])
        .insert();

    registry
        .one(
            "result_view",
            |results: Vec<Vec<int>>| {
                format!("{}, {}", results[0][0], results[0][1])
            }
        )
        .with_arg("results")
        .insert();

    match registry.compile() {
        Ok(container) => {
            let view = container.get::<String>("result_view").unwrap();
            println!("result: {}", view.take());
        },
        Err(errors) => {
            di::error_printer::pretty_print(&errors);
        }
    }
}
use super::TableRenderable;

pub fn render<T: TableRenderable>(item: &T) {
    render_list(std::slice::from_ref(item));
}

pub fn render_list<T: TableRenderable>(items: &[T]) {
    // Print header
    println!("{}", T::headers().join("\t"));
    for item in items {
        println!("{}", item.text_row().join("\t"));
    }
}

#[derive(Serialize)]
pub struct ItemContext<T> {
    pub item: T,
}

#[derive(Serialize)]
pub struct ListContext<T> {
    pub items: Vec<T>,
}

#[derive(Serialize)]
pub struct NoContext {}

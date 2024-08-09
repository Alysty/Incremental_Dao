#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub qi: u128
}
impl Player {
    pub fn temp(&self) -> tera::Context {
        let mut c = tera::Context::new();
        c.insert("name", &self.name);
        c.insert("qi", &self.qi);
        return c;
    }
}

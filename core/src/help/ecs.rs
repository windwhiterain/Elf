pub trait Attach<Component> {
    fn get(&self) -> &Component;
}
pub trait GenericAttach<Component> {
    fn get(&self) -> &Component;
}
pub trait Entity {
    fn comp<Component>(&self) -> &Component
    where
        Self: Attach<Component>;
    fn g_comp<Component>(&self) -> &Component
    where
        Self: GenericAttach<Component>;
}
impl<T> Entity for T {
    fn comp<Component>(&self) -> &Component
    where
        Self: Attach<Component>,
    {
        self.get()
    }
    fn g_comp<Component>(&self) -> &Component
    where
        Self: GenericAttach<Component>,
    {
        self.get()
    }
}

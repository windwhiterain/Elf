pub trait Attach<Component> {
    fn get(&self) -> &Component;
    fn get_mut(&mut self) -> &mut Component;
}
pub trait GenericAttach<Component> {
    fn get(&self) -> &Component;
    fn get_mut(&mut self) -> &mut Component;
}
pub trait Entity {
    fn comp<Component>(&self) -> &Component
    where
        Self: Attach<Component>;
    fn g_comp<Component>(&self) -> &Component
    where
        Self: GenericAttach<Component>;
    fn comp_mut<Component>(&mut self) -> &mut Component
    where
        Self: Attach<Component>;
    fn g_comp_mut<Component>(&mut self) -> &mut Component
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
    fn comp_mut<Component>(&mut self) -> &mut Component
    where
        Self: Attach<Component>,
    {
        self.get_mut()
    }
    fn g_comp_mut<Component>(&mut self) -> &mut Component
    where
        Self: GenericAttach<Component>,
    {
        self.get_mut()
    }
}

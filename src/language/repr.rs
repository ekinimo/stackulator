pub trait Representation<ReprType, Ctx> {
    fn get_repr(&self, context: &Ctx) -> String;
}

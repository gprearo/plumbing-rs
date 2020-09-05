pub trait Converter<Tin, Tout> {
    fn convert(data: Tin) -> Tout ;
}
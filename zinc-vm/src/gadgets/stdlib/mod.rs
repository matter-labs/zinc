pub mod crypto;

//pub struct StdLib<E: ZincEngine> {
//    sha256: Sha256,
//    pedersen: Pedersen<E>,
//}
//
//impl<E: ZincEngine> StdLib<E> {
//    fn sha256(&self) -> &Sha256 {
//        &self.sha256
//    }
//    fn pedersen(&self) -> &Pedersen<E> { &self.pedersen }
//}
//
//impl StdLib<Bn256> {
//    fn new() -> Self {
//        Self {
//            sha256: Sha256,
//            pedersen: Pedersen::new(JubjubBn256::new())
//        }
//    }
//}

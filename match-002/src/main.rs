fn main() {

    fn mais_um(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }

    let cinco = Some(5);
    let seis = mais_um(cinco);
    let nenhum = mais_um(None);

}

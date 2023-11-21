
pub mod stp;

pub fn scp(mucape:f64, esrh:f64, ebwd:f64) -> f64 {
    (mucape/1_000.0) * (esrh/50.0) * (ebwd/20.0)
}

pub fn ehi(cape: f64, srh: f64) -> f64 {
    let ehi = (cape * srh)/160_000.0;
    ehi
}
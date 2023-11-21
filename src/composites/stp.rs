/// Calculate the Significant Tornado Parameter for the effective layer.
/// 
/// # Purpose
/// A multiple component index that is meant to highlight the co-existence of ingredients favoring right-moving supercells capable of producing EF2-EF5 tornadoes. 
///
/// # Parameters
/// - `mlcape`: Mixed-Layer Convective Available Potential Energy (J/kg)
/// - `mllcl`: Mixed-Layer Lifting Condensation Level (m)
/// - `esrh`: Effective Storm Relative Helicity (m^2/s^2)
/// - `ebwd`: Effective Bulk Wind Difference (m/s)
/// - `mlcin`: Mixed-Layer Convective inhibition (J/kg)
/// # Returns
/// This function returns the STP values
/// Anything above 1 means the atmosphere at the given point is viable for a tornado of EF2 strength or higher.
/// Violent Tornadoes are likely to appear in STP values of 3-8.
///
/// # Examples
/// ```
/// use::composites::stp::stp
/// 
/// let SigTor = stp(3000.0, 1000.0, 600.0, 30.0, -50.0 );
/// println!("The Significant Tornado Parameter is: {:.1}",  SigTor);
/// ```
pub fn stp(mlcape: f64, mllcl: f64, esrh: f64, ebwd: f64, mlcin: f64) -> f64 {
    return mlcape/1500.0
    * (2000.0 - mllcl.clamp(1000.0, 2000.0))/1000.0
    * (esrh/150.0) 
    * (ebwd.clamp(12.5, 30.0)/20.0) 
    * (200.0 + mlcin.clamp(-200.0, -50.0))/150.0
}






pub fn vtp(mlcape: f64, mllcl: f64, esrh: f64, ebwd: f64, mlcin: f64, mlcape3: f64, lapse3: f64 ) -> f64 {
    //The 0-3 km lapse rate term is set to 2.0 when 0-3 km MLCAPE > 100 J kg-1.
    return stp(mlcape, mllcl, esrh, ebwd, mlcin) * (mlcape3)/50.0 * if mlcape3 > 100.0 { 2.0 } else { lapse3/6.5 }
}
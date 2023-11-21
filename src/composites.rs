//Significant Tornado Parameter (CIN or Effective Layer)
#[allow(dead_code)]
pub fn stpc(mlcape:f64, mut mllcl:f64, esrh:f64, mut ebwd:f64, mut mlcin:f64) -> f64 {
    // The mlLCL term is set to 1.0 when mlLCL < 1000 m, and set to 0.0 when mlLCL > 2000 m;
    if mllcl < 1000.0 {
        mllcl = 1000.0;
    } else if mllcl > 2000.0 {
        mllcl = 2000.0;
    }
    // the mlCIN term is set to 1.0 when mlCIN > -50 J kg-1, and set to 0.0 when mlCIN < -200;
    if mlcin > -50.0 {
        mlcin = -50.0;
    } else if mlcin < -200.0 {
        mlcin = -200.0;
    }
    // the EBWD term is capped at a value of 1.5 for EBWD > 30 m s-1, and set to 0.0 when EBWD < 12.5 m s-1. 
    if ebwd > 30.0 {
        ebwd = 30.0;
    } else if ebwd < 12.5 {
        ebwd = 12.5;
    }
    // Lastly, the entire index is set to 0.0 when the effective inflow base is above the ground.
    let stp = (mlcape/1500.0) * ((2000.0 - mllcl)/1000.0) * (esrh/150.0) * (ebwd/20.0) * ((200.0 + mlcin)/150.0);
    stp
}

// Siginificant Tornado Parameter (fixed layer)
#[allow(dead_code)]
pub fn stp_fixed(sbcape:f64, mut sblcl:f64, srh1:f64, mut bwd6:f64, mut sbcin: f64) -> f64 {
    // The sbLCL term is set to 1.0 when sbLCL < 1000 m, and set to 0.0 when sbLCL > 2000 m;
    if sblcl < 1000.0 {
        sblcl = 1000.0;
    } else if sblcl > 2000.0 {
        sblcl = 2000.0;
    }
    // the sbCIN term is set to 1.0 when sbCIN > -50 J kg-1, and set to 0.0 when sbCIN < -200;
    if sbcin > -50.0 {
        sbcin = -50.0;
    } else if sbcin < -200.0 {
        sbcin = -200.0;
    }
    // the 6BWD term is capped at a value of 1.5 for 6BWD > 30 m s-1, and set to 0.0 when 6BWD < 12.5 m s-1.
    if bwd6 > 30.0 {
        bwd6 = 30.0;
    } else if bwd6 < 12.5 {
        bwd6 = 12.5;
    }

    let stp = (sbcape/1500.0) * ((2000.0-sblcl)/1000.0) * (srh1/150.0) * (bwd6/20.0)* ((200.0+sbcin)/150.0);
    stp
}

// Violent Tornado Parameter
#[allow(dead_code)]
pub fn vtp(mlcape:f64, mut mllcl:f64, esrh:f64, mut ebwd:f64, mut mlcin:f64, ml3cape:f64, mut lr3:f64) -> f64 {
    // The 0-3 km lapse rate term is set to 2.0 when 0-3 km MLCAPE > 100 J kg-1.
    if ml3cape > 100.0 {
        lr3 = 13.0;
    }
    // Like STP, the mlLCL term is set to 1.0 when mlLCL < 1000 m, and set to 0.0 when mlLCL > 2000 m;
    if mllcl < 1000.0 {
        mllcl = 1000.0;
    } else if mllcl > 2000.0 {
        mllcl = 2000.0;
    }
    // the mlCIN term is set to 1.0 when mlCIN > -50 J kg-1, and set to 0.0 when mlCIN < -200;
    if mlcin > -50.0 {
        mlcin = -50.0;
    } else if mlcin < -200.0 {
        mlcin = -200.0;
    }
    // the EBWD term is capped at a value of 1.5 for EBWD > 30 m s-1, and set to 0.0 when EBWD < 12.5 m s-1.
    if ebwd > 30.0 {
        ebwd = 30.0;
    } else if ebwd < 12.5 {
        ebwd = 12.5;
    }
    // Lastly, the entire index is set to 0.0 when the effective inflow base is above the ground.
    let vtp = stpc(mlcape, mllcl, esrh, ebwd, mlcin) * (ml3cape/50.0) * (lr3/6.5);
    vtp
}

// Supercell Composite Parameter
#[allow(dead_code)]
pub fn scp(mucape:f64, esrh:f64, ebwd:f64) -> f64 {
    let scp = (mucape/1000.0) * (esrh/50.0) * (ebwd/20.0);
    scp
}

// Energy Helicity Index
#[allow(dead_code)]
pub fn ehi(cape:f64, srh:f64) -> f64 {
    let ehi = (cape * srh)/160_000.0;
    ehi
}
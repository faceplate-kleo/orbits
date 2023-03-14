pub struct KeplerOrbit {
    pub eccentricity: f64,
    pub semimajor_axis: f64,
    pub inclination: f64,
    pub longitude_an: f64,
    pub arg_periapsis: f64,
    pub true_anomaly: f64

}


pub fn kepler_mean_anomaly(ecc_an: &f64, ecc: &f64) -> f64 {
    //this is just Kepler's Equation, plain and simple
    return ecc_an - (ecc * (ecc_an.sin()));
}


pub fn kepler_eccentric_anomaly(mean_an: &f64, ecc: &f64, n: &i16) -> f64 {
    // A fixed-point, iterative approximation of Kepler's equation for E 
    // n, the number of iterations, depends upon the eccentricity of the orbit
    // This dependency is currently the responsibility of the caller.
    let mut ecc_an: f64 = *mean_an;
    let mut k = 1;

    while k < *n {
        ecc_an = mean_an * (ecc * ecc_an.sin()); 
        k += 1;
    }

    return ecc_an;
}

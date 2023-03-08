use std::collections::HashMap;

pub struct PlanetSystem {
    pub name: &'static str,
    pub mass: f64,
    pub semimajor_axis: f64
}


pub struct KeplerOrbit {
    pub eccentricity: f64,
    pub semimajor_axis: f64,
    pub inclination: f64,
    pub longitude_an: f64,
    pub arg_periapsis: f64,
    pub true_anomaly: f64

}

pub fn au_to_km(dist_au: &f64) -> f64 {
    return dist_au * 149597870.700_f64;
}

pub fn km_to_mi(dist_km: &f64) -> f64 {
    return dist_km * 0.621371_f64; 
}

pub fn angle_between(a: &vecmath::Vector3<f64>, b: &vecmath::Vector3<f64>) -> f64 {
    let dot = vecmath::vec3_dot(*a, *b);
    let mag_a = vecmath::vec3_len(*a);
    let mag_b = vecmath::vec3_len(*b);

    (dot / (mag_a * mag_b)).acos()
}

pub fn calculate_soi_radius(semi_maj_axis: &f64, mass_a: &f64, mass_b: &f64) -> f64 { 
    let mut mass_ratio = mass_a / mass_b; 
    if mass_b < mass_a { 
        mass_ratio = mass_b / mass_a; 
    } 
    return semi_maj_axis * f64::powf(mass_ratio, 2.0/5.0); 
}

pub fn generate_soi_hm(solar_system: &HashMap<&'static str, PlanetSystem>, stellar_mass: &f64) -> HashMap<&'static str, f64> {
    let mut soi_hm = HashMap::new();
    for (name, ps) in solar_system {
        soi_hm.insert(*name, calculate_soi_radius(&ps.semimajor_axis, &ps.mass, stellar_mass));
    }

    return soi_hm;
}



pub fn calculate_transfer_dv_vectors(v_transfer: vecmath::Vector3<f64>, v_parking: vecmath::Vector3<f64>, transfer_angle: f64) -> f64{
    //v_transfer (->Vc) is the circular velocity of object 1 with respect to parent object
    //v_parking (->V) is the orbit velocity
    //transfer_angle (theta) is the flight path angle of v_parking

    let mag_vt = vecmath::vec3_len(v_transfer);
    let mag_vp = vecmath::vec3_len(v_parking);
    calculate_transfer_dv(mag_vt, mag_vp, transfer_angle)
}

pub fn calculate_transfer_dv(mag_vt: f64, mag_vp: f64, transfer_angle: f64) -> f64 {
    
    ((mag_vt * mag_vt) + (mag_vp * mag_vp) - (2.0 * mag_vt * mag_vp * transfer_angle.cos())).sqrt()
}



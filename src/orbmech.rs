use std::collections::HashMap;

pub struct PlanetSystem {
    pub name: &'static str,
    pub mass: f64,
    pub semimajor_axis: f64
}


pub struct ConicSection {
    pub apoapsis: f64,
    pub periapsis: f64,
    pub eccentricity: f64,
    pub inclination: f64
}
pub fn au_to_km(dist_au: &f64) -> f64 {
    return dist_au * 149597870.700_f64;
}

pub fn km_to_mi(dist_km: &f64) -> f64 {
    return dist_km * 0.621371_f64; 
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

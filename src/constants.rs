
use std::collections::HashMap;
use crate::orbmech;
pub const SUN_MASS: f64 = 1.9885e+30;
pub const MERCURY_MASS: f64 =  3.3011e+23; 
pub const VENUS_MASS: f64 = 4.8675e+24; 
pub const EARTH_MASS: f64 = 5.9722e+24; 
pub const LUNA_MASS: f64 = 7.342e+22; 
pub const MARS_MASS: f64 = 6.4171e+23; 
pub const JUPITER_MASS: f64 = 1.8982e+27;
pub const SATURN_MASS: f64 = 5.6834e+26;
pub const URANUS_MASS: f64 = 8.6810e+25;
pub const NEPTUNE_MASS: f64 = 1.024e+26;
pub const PLUTO_MASS: f64 = 1.303e+22;
pub const MERCURY_SMA: f64 = (0.387 + 0.37870) / 2.0;
pub const VENUS_SMA: f64 = (0.723 + 0.72298) / 2.0;
pub const EARTH_SMA: f64 = (1.0 + 0.99986) / 2.0;
pub const MARS_SMA: f64 = (1.524 + 1.51740) / 2.0;
pub const JUPITER_SMA: f64 = (5.2044 + 5.1982) / 2.0;
pub const SATURN_SMA: f64 = (9.5826 + 9.5673) / 2.0;
pub const URANUS_SMA: f64 = (19.2184 + 19.1977) / 2.0;
pub const NEPTUNE_SMA: f64 = (30.11 + 30.1087) / 2.0;
pub const PLUTO_SMA: f64 = 39.482;

pub const PLANET_NAMES: &[&str] = &["MERCURY", "VENUS", "EARTH", "MARS", "JUPITER", "SATURN", "URANUS", "NEPTUNE", "PLUTO"];

pub const G: f64 = 6.674e-11_f64;



pub fn generate_standard_mass_hm() -> HashMap<&'static str, f64> {
    
    let planet_masses: HashMap<&str, f64> = HashMap::from([
                                        ("MERCURY", MERCURY_MASS),
                                        ("VENUS", VENUS_MASS),
                                        ("EARTH", EARTH_MASS),
                                        ("MARS", MARS_MASS),
                                        ("JUPITER", JUPITER_MASS),
                                        ("SATURN", SATURN_MASS),
                                        ("URANUS", URANUS_MASS),
                                        ("NEPTUNE", NEPTUNE_MASS), 
                                        ("PLUTO", PLUTO_MASS), ]);
    return planet_masses;
}

pub fn generate_standard_semimaj_hm() -> HashMap<&'static str, f64> {
    let planet_axes: HashMap<&str, f64> = HashMap::from([
                                        ("MERCURY", orbmech::au_to_km(&MERCURY_SMA)),
                                        ("VENUS", orbmech::au_to_km(&VENUS_SMA)),
                                        ("EARTH", orbmech::au_to_km(&EARTH_SMA)),
                                        ("MARS", orbmech::au_to_km(&MARS_SMA)),
                                        ("JUPITER", orbmech::au_to_km(&JUPITER_SMA)),
                                        ("SATURN", orbmech::au_to_km(&SATURN_SMA)),
                                        ("URANUS", orbmech::au_to_km(&URANUS_SMA)),
                                        ("NEPTUNE", orbmech::au_to_km(&NEPTUNE_SMA)),
                                        ("PLUTO", orbmech::au_to_km(&PLUTO_SMA)) ]);

    return planet_axes;
}
pub fn populate_solar_system(masses: &HashMap<&'static str, f64>, axes: &HashMap<&'static str, f64>) -> HashMap<&'static str, orbmech::PlanetSystem> {
    let mut solar_system = HashMap::new();

    for (name, mass) in masses {
        let curr_axis_opt = axes.get(name);
        if curr_axis_opt.is_some() {
            let semimajor_axis = curr_axis_opt.unwrap();
            let psystem = orbmech::PlanetSystem {name, 
                                                mass: *mass, 
                                                semimajor_axis: *semimajor_axis};
            solar_system.insert(*name, psystem );
        }
    }

    return solar_system;
}

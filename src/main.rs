use orbits::constants as pconst;
use orbits::orbmech;
fn main() {

    //test commit comment from macbook baybeeeee


    let solar_masses = pconst::generate_standard_mass_hm();
    let solar_smaxes = pconst::generate_standard_semimaj_hm();
    let solarsystem_ps = pconst::populate_solar_system(&solar_masses, &solar_smaxes);
    let soi_hm = orbmech::generate_soi_hm(&solarsystem_ps, &pconst::SUN_MASS);

    println!("=====SPHERES OF INFLUENCE=====");

    for name in pconst::PLANET_NAMES {
        let system_opt = solarsystem_ps.get(name);
        if system_opt.is_some() {
            let system = system_opt.unwrap();
            let system_soi = soi_hm.get(name).unwrap(); //This is_some() by definition (I think)
            let system_soi_mi = orbmech::km_to_mi(&system_soi);
            let system_soi_km_106 = system_soi / 1.0e+6;

            println!("{}: {:.3} mi, {:.3}x10^6 km, calculated SMA: {:.4}", system.name, system_soi_mi, system_soi_km_106, system.semimajor_axis);
        }
    }
}

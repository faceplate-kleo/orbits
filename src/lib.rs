pub mod constants;
pub mod orbmech;
pub mod kepler;
pub mod spacecraft;


extern crate vecmath;


#[cfg(test)]
mod tests {
    use crate::orbmech;
    fn is_within_percentage(a: f64, b: f64, margin: f64) -> bool {
        (a - b) / b < margin
    }
    #[test]
    fn test_dv_calculation() {
        let earth_orbital_speed: f64 = 29.7827; //km/s 

        let mars_orbital_speed: f64 = 24.07; //km/s  
        

        let earth_mars_theta: f64 = 44.0;

        let earth_mars_dv = orbmech::calculate_transfer_dv(earth_orbital_speed, mars_orbital_speed, earth_mars_theta);


        println!("{}", earth_mars_dv);
        assert!(is_within_percentage(earth_mars_dv, 4.3, 0.2)) 
    }
    #[test]
    fn test_vecangle() {
        let veca: vecmath::Vector3<f64> =  [  1.0,  0.0,  0.0 ];
        let vecb: vecmath::Vector3<f64> =  [  0.0,  1.0,  0.0 ];

        let theta = orbmech::angle_between(&veca, &vecb) * 57.2958;

        assert!(is_within_percentage(theta, 90.0, 0.1));
    }
}

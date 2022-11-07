use std::f64::consts::PI;

#[derive(Clone,Copy)]
pub struct CabinetDimensions {
    pub port_length: u32,
    pub port_flare_radius: u32,
    pub port_external_width: u32,
    pub port_external_height: u32,
    pub box_volume: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CabinetModel {
    pub port_length: f64,
    pub port_flare_radius: f64,
    pub port_external_width: f64,
    pub port_external_height: f64,
    pub box_volume: f64,
}

impl From<CabinetDimensions> for CabinetModel {
    fn from(item: CabinetDimensions) -> Self {
        CabinetModel {
            port_length: f64::from(item.port_length),
            port_flare_radius: f64::from(item.port_flare_radius),
            port_external_width: f64::from(item.port_external_width),
            port_external_height: f64::from(item.port_external_height),
            box_volume: f64::from(item.box_volume)
        }
    }
}
impl CabinetModel {
    #[allow(non_snake_case)]
    pub fn port_flare_arc_start(&self) -> f64 {
        let l: f64 = self.port_length / 2.0; 
        let R = self.port_flare_radius;
        return (l/R).acos();
    }

    #[allow(non_snake_case)]
    pub fn port_min_diameter(&self) -> f64 {
        let l: f64 = self.port_length / 2.0; 
        let R = self.port_flare_radius;
        let d = (R.powi(2) - l.powi(2)).sqrt();
        let port_max_radius = self.port_external_height / 2.0;
        let port_min_radius = port_max_radius - (R - d); 
        return port_min_radius * 2.0;
    }
    
    #[allow(non_snake_case)]
    pub fn nfr_ratio(&self) -> f64 {
        let L_actual = self.port_length/1000.0;
        let r_fit = self.port_flare_radius/1000.0;
        return L_actual/(2.0*r_fit);
    }
    
    #[allow(non_snake_case)]
    pub fn resonant_frequency(&self) -> f64 {
        let L_actual = self.port_length/1000.0;
        let r_fit = self.port_flare_radius/1000.0;
        let D_min = self.port_min_diameter() / 1000.0;
        
        // special calculation for Jack based on square cross section
        //let A_min = PI*(D_min/2.0).powi(2);
        let A_min = D_min * (self.port_external_width/1000.0);
        let L_effective = L_actual + D_min;

        let A_effective = A_min * (1.0 + 0.576*(L_actual/(2.0*r_fit)));
        
        // calculate volume. Convert liters to m3
        let V_box = self.box_volume/1000.0;

        let speed_of_sound_constant = 343.0 / (2.0 * PI); 
        
        let frequency = speed_of_sound_constant * (A_effective / (L_effective * V_box)).sqrt();
        return frequency;
    }

}
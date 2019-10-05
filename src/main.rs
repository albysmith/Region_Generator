use rand::seq::SliceRandom;
use rand::Rng;
// use std::borrow::Borrow;
// use std::collections::HashMap;
// use std::fs;
use std::fs::File;
use std::io::Write;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;


#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    config: Config,
    spline_cubed: SplineCubed,
    spline_squared: SplineSquared,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct Config {

}

#[derive(Deserialize, Debug, Default, Clone)]
struct SplineCubed {
    step_range: Vec<i64>,
    step_rate: i64,
    h_offset_range: Vec<i64>,
    h_deadzone_range: Vec<i64>,
    v_offset_range: Vec<i64>,
    v_deadzone_range: Vec<i64>,
    width_variance: Vec<i64>,
    width_deadzone: Vec<i64>,
    y_values: Vec<i64>,
    y_variance: Vec<i64>,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct SplineSquared {
    step_range: Vec<i64>,
    step_rate: i64,
    h_offset_range: Vec<i64>,
    h_deadzone_range: Vec<i64>,
    v_offset_range: Vec<i64>,
    v_deadzone_range: Vec<i64>,
    width_variance: Vec<i64>,
    width_deadzone: Vec<i64>,
    y_values: Vec<i64>,
    y_variance: Vec<i64>,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct DefaultsToml {
    defaults: Defaults,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Defaults {
    asteroids: Asteroids,
    debris: Debris,
    lockbox: Lockbox,
    nebula: Nebula,
    positionals: Positionals,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Asteroids {
    asteroid_highyield_v1: ResourceAsteroid,
    asteroid_highyield_sil_v1: ResourceAsteroid,
    asteroid_highyield_niv_v1: ResourceAsteroid,
    asteroid_ore_xxl: ResourceAsteroid,
    asteroid_ore_xl: ResourceAsteroid,
    asteroid_ore_l: ResourceAsteroid,
    asteroid_ore_m: ResourceAsteroid,
    asteroid_ore_s: ResourceAsteroid,
    asteroid_ore_xs: ResourceAsteroid,
    asteroid_silicon_xl: ResourceAsteroid,
    asteroid_silicon_l: ResourceAsteroid,
    asteroid_silicon_m: ResourceAsteroid,
    asteroid_silicon_s: ResourceAsteroid,
    asteroid_silicon_xs: ResourceAsteroid,
    asteroid_nividium_l: ResourceAsteroid,
    asteroid_nividium_m: ResourceAsteroid,
    asteroid_nividium_s: ResourceAsteroid,
    asteroid_nividium_xs: ResourceAsteroid,
    asteroid_ice_xl: ResourceAsteroid,
    asteroid_ice_l: ResourceAsteroid,
    asteroid_ice_m: ResourceAsteroid,
    asteroid_ice_s: ResourceAsteroid,
    asteroid_ice_xs: ResourceAsteroid,
    asteroid_xenon_xxl: NonResource,
    asteroid_xenon_xl: NonResource,
    asteroid_xenon_l: NonResource,
    asteroid_xenon_m: NonResource,
    asteroid_xenon_s: NonResource,
    asteroid_xenon_xs: NonResource,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Debris {
    debris_xl: NonResource,
    debris_l: NonResource,
    debris_m: NonResource,
    debris_s: NonResource,
    debris_station_l: NonResource,
    env_debris_station_l_05: NonResource,
    debris_teladi_xl: NonResource,
    debris_paranid_xl: NonResource,
    debris_paranid_l: NonResource,
    debris_split_xl: NonResource,
    debris_split_s: NonResource,
    debris_xenon_xl: NonResource,
    debris_xenon_l: NonResource,
    debris_xenon_m: NonResource,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Lockbox {
    lockboxes_rare: NonResource,
    lockboxes_extra: NonResource,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Nebula {
    fogtest_nebula_vol_macro: ResourceNebula,
    fog_smallstones_v1_macro: ResourceNebula,
    fogvolume_small_macro: ResourceNebula,
    fogpattern_v2_macro: ResourceNebula,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Positionals {
    fog_outside_set3_macro: OtherNebula,
    fog_outside_set1_whiteblue_macro: OtherNebula,
    fog_outside_set1_lightbrown_macro: OtherNebula,
    fog_outside_set1_big_lightorange_macro: OtherNebula,
    fog_outside_set1_lightorange_macro: OtherNebula,
    fog_outside_set1_lightblue_macro: OtherNebula,
    fog_outside_set1_big_lightpurple_macro: OtherNebula,
    fog_outside_set1_blue_macro: OtherNebula,
    fog_outside_set1_burgundy_macro: OtherNebula,
    fog_outside_set1_green_macro: OtherNebula,
    fog_outside_set1_darkblue_macro: OtherNebula,
    fog_outside_set1_red_macro: OtherNebula,
    fog_outside_set1_grey_macro: OtherNebula,
    fog_outside_set1_dust_macro: OtherNebula,
    fog_outside_set1_lightbrown2_macro: OtherNebula,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct ResourceAsteroid {
    density_factor: f64, 
    density_randomization: Vec<f64>,
    rotation: i64, 
    rotationvariation: i64, 
    noisescale: i64,
    minnoisevalue: f64, 
    maxnoisevalue: f64, 
    distancefactor: f64, 
    resource: String,
    resource_amount: String,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct NonResource {
    density_factor: f64, 
    density_randomization: Vec<f64>,
    rotation: i64, 
    rotationvariation: i64, 
    noisescale: i64,
    minnoisevalue: f64, 
    maxnoisevalue: f64, 
    distancefactor: f64, 
}
#[derive(Deserialize, Debug, Default, Clone)]
struct ResourceNebula {
    localred: i64,
    localgreen: i64,
    localblue: i64,
    localdensity: f64,
    uniformred: i64,
    uniformgreen: i64,
    uniformblue: i64,
    uniformdensity: f64,
    backgroundfog: bool,
    resources: String,
    noisescale: i64,
    minnoisevalue: f64,
    maxnoisevalue: i64,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct OtherNebula {
    lodrule: String,
    densityfactor: f64,
    rotation: i64,
    rotationvariation: f64,
    noisescale: i64,
    minnoisevalue: f64,
    maxnoisevalue: f64,
    distancefactor: f64,
}


/*

To make a region:
<region name="" density="1.5" rotation="0")>
    <boundary class="splinetube">
        <size r="5000" />
        <splineposition x="" y="" z="" inlength="" outlength="" />
    </boundary>
    <falloff>
      <lateral>
        <step position="0.0" value="0.0" />
        <step position="0.1" value="1.0" />
        <step position="0.9" value="1.0" />
        <step position="1.0" value="0.0" />
      </lateral>
      <radial>
        <step position="0.0" value="1.0" />
        <step position="0.3" value="1.0" />
        <step position="0.5" value="0.9" />
        <step position="0.9" value="0.4" />
        <step position="1.0" value="0.0" />
      </radial>
    </falloff>
      <positional ref="fog_outside_set7_macro" densityfactor="3.0" noisescale="1000" seed="3214" minnoisevalue="0.0" maxnoisevalue="1"/>
      <positional ref="fog_outside_set1_lightbrown_macro" densityfactor="0.30" noisescale="1000" seed="3214" minnoisevalue="0.10" maxnoisevalue="1" distancefactor="0.5"/>
      <positional ref="fog_outside_set1_lightblue_macro" lodrule="nebula" densityfactor="0.50" noisescale="1000" seed="3214" minnoisevalue="0.10" maxnoisevalue="0.5" distancefactor="0.5"/>
      <positional ref="fog_outside_set1_big_lightpurple_macro" lodrule="nebula" densityfactor="0.40" noisescale="1000" seed="3214" minnoisevalue="0.50" maxnoisevalue="1" distancefactor="0.5"/>
      <positional ref="fog_outside_set1_lightspot_macro" lodrule="nebula" densityfactor="0.40" noisescale="1000" seed="3214" minnoisevalue="0.80" maxnoisevalue="1" distancefactor="1.0"/>
      <positional ref="fog_outside_set1_lightorange_macro" lodrule="nebula" densityfactor="0.50" noisescale="1000" seed="3214" minnoisevalue="0.10" maxnoisevalue="0.5" distancefactor="0.5"/>
      <positional ref="fog_outside_set1_green_macro" lodrule="nebula" densityfactor="0.50" noisescale="1000" seed="3214" minnoisevalue="0.40" maxnoisevalue="0.9" distancefactor="1.5"/>
      <positional ref="fog_outside_set1_blue_macro" lodrule="nebula" densityfactor="0.50" noisescale="1000" seed="3214" minnoisevalue="0.40" maxnoisevalue="0.9" distancefactor="1.5"/>
      <positional ref="fog_outside_set1_lightgreen_macro" lodrule="nebula" densityfactor="0.50" noisescale="1000" seed="3214" minnoisevalue="0.30" maxnoisevalue="0.9" distancefactor="0.5"/>
      <asteroid groupref="asteroid_ore_l" densityfactor="2" noisescale="1000" seed="3214" minnoisevalue="0.5" maxnoisevalue="1"/>
      <asteroid groupref="asteroid_silicon_m" densityfactor="1" noisescale="1000" seed="3214" minnoisevalue="0.25" maxnoisevalue="1"/>
      <asteroid groupref="asteroid_ice_l" densityfactor="0.3" noisescale="5000" seed="26041984" minnoisevalue="0.75" maxnoisevalue="1"/>
      <asteroid groupref="asteroid_ice_m" densityfactor="1.8" noisescale="5000" seed="26041984" minnoisevalue="0.75" maxnoisevalue="1"/>
      <asteroid groupref="asteroid_xenon_xxl" densityfactor="2" noisescale="5000" seed="26041984" minnoisevalue="0.75" maxnoisevalue="1" boxchecks="true" />
      <asteroid groupref="asteroid_xenon_xl" densityfactor="1" noisescale="5000" seed="26041984" minnoisevalue="0.75" maxnoisevalue="1" boxchecks="true" />
      <asteroid groupref="asteroid_xenon_l" densityfactor="1" noisescale="5000" seed="26041984" minnoisevalue="0.75" maxnoisevalue="1" boxchecks="true" />
      <asteroid groupref="asteroid_nividium_l" densityfactor="0.125" noisescale="1000" seed="3214" minnoisevalue="0.95" maxnoisevalue="1"/>
      <object ref="props_sm_lockbox_common_01_macro" densityfactor="0.005" />
      <object ref="props_sm_lockbox_unusual_01_macro" densityfactor="0.005" />
      <object ref="props_sm_lockbox_unusual_explosive_01_macro" densityfactor="0.0025" />
      <object ref="props_sm_lockbox_unusual_fragile_01_macro" densityfactor="0.0025" />
      <object ref="props_sm_lockbox_rare_02_macro" densityfactor="0.0025" />
      <object ref="props_sm_lockbox_rare_explosive_01_macro" densityfactor="0.0025" />
      <object ref="props_sm_lockbox_rare_fragile_01_macro" densityfactor="0.0025" />
      <nebula ref="fogvolume_small_macro" localred="49" localgreen="43" localblue="19" localdensity="0.8" uniformred="49" uniformgreen="43" uniformblue="19" uniformdensity="0.2" />
      <nebula ref="fogvolume_far_dv_macro" localred="172" localgreen="104" localblue="4" localdensity="0.2" uniformred="210" uniformgreen="125" uniformblue="6" uniformdensity="0.2"/>
      <nebula ref="fogvolume_near_v2_dv_macro" localred="210" localgreen="125" localblue="6" localdensity="0.5" uniformred="210" uniformgreen="125" uniformblue="6" uniformdensity="0.0"/>
    </fields>
    <resources>
      <resource ware="WARE" yield="medium or high" />
    </resources>
  </region>


globalregion_<sector_macro_name>

need input for sector macro names, count through all to create 1 region each
but first i just need to be able to make 1 region


Create this first, then play with the rest; that will be easier!

    <boundary class="splinetube">
      <size r="5000" />
      <splineposition x="2409.45" y="-875.0" z="23566.0" tx="-0.330587" ty="0.265313" tz="-0.905716" inlength="0.0" outlength="4004.71" />
      <splineposition x="-854.973" y="937.5" z="13772.7" tx="-0.28149" ty="0.0479245" tz="-0.958367" inlength="3027.82" outlength="3027.82" />
      <splineposition x="-2704.81" y="0.0" z="6155.77" tx="-0.192943" ty="-0.223302" tz="-0.955463" inlength="2731.62" outlength="2731.62" />
      <splineposition x="-4010.57" y="-2687.5" z="-1896.46" tx="0.0133637" ty="-0.0644138" tz="-0.997834" inlength="2599.0" outlength="2599.0" />
      <splineposition x="-2487.18" y="-1000.0" z="-9404.63" tx="0.375348" ty="0.18733" tz="-0.907756" inlength="2416.26" outlength="2416.26" />
      <splineposition x="1430.12" y="0.0" z="-15063.0" tx="0.747702" ty="-0.0194627" tz="-0.663749" inlength="1940.23" outlength="1940.23" />
      <splineposition x="6217.93" y="-1234.38" z="-17130.4" tx="0.910818" ty="-0.410074" tz="-0.0474369" inlength="1911.49" outlength="0.0" />
    </boundary>

***what are some good formulas to use?

  h_offset = horizontal offset (applied inversely to pos/neg sign)
  v_offset = vertical offset

  y = (x + h_offset)^2 / width + v_offset --- width between 100000-2000000 and inverse h_offset
  y = (x + h_offset)^3 / width + v_offset --- width between 10000000000-100000000000
  y = height * sin ( x + h_offset / width) + v_offset

  set a few height/width values and create 10+ formulas to pick from.  maybe also do some linear options?


***how do I get good points on the curve to create a nice curve?

  points every 10km (10000)
  lower the variance for the lengths


*/ 

fn main() {
    let mut prng = rand::thread_rng();
    let toml_str = include_str!("Config.toml");
    let toml_parsed: Toml = toml::from_str(&toml_str).unwrap();
    let defaults_str = include_str!("RegionConfig.toml");
    let defaults_parsed: DefaultsToml = toml::from_str(&defaults_str).unwrap_or_default();
    


    let chosen_profile = ["squared", "cubed"].choose(&mut prng).unwrap();

    let positions = match chosen_profile {
      &"cubed"   => get_positions_spline_cubed(&toml_parsed.spline_cubed),
      &"squared" => get_positions_spline_squared(&toml_parsed.spline_squared),
      _          => panic!("broken at profile select"),
    };

    let mut lengths = Vec::new();
    for j in 0..positions.len() {
        let mut inlength = 0;
        let current_pos = &positions[j];
        if j as i64 + 1 == positions.len() as i64 {
            let prior_pos = &positions[j-1];
            inlength = (distance(current_pos, prior_pos) / 2.0) as i64;
        }
        else if j as i64 - 1 >= 0 {
            let prior_pos = &positions[j-1];
            inlength = (distance(current_pos, prior_pos) / 2.0) as i64;
        }
        else {
        }
        lengths.push(inlength);
    }

    let mut region_string = "".to_string();
    for q in 0..positions.len() {
        let position = &positions[q];
        let inlength = lengths[q];
        let mut outlength = 0;
        if ((q + 1) as i64) < positions.len() as i64 {
            outlength = lengths[q+1];
        }
        let add_string = format!("<splineposition x=\"{}\" y=\"{}\" z=\"{}\" inlength=\"{}\" outlength=\"{}\" /> \n", position.0, position.1, position.2, inlength, outlength);
        region_string.push_str(add_string.as_str());
    }
    let out_path = "E:/Rust/Projects/OutPut Files/Regions/";
    let mut outputfile = File::create(format!("{}{}", &out_path, "region_definitions.xml")).unwrap();
    outputfile.write_all(region_string.as_bytes()).unwrap();
}

fn get_random_in_range(range: &Vec<i64>) -> (i64) {
    let mut prng = rand::thread_rng();
    let value = prng.gen_range(range[0], range[1]);
    value
}

// fn get_float_in_range(range: &Vec<f64>) -> (f64) {
//     let mut prng = rand::thread_rng();
//     let value = prng.gen_range(range[0], range[1]);
//     value
// }

fn distance(point_a: &(i64, i64, i64), point_b: &(i64, i64, i64)) -> (f64) {
    let dx = (point_a.0 - point_b.0) as f64;
    let dy = (point_a.1 - point_b.1) as f64;
    let dz = (point_a.2 - point_b.2) as f64;
    let value = ((((dz/dx).powi(2)) + 1.0).sqrt() * dx) + dy;
    value
}

fn get_spline_offset(range: &Vec<i64>, deadzone: &Vec<i64>) -> f64 {
    let mut prng = rand::thread_rng();
    let value = *[prng.gen_range(range[0], deadzone[0]), prng.gen_range(deadzone[1], range[1])].choose(&mut prng).unwrap() as f64;
    value
}

fn get_positions_spline_cubed(config: &SplineCubed) -> Vec<(i64, i64, i64)> {
    // spline_cubed
    // if we pick spline_cubed, then dump toml_parsed.spline_cubed and output a list of x, y, z positions
    let min_step = config.step_range[0];
    let max_step = config.step_range[1];
    let step = config.step_rate;
    let h_offset = get_spline_offset(&config.h_offset_range, &config.h_deadzone_range);
    let v_offset = get_spline_offset(&config.v_offset_range, &config.v_deadzone_range);
    let width = get_spline_offset(&config.width_variance, &config.width_deadzone);
    let mut y = get_random_in_range(&config.y_values);
    let y_variance = get_random_in_range(&config.y_variance);

    let mut positions = Vec::new();
    for i in min_step..=max_step {
        let x = i * step;
        y += y_variance;
        let z = (((x as f64 + h_offset).powi(3)) / width + v_offset) as i64;
        let position = (x, y, z);
        positions.push(position)
    }
    positions
}

fn get_positions_spline_squared(config: &SplineSquared) -> Vec<(i64, i64, i64)> {
    // spline_squared
    // if we pick spline_squared, then dump toml_parsed.spline_squared and output a list of x, y, z positions
    let min_step = config.step_range[0];
    let max_step = config.step_range[1];
    let step = config.step_rate;
    let h_offset = get_spline_offset(&config.h_offset_range, &config.h_deadzone_range);
    let v_offset = get_spline_offset(&config.v_offset_range, &config.v_deadzone_range);
    let width = get_spline_offset(&config.width_variance, &config.width_deadzone);
    let mut y = get_random_in_range(&config.y_values);
    let y_variance = get_random_in_range(&config.y_variance);

    let mut positions = Vec::new();
    for i in min_step..=max_step {
        let x = i * step;
        y += y_variance;
        let z = (((x as f64 + h_offset).powi(2)) / width + v_offset) as i64;
        let position = (x, y, z);
        positions.push(position)
    }
    positions
}
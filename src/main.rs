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
    region_types: Vec<String>,
    number_to_create: i64,
    out_path: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct SplineCubed {
    radius: Vec<i64>,
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
    fields: Fields,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct SplineSquared {
    radius: Vec<i64>,
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
    fields: Fields,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Fields {
    region_objects: Vec<i64>,
    region_count: i64,
    fields_mods: FieldsMods,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct FieldsMods {
    density_factor: Vec<f64>,
    density_randomization: Vec<f64>,
    rotation: Vec<f64>,
    rotationvariation: Vec<f64>,
    noisescale: Vec<f64>,
    minnoisevalue: Vec<f64>,
    maxnoisevalue: Vec<f64>,
    distancefactor: Vec<f64>,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct DefaultsToml {
    defaults: Defaults,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Defaults {
    resourceasteroids: ResourceRoids,
    asteroids: NonResourceRoids,
    debris: Debris,
    lockbox: Lockbox,
    nebula: Nebula,
    positionals: Positionals,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct ResourceRoids {
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
}
#[derive(Deserialize, Debug, Default, Clone)]
struct NonResourceRoids {
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
    name: String,
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
    name: String,
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
    name: String,
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
    name: String,
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
    let defaults_parsed: DefaultsToml = toml::from_str(&defaults_str).unwrap();
    
    let mut region_def_string = "".to_string();

    for _ in 0..toml_parsed.config.number_to_create {
        let chosen_profile = &toml_parsed.config.region_types.choose(&mut prng).unwrap();
        let region = match chosen_profile.as_str() {
            "cubed"   => create_region_spline_cubed(&toml_parsed.spline_cubed, &defaults_parsed.defaults),
            "squared" => create_region_spline_squared(&toml_parsed.spline_squared, &defaults_parsed.defaults),
            // "splat"   => create_region_splat(&toml_parsed.splat, &defaults_parsed),
            _         => panic!("broken at profile select"),
        };
        region_def_string.push_str(region.as_str());
    }

    let out_path = &toml_parsed.config.out_path;
    let mut outputfile = File::create(format!("{}{}", out_path, "region_definitions.xml")).unwrap();
    outputfile.write_all(region_def_string.as_bytes()).unwrap();
}

fn create_region_spline_cubed(spline_cubed: &SplineCubed, defaults: &Defaults) -> String {
    let mut region_string = "\n<region name=\"spline_cubed\" > \n".to_string();
    let positions = get_positions_spline_cubed(&spline_cubed);
    let lengths = get_lengths(&positions);
    let boundary_string = get_spline_boundary_format(&positions, &lengths, &spline_cubed.radius);
    region_string.push_str(boundary_string.as_str());
    region_string.push_str("<falloff> \n<lateral> \n<step position=\"0.0\" value=\"0.0\" /> \n<step position=\"0.1\" value=\"1.0\" /> \n<step position=\"0.9\" value=\"1.0\" /> \n<step position=\"1.0\" value=\"0.0\" /> \n</lateral> \n<radial> \n<step position=\"0.0\" value=\"1.0\" /> \n<step position=\"0.3\" value=\"1.0\" /> \n<step position=\"0.5\" value=\"0.9\" />\n<step position=\"0.9\" value=\"0.4\" />\n<step position=\"1.0\" value=\"0.0\" />\n</radial>\n</falloff>\n");
    let fields_string = get_fields_and_resources(&spline_cubed.fields, defaults);
    region_string.push_str(fields_string.as_str());
    region_string.push_str("</region> \n");
    region_string
}

fn create_region_spline_squared(spline_squared: &SplineSquared, defaults: &Defaults) -> String {
    let mut region_string = "\n<region name=\"spline_squared\" > \n".to_string();
    let positions = get_positions_spline_squared(&spline_squared);
    let lengths = get_lengths(&positions);
    let boundary_string = get_spline_boundary_format(&positions, &lengths, &spline_squared.radius);
    region_string.push_str(boundary_string.as_str());
    region_string.push_str("<falloff> \n<lateral> \n<step position=\"0.0\" value=\"0.0\" /> \n<step position=\"0.1\" value=\"1.0\" /> \n<step position=\"0.9\" value=\"1.0\" /> \n<step position=\"1.0\" value=\"0.0\" /> \n</lateral> \n<radial> \n<step position=\"0.0\" value=\"1.0\" /> \n<step position=\"0.3\" value=\"1.0\" /> \n<step position=\"0.5\" value=\"0.9\" />\n<step position=\"0.9\" value=\"0.4\" />\n<step position=\"1.0\" value=\"0.0\" />\n</radial>\n</falloff>\n");
    let fields_string = get_fields_and_resources(&spline_squared.fields, defaults);
    region_string.push_str(fields_string.as_str());
    region_string.push_str("</region> \n");
    region_string
}

fn get_random_in_range(range: &Vec<i64>) -> (i64) {
    let mut prng = rand::thread_rng();
    let value = prng.gen_range(range[0], range[1]);
    value
}

fn get_variant_in_range(range: &Vec<f64>) -> (f32) {
    let mut prng = rand::thread_rng();
    let value = prng.gen_range(range[0], range[1]);
    value as f32
}

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

fn get_lengths(positions: &Vec<(i64, i64, i64)>) -> Vec<i64> {
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
    lengths
}

/*
fn match_objects(defaults_parsed: &DefaultsToml, chosen_object: Toml)  {  //pass in the object and its struct from the profile
    let default_values = match chosen_object.spline_cubed.asteroid_highyield_sil_v1.name.as_str() {
    "asteroid_highyield_sil_v1"     =>  {  //pass to a function that checks for non-null values and replaces them with the values from the correct default struct.  update profile with defaults, but don't overwrite anything that's non-null
                                      if chosen_object.spline_cubed.asteroid_highyield_sil_v1.density_factor != None {
                                        println!("value is none");
                                        // chosen_object.spline_cubed.asteroid_highyield_sil_v1 = ObjectStruct {density_randomization = defaults_parsed.defaults.asteroids.asteroid_highyield_v1.density_randomization..}
                                        let profile = chosen_object.spline_cubed.asteroid_highyield_sil_v1;
                                        let default = defaults_parsed.defaults.asteroids.asteroid_highyield_v1.clone();
                                        let new_struct = ResourceAsteroid {density_factor: profile.density_factor.unwrap(), .. default } ;
                                          println!("{:#?}", new_struct);
                                      }
                                    }, 
    // "asteroid_highyield_sil_v1" => defaults_parsed.defaults.asteroids.asteroid_highyield_sil_v1.clone(),
    _                           => panic!("broken at object match"),
    };
    
}
*/

fn get_spline_boundary_format(positions: &Vec<(i64, i64, i64)>, lengths: &Vec<i64>, radius_range: &Vec<i64>) -> String {
    let mut boundary_string = "  <boundary class=\"splinetube\"> \n".to_string();
    let radius = get_random_in_range(radius_range);
    boundary_string.push_str(format!("    <radius r=\"{}\" /> \n", radius).as_str());
    for q in 0..positions.len() {
        let position = &positions[q];
        let inlength = lengths[q];
        let mut outlength = 0;
        if ((q + 1) as i64) < positions.len() as i64 {
            outlength = lengths[q+1];
        }
        let add_string = format!("    <splineposition x=\"{}\" y=\"{}\" z=\"{}\" inlength=\"{}\" outlength=\"{}\" /> \n", position.0, position.1, position.2, inlength, outlength);
        boundary_string.push_str(add_string.as_str());
    }
    boundary_string.push_str("  </boundary> \n");
    boundary_string
}

fn get_fields_and_resources(fields: &Fields, defaults: &Defaults) -> String {
    let mut prng = rand::thread_rng();
    let mut fields_string = "<fields> \n".to_string();
    let mut resources_string = "<resources> \n".to_string();
    for _ in 0..fields.region_count {
        let chosen_field = &fields.region_objects.choose(&mut prng).unwrap();
        let add_strings = match chosen_field {
            1 => get_resource_asteroid(&fields.fields_mods, &defaults.resourceasteroids),
            2 => get_nonresource_asteroid(&fields.fields_mods, &defaults.asteroids),
            // 3 => get_debris(&fields.debris_mods, &defaults.debris);
            // 4 => get_lockbox(&fields.lockbox_mods, &defaults.lockbox);
            // 5 => get_resource_nebula(&fields.resource_nebula_mods, &defaults.nebula);
            // 6 => get_nonresource_nebula(&fields.nonresource_nebula_mods, &defaults.positionals);
            // 7 => get_sound_region(&fields.sound_mods, &defaults.sounds);
            _ => panic!("broken at resource choices"),
        };
        fields_string.push_str(&add_strings[0].as_str());
        if add_strings.len() > 1 {
            resources_string.push_str(&add_strings[1].as_str());
        }
    }
    fields_string.push_str("</fields> \n");
    resources_string.push_str("</resources> \n");
    fields_string.push_str(resources_string.as_str());
    fields_string
}

fn get_resource_asteroid(fields_mods: &FieldsMods, resourceasteroids: &ResourceRoids) -> Vec<String> {
    let mut prng = rand::thread_rng();
    let mut add_strings = Vec::new();
    let roid_choice = match [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23].choose(&mut prng).unwrap() {
        1 => &resourceasteroids.asteroid_highyield_v1,
        2 => &resourceasteroids.asteroid_highyield_sil_v1,
        3 => &resourceasteroids.asteroid_highyield_niv_v1,
        4 => &resourceasteroids.asteroid_ore_xxl,
        5 => &resourceasteroids.asteroid_ore_xl,
        6 => &resourceasteroids.asteroid_ore_l,
        7 => &resourceasteroids.asteroid_ore_m,
        8 => &resourceasteroids.asteroid_ore_s,
        9 => &resourceasteroids.asteroid_ore_xs,
        10=> &resourceasteroids.asteroid_silicon_xl,
        11=> &resourceasteroids.asteroid_silicon_l,
        12=> &resourceasteroids.asteroid_silicon_m,
        13=> &resourceasteroids.asteroid_silicon_s,
        14=> &resourceasteroids.asteroid_silicon_xs,
        15=> &resourceasteroids.asteroid_nividium_l,
        16=> &resourceasteroids.asteroid_nividium_m,
        17=> &resourceasteroids.asteroid_nividium_s,
        18=> &resourceasteroids.asteroid_nividium_xs,
        19=> &resourceasteroids.asteroid_ice_xl,
        20=> &resourceasteroids.asteroid_ice_l,
        21=> &resourceasteroids.asteroid_ice_m,
        22=> &resourceasteroids.asteroid_ice_s,
        23=> &resourceasteroids.asteroid_ice_xs,
        _ => panic!("broken at resourceroids"),
    };
    let density_factor = roid_choice.density_factor as f32 * get_variant_in_range(&fields_mods.density_factor);
    // let density_randomization = roid_choice.density_randomization * get_variant_in_range(&fields_mods.density_randomization);
    let rotation = roid_choice.rotation as f32 * get_variant_in_range(&fields_mods.rotation);
    let rotationvariation = roid_choice.rotationvariation as f32 * get_variant_in_range(&fields_mods.rotationvariation);
    let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
    let minnoisevalue = roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
    let maxnoisevalue = roid_choice.maxnoisevalue as f32 * get_variant_in_range(&fields_mods.maxnoisevalue);
    let distancefactor = roid_choice.distancefactor;
    let field = format!("<asteroid groupref=\"{}\" densityfactor=\"{}\" rotation=\"{}\" rotationvariation=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\" maxnoisevalue=\"{}\" distancefactor=\"{}\" /> \n", roid_choice.name, density_factor, rotation, rotationvariation, noisescale, minnoisevalue, maxnoisevalue, distancefactor).to_string();
    let resource = format!("<resource ware=\"{}\" yield=\"{}\" />\n", roid_choice.resource, roid_choice.resource_amount).to_string();
    add_strings.push(field);
    add_strings.push(resource);
    add_strings
}

fn get_nonresource_asteroid(fields_mods: &FieldsMods, asteroids: &NonResourceRoids) -> Vec<String> {
    let mut prng = rand::thread_rng();
    let mut add_strings = Vec::new();
    let roid_choice = match [1,2,3,4,5,6].choose(&mut prng).unwrap() {
        1 => &asteroids.asteroid_xenon_xxl,
        2 => &asteroids.asteroid_xenon_xl,
        3 => &asteroids.asteroid_xenon_l,
        4 => &asteroids.asteroid_xenon_m,
        5 => &asteroids.asteroid_xenon_s,
        6 => &asteroids.asteroid_xenon_xs,
        _ => panic!("broken at nonresourceroids"),
    };
    let density_factor = roid_choice.density_factor as f32 * get_variant_in_range(&fields_mods.density_factor);
    // let density_randomization = roid_choice.density_randomization * get_variant_in_range(&fields_mods.density_randomization);
    let rotation = roid_choice.rotation as f32 * get_variant_in_range(&fields_mods.rotation);
    let rotationvariation = roid_choice.rotationvariation as f32 * get_variant_in_range(&fields_mods.rotationvariation);
    let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
    let minnoisevalue = roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
    let maxnoisevalue = roid_choice.maxnoisevalue as f32 * get_variant_in_range(&fields_mods.maxnoisevalue);
    let distancefactor = roid_choice.distancefactor;
    let field = format!("<asteroid groupref=\"{}\" densityfactor=\"{}\" rotation=\"{}\" rotationvariation=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\" maxnoisevalue=\"{}\" distancefactor=\"{}\" /> \n", roid_choice.name, density_factor, rotation, rotationvariation, noisescale, minnoisevalue, maxnoisevalue, distancefactor).to_string();
    add_strings.push(field);
    add_strings
}


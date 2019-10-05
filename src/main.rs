use rand::seq::SliceRandom;
use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;


#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    config: Config,

}

#[derive(Deserialize, Debug, Default, Clone)]
struct Config {

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
    // let x_values = vec![-500000, -400000, -300000, -200000, -100000, 0, 100000, 200000, 300000, 400000, 500000];
    let y_values = vec![-10000,10000];
    // let z_values = vec![-500000,500000];
    let variant = vec![1.0, 1.2];
    let mut positions = Vec::new();
    for i in -50..=50 {
        let x = i as f32 * 10000.0;
        let y = get_random_in_range(&y_values) as f32;
        let z = (x.powi(3)) / 100000000000.0;
        let position = vec![x, y, z];
        positions.push(position)
    }
    let mut lengths = Vec::new();
    for j in 0..positions.len() {
        let mut inlength = 0;
        let p = j as i32;
        let current_pos = &positions[j];
        if p + 1 == positions.len() as i32 {
            let prior_pos = &positions[j-1];
            let distance = distance(current_pos, prior_pos);
            let variance =  get_variance_in_range(&variant);
            inlength = (distance / 2.0 * variance) as i32;
        }
        else if p - 1 >= 0 {
            let prior_pos = &positions[j-1];
            let distance = distance(current_pos, prior_pos);

            let variance =  get_variance_in_range(&variant);
            inlength = (distance / 2.0 * variance) as i32;
        }
        else {

        }
        lengths.push(inlength);
        // println!("x=\"{}\" y=\"{}\" z=\"{}\"", current_pos[0], current_pos[1], current_pos[2])
    }
    // println!("{:?}", lengths);

    let mut region_string = "".to_string();
    for q in 0..positions.len() {
        let position = &positions[q];
        let inlength = lengths[q];
        let mut outlength = 0;
        if ((q + 1) as i32) < positions.len() as i32 {
            outlength = lengths[q+1];
        }
        let add_string = format!("<splineposition x=\"{}\" y=\"{}\" z=\"{}\" inlength=\"{}\" outlength=\"{}\" /> \n", position[0], position[1], position [2], inlength, outlength);
        region_string.push_str(add_string.as_str());
    }
    let out_path = "E:/Rust/Projects/OutPut Files/Regions/";
    let mut outputfile = File::create(format!("{}{}", &out_path, "region_definitions.xml")).unwrap();
    outputfile.write_all(region_string.as_bytes()).unwrap();
}

fn get_random_in_range(range: &Vec<i32>) -> (i32) {
    let mut prng = rand::thread_rng();
    let value = prng.gen_range(range[0], range[1]);
    value
}

fn get_variance_in_range(range: &Vec<f32>) -> (f32) {
    let mut prng = rand::thread_rng();
    let value = prng.gen_range(range[0], range[1]);
    value
}

fn distance(point_b: &Vec<f32>, point_a: &Vec<f32>) -> (f32) {
    let squared = (point_b[0] - point_a[0]).powi(2) + (point_b[2] - point_a[2]).powi(2);
    let value = squared.sqrt();
    value
}


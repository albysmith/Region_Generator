use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io::Write;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;

#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    config: Config,
    spline_cubed: Spline,
    splat: Splat,
    spline_squared: Spline,
    spline_sin: Spline,
    spline_ellipse: Ellipse,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct Config {
    number_of_cycles: i64,
    region_range: Vec<i64>,
    region_types: Vec<i64>,
    out_path: String,
    clean_cluster_file: String,
    pitch: Vec<i64>,
    yaw: Vec<i64>,
    roll: Vec<i64>,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct Ellipse {
    radius: Vec<i64>,
    step_range: Vec<i64>,
    ellipse_radius: Vec<i64>,
    starting_angle: Vec<f64>,
    h_offset_range: Vec<i64>,
    h_deadzone_range: Vec<i64>,
    v_offset_range: Vec<i64>,
    v_deadzone_range: Vec<i64>,
    y_values: Vec<i64>,
    y_variance: Vec<i64>,
    fields: Fields,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Spline {
    radius: Vec<i64>,
    step_range: Vec<i64>,
    step_rate: i64,
    h_offset_range: Vec<i64>,
    h_deadzone_range: Vec<i64>,
    v_offset_range: Vec<i64>,
    v_deadzone_range: Vec<i64>,
    width_variance: Vec<f64>,
    width_deadzone: Vec<f64>,
    y_values: Vec<i64>,
    y_variance: Vec<i64>,
    fields: Fields,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Splat {
    radius: Vec<i64>,
    linear: Vec<i64>,
    // x_offset: Vec<i64>,
    // z_offset: Vec<i64>,
    fields: Fields,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Fields {
    region_objects: Vec<i64>,
    region_count: Vec<i64>,
    fields_mods: FieldsMods,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct FieldsMods {
    resource_roid_weight: Vec<i64>,
    nonresource_roid_weight: Vec<i64>,
    debris_weight: Vec<i64>,
    lockbox_weight: Vec<i64>,
    resource_neb_weight: Vec<i64>,
    positional_weight: Vec<i64>,
    sound_weight: Vec<i64>,
    density_factor: Vec<f64>,
    rotation: Vec<f64>,
    rotationvariation: Vec<f64>,
    noisescale: Vec<f64>,
    minnoisevalue: Vec<f64>,
    maxnoisevalue: Vec<f64>,
    lateral_1: Vec<f64>,
    lateral_2: Vec<f64>,
    lateral_3: Vec<f64>,
    lateral_4: Vec<f64>,
    lateral_5: Vec<f64>,
    radial_1: Vec<f64>,
    radial_2: Vec<f64>,
    radial_3: Vec<f64>,
    radial_4: Vec<f64>,
    radial_5: Vec<f64>,
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
    sounds: Sounds,
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
struct Sounds {
    zoned_sound_01: Sound,
    zoned_sound_02: Sound,
    zoned_sound_03: Sound,
    zoned_sound_04: Sound,
    zoned_sound_05: Sound,
    zoned_sound_06: Sound,
    zoned_sound_07: Sound,
    legacy_music_pool: Sound,
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
    maxnoisevalue: f64,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct OtherNebula {
    name: String,
    lodrule: String,
    density_factor: f64,
    rotation: i64,
    rotationvariation: f64,
    noisescale: i64,
    minnoisevalue: f64,
    maxnoisevalue: f64,
    distancefactor: f64,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Sound {
    name: String,
    noisescale: i64,
    minnoisevalue: f64,
    playtime: i64,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct OffsetsFile {
    offset: Vec<Offset>,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Offset {
    cluster: String,
    name: String,
    x: i64,
    y: i64,
    z: i64,
}

fn main() {
    let mut prng = rand::thread_rng();
    let toml_str = include_str!("Config.toml");
    let toml_parsed: Toml = toml::from_str(&toml_str).unwrap();
    let defaults_str = include_str!("RegionConfig.toml");
    let defaults_parsed: DefaultsToml = toml::from_str(&defaults_str).unwrap();
    let offsets_str = include_str!("offsets.toml");
    let offsets_parsed: OffsetsFile = toml::from_str(&offsets_str).unwrap();
    let clusters_str = include_str!("E:/Rust/Projects/OutPut Files/Regions/clustersnotags.xml");

    let mut region_def_string = "<?xml version=\"1.0\" encoding=\"utf-8\" ?>\n<regions>\n".to_string();
    let mut region_names = Vec::new();

    for sector in 0..toml_parsed.config.number_of_cycles {
        let region_count = get_random_in_range(&toml_parsed.config.region_range);
        for region in 0..region_count {
            let obj = toml_parsed.config.region_types.choose(&mut prng).unwrap();
            let region = match obj {
                1 => create_region_spline_cubed(&toml_parsed.spline_cubed, &defaults_parsed.defaults, (sector, region)),
                2 => create_region_spline_squared(&toml_parsed.spline_squared, &defaults_parsed.defaults, (sector, region)),
                3 => create_region_splat(&toml_parsed.splat, &defaults_parsed.defaults, (sector, region)),
                4 => create_region_spline_sin(&toml_parsed.spline_sin, &defaults_parsed.defaults, (sector, region)),
                5 => create_region_spline_ellipse(&toml_parsed.spline_ellipse, &defaults_parsed.defaults, (sector, region)),
                _ => panic!("broken at region shape choice"),
            };
            region_names.push(region.1);
            region_def_string.push_str(region.0.as_str());
        }
    }
    region_def_string.push_str("\n</regions>");

    let out_path = &toml_parsed.config.out_path;
    let mut outputfile = File::create(format!("{}{}", out_path, "region_definitions.xml")).unwrap();
    outputfile.write_all(region_def_string.as_bytes()).unwrap();

    let mut connections_string = "".to_string();
    let mut congroup = 0;
    for region in region_names.iter() {
        if congroup != region.1 {
            congroup += 1;
            connections_string.push_str(&format!("\n new region\n"));
        }
        let name = &region.0;
        let mut offset_values = offsets_parsed.offset[region.1 as usize].clone();
        if region.0.contains("splat") {
            offset_values.y -= 100000;
        }
        let pitch = get_random_in_range(&toml_parsed.config.pitch);
        let yaw = get_random_in_range(&toml_parsed.config.yaw);
        let roll = get_random_in_range(&toml_parsed.config.roll);
        connections_string.push_str(format!("
        <!-- CLUSTER: {} SECTOR: {} -->\n<connection name=\"{}\" ref=\"regions\"> \n<offset>\n <position x=\"{}\" y=\"{}\" z=\"{}\" />\n<rotation pitch=\"{}\" yaw=\"{}\" roll=\"{}\" />\n</offset>\n<macro name=\"{}_macro\">\n<component connection=\"cluster\" ref=\"standardregion\" />\n<properties>\n<region ref=\"{}\" />\n</properties>\n</macro>\n</connection>\n", 
        offset_values.cluster, offset_values.name, name, offset_values.x, offset_values.y, offset_values.z, pitch, yaw, roll, name, name).as_str());
    }
    // let mut connectionfile = File::create(format!("{}{}", out_path, "connections.xml")).unwrap();
    // connectionfile
    //     .write_all(connections_string.as_bytes())
    //     .unwrap();
    let mut outputfileclusters = File::create(format!("{}{}", out_path, "clusters.xml")).unwrap();
    outputfileclusters.write_all(convert_to_clusters_file(connections_string, clusters_str.to_string()).as_bytes()).unwrap();
}

fn create_region_spline_cubed(spline_cubed: &Spline, defaults: &Defaults, counts: (i64, i64)) -> (String, (String, i64)) {
    let name = format!("sector_{}_region_{}_cubed", counts.0, counts.1).to_string();
    let mut region_string = format!("\n<region name=\"{}\" > \n", name).to_string();
    let positions = get_positions_spline_cubed(&spline_cubed);
    let lengths = get_lengths(&positions);
    region_string.push_str(get_spline_boundary_format(&positions, &lengths, &spline_cubed.radius).as_str());
    region_string.push_str(get_falloff(&spline_cubed.fields.fields_mods).as_str());    
    region_string.push_str(get_fields_and_resources(&spline_cubed.fields, defaults).as_str());
    region_string.push_str("</region> \n");
    (region_string, (name, counts.0))
}

fn create_region_spline_squared(spline_squared: &Spline, defaults: &Defaults, counts: (i64, i64)) -> (String, (String, i64)) {
    let name = format!("sector_{}_region_{}_squared", counts.0, counts.1).to_string();
    let mut region_string = format!("\n<region name=\"{}\" > \n", name).to_string();
    let positions = get_positions_spline_squared(&spline_squared);
    let lengths = get_lengths(&positions);
    region_string.push_str(get_spline_boundary_format(&positions, &lengths, &spline_squared.radius).as_str());
    region_string.push_str(get_falloff(&spline_squared.fields.fields_mods).as_str());    
    region_string.push_str(get_fields_and_resources(&spline_squared.fields, defaults).as_str());
    region_string.push_str("</region> \n");
    (region_string, (name, counts.0))
}

fn create_region_spline_sin(spline_sin: &Spline, defaults: &Defaults, counts: (i64, i64)) -> (String, (String, i64)) {
    let name = format!("sector_{}_region_{}_sin", counts.0, counts.1).to_string();
    let mut region_string = format!("\n<region name=\"{}\" > \n", name).to_string();
    let positions = get_positions_spline_sin(&spline_sin);
    let lengths = get_lengths(&positions);
    region_string.push_str(get_spline_boundary_format(&positions, &lengths, &spline_sin.radius).as_str());
    region_string.push_str(get_falloff(&spline_sin.fields.fields_mods).as_str());    
    region_string.push_str(get_fields_and_resources(&spline_sin.fields, defaults).as_str());
    region_string.push_str("</region> \n");
    (region_string, (name, counts.0))
}

fn create_region_spline_ellipse(spline_ellipse: &Ellipse, defaults: &Defaults, counts: (i64, i64)) -> (String, (String, i64)) {
    let name = format!("sector_{}_region_{}_ellipse", counts.0, counts.1).to_string();
    let mut region_string = format!("\n<region name=\"{}\" > \n", name).to_string();
    let positions = get_positions_spline_ellipse(&spline_ellipse);
    let lengths = get_lengths(&positions);
    region_string.push_str(get_spline_boundary_format(&positions, &lengths, &spline_ellipse.radius).as_str());
    region_string.push_str(get_falloff(&spline_ellipse.fields.fields_mods).as_str());    
    region_string.push_str(get_fields_and_resources(&spline_ellipse.fields, defaults).as_str());
    region_string.push_str("</region> \n");
    (region_string, (name, counts.0))
}

fn create_region_splat(splat: &Splat, defaults: &Defaults, counts: (i64, i64)) -> (String, (String, i64)) {
    let name = format!("sector_{}_region_{}_splat", counts.0, counts.1).to_string();
    let mut region_string = format!("\n<region name=\"{}\" > \n", name).to_string();
    region_string.push_str(format!("<boundary class=\"cylinder\"> \n<size r=\"{}\" linear=\"{}\" />\n</boundary>\n", get_random_in_range(&splat.radius), get_random_in_range(&splat.linear)).as_str());
    region_string.push_str(get_falloff(&splat.fields.fields_mods).as_str());
    region_string.push_str(get_fields_and_resources(&splat.fields, defaults).as_str());
    region_string.push_str("</region> \n");
    (region_string, (name, counts.0))
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
    let dx = (point_a.0 - point_b.0) as f64 + 1.0;
    let mut dy = (point_a.1 - point_b.1) as f64 + 1.0;
    let dz = (point_a.2 - point_b.2) as f64 + 1.0;
    if dy < 0.0 { dy = -dy }
    let value = (((dz/dx).powi(2) + 1.0).sqrt() * dx ) + dy;
    value
}

fn get_spline_offset(range: &Vec<i64>, deadzone: &Vec<i64>) -> f64 {
    let mut prng = rand::thread_rng();
    let value = *[
        prng.gen_range(range[0], deadzone[0]),
        prng.gen_range(deadzone[1], range[1]),
    ]
    .choose(&mut prng)
    .unwrap() as f64;
    value
}

fn get_width_offset(range: &Vec<f64>, deadzone: &Vec<f64>) -> f64 {
    let mut prng = rand::thread_rng();
    let value = *[
        prng.gen_range(range[0], deadzone[0]),
        prng.gen_range(deadzone[1], range[1]),
    ]
    .choose(&mut prng)
    .unwrap() as f64;
    value
}

fn get_positions_spline_cubed(config: &Spline) -> Vec<(i64, i64, i64)> {
    let min_step = config.step_range[0];
    let max_step = config.step_range[1];
    let step = config.step_rate;
    let h_offset = get_spline_offset(&config.h_offset_range, &config.h_deadzone_range);
    let v_offset = get_spline_offset(&config.v_offset_range, &config.v_deadzone_range);
    let width = get_width_offset(&config.width_variance, &config.width_deadzone);
    let mut y = get_random_in_range(&config.y_values);
    let y_variance = get_random_in_range(&config.y_variance);

    let mut positions = Vec::new();
    for i in min_step..=max_step {
        let x = i * step;
        y += y_variance;
        let z = (((x as f64 + h_offset).powi(3)) / width + v_offset) as i64;
        let position = (x, y, z);
        if z < 10000000 && x > -10000000 {
            positions.push(position)
        }
    }
    positions
}

fn get_positions_spline_squared(config: &Spline) -> Vec<(i64, i64, i64)> {
    let min_step = config.step_range[0];
    let max_step = config.step_range[1];
    let step = config.step_rate;
    let h_offset = get_spline_offset(&config.h_offset_range, &config.h_deadzone_range);
    let v_offset = get_spline_offset(&config.v_offset_range, &config.v_deadzone_range);
    let width = get_width_offset(&config.width_variance, &config.width_deadzone);
    let mut y = get_random_in_range(&config.y_values);
    let y_variance = get_random_in_range(&config.y_variance);

    let mut positions = Vec::new();
    for i in min_step..=max_step {
        let x = i * step;
        y += y_variance;
        let z = (((x as f64 + h_offset).powi(2)) / width + v_offset) as i64;
        let position = (x, y, z);
        if z < 10000000 && x > -10000000 {
            positions.push(position)
        }
    }
    positions
}

fn get_positions_spline_sin(config: &Spline) -> Vec<(i64, i64, i64)> {
    let min_step = config.step_range[0];
    let max_step = config.step_range[1];
    let step = config.step_rate;
    let h_offset = get_spline_offset(&config.h_offset_range, &config.h_deadzone_range);
    let v_offset = get_spline_offset(&config.v_offset_range, &config.v_deadzone_range);
    let width = get_width_offset(&config.width_variance, &config.width_deadzone);
    let mut y = get_random_in_range(&config.y_values);
    let y_variance = get_random_in_range(&config.y_variance);

    let mut positions = Vec::new();
    for i in min_step..=max_step {
        let x = i * step;
        y += y_variance;
        let z = ((((x as f64 * width) + h_offset).sin()) * x as f64 + v_offset) as i64;
        let position = (x, y, z);
        if z < 10000000 && x > -10000000 {
            positions.push(position)
        }
    }
    positions
}

fn get_positions_spline_ellipse(config: &Ellipse) -> Vec<(i64, i64, i64)> {
    let step_count = get_random_in_range(&config.step_range);
    let radius = get_random_in_range(&config.ellipse_radius);
    let h_offset = get_spline_offset(&config.h_offset_range, &config.h_deadzone_range);
    let v_offset = get_spline_offset(&config.v_offset_range, &config.v_deadzone_range);
    let y_variance = get_random_in_range(&config.y_variance);

    let mut positions = Vec::new();
    let mut angle: f64 = get_variant_in_range(&config.starting_angle) as f64;
    let mut y = get_random_in_range(&config.y_values);
    for _ in 0..step_count {
        angle += 0.13;
        let x = radius as f64 * angle.cos() + radius as f64 * angle.sin() + h_offset as f64;
        y += y_variance;
        let z = -radius as f64 * angle.sin() + radius as f64 * angle.cos() + v_offset as f64;
        let position = (x as i64, y, z as i64);
        if z < 10000000.0 && x > -10000000.0 {
            positions.push(position)
        }
    }
    // println!("{:?}", positions);
    positions
}

fn get_lengths(positions: &Vec<(i64, i64, i64)>) -> Vec<i32> {
    let mut lengths = Vec::new();
    for j in 0..positions.len() {
        let mut inlength: i32 = 0;
        let current_pos = &positions[j];
        if j != 0 {
            let prior_pos = &positions[j - 1];
            inlength = (distance(current_pos, prior_pos) / 2.0) as i32;
        } else {
        }
        if inlength < 0 {
            inlength = inlength * -1
        }
        lengths.push(inlength);
        // println!("{:?}", inlength);
    }
    lengths
}

fn get_spline_boundary_format(positions: &Vec<(i64, i64, i64)>, lengths: &Vec<i32>, radius_range: &Vec<i64>) -> String {
    let mut boundary_string = "  <boundary class=\"splinetube\"> \n".to_string();
    let radius = get_random_in_range(radius_range);
    boundary_string.push_str(format!("    <size r=\"{}\" /> \n", radius).as_str());
    for q in 0..positions.len() {
        let position = &positions[q];
        let inlength = lengths[q];
        let mut outlength = 0;
        if ((q + 1) as i64) < positions.len() as i64 {
            outlength = lengths[q + 1];
        }
        let add_string = format!(
            "    <splineposition x=\"{}\" y=\"{}\" z=\"{}\" inlength=\"{}\" outlength=\"{}\" /> \n",
            position.0, position.1, position.2, inlength, outlength
        );
        boundary_string.push_str(add_string.as_str());
    }
    boundary_string.push_str("  </boundary> \n");
    boundary_string
}

fn get_falloff (fieldmods: &FieldsMods) -> String {
    let mut falloff_string = "<falloff>\n<lateral>\n".to_string();
    falloff_string.push_str(format!("<step position=\"0.0\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.lateral_5)).as_str());
    falloff_string.push_str(format!("<step position=\"0.25\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.lateral_4)).as_str());
    falloff_string.push_str(format!("<step position=\"0.5\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.lateral_3)).as_str());
    falloff_string.push_str(format!("<step position=\"0.75\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.lateral_2)).as_str());
    falloff_string.push_str(format!("<step position=\"1.0\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.lateral_1)).as_str());
    falloff_string.push_str(format!("</lateral>\n<radial>\n<step position=\"0.0\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.radial_5)).as_str());
    falloff_string.push_str(format!("<step position=\"0.25\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.lateral_4)).as_str());
    falloff_string.push_str(format!("<step position=\"0.5\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.lateral_3)).as_str());
    falloff_string.push_str(format!("<step position=\"0.75\" value=\"{}\" />\n", get_variant_in_range(&fieldmods.lateral_2)).as_str());
    falloff_string.push_str(format!("<step position=\"1.0\" value=\"{}\" />\n</radial>\n</falloff>\n", get_variant_in_range(&fieldmods.lateral_1)).as_str());
    falloff_string
}

fn get_fields_and_resources(fields: &Fields, defaults: &Defaults) -> String {
    let mut prng = rand::thread_rng();
    let mut fields_string = "<fields> \n".to_string();
    let mut resources_string = "<resources> \n".to_string();
    let field_count = get_random_in_range(&fields.region_count);
    for _ in 0..field_count {
        let mut obj = fields.region_objects.choose(&mut prng).unwrap();
        if fields_string.contains("positional ref") && obj == &6 {
            obj = fields.region_objects.choose(&mut prng).unwrap();
        }
        if fields_string.contains("positional ref") && obj == &6 {
            obj = fields.region_objects.choose(&mut prng).unwrap();
        }
        if fields_string.contains("positional ref") && obj == &6 {
            obj = fields.region_objects.choose(&mut prng).unwrap();
        }
        if fields_string.contains("positional ref") && obj == &6 {
            obj = fields.region_objects.choose(&mut prng).unwrap();
        }
        let add_strings = match obj {
            1 => get_resource_asteroid(&fields.fields_mods, &defaults.resourceasteroids),
            2 => get_nonresource_asteroid(&fields.fields_mods, &defaults.asteroids),
            3 => get_debris(&fields.fields_mods, &defaults.debris),
            4 => get_lockbox(&fields.fields_mods, &defaults.lockbox),
            5 => get_resource_nebula(&fields.fields_mods, &defaults.nebula),
            6 => get_nonresource_nebula(&fields.fields_mods, &defaults.positionals),
            7 => get_sound_region(&fields.fields_mods, &defaults.sounds),
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
    let obj = fields_mods.resource_roid_weight.choose(&mut prng).unwrap();
    let roid_choice = match obj {
        1 => &resourceasteroids.asteroid_highyield_v1,
        2 => &resourceasteroids.asteroid_highyield_sil_v1,
        3 => &resourceasteroids.asteroid_highyield_niv_v1,
        4 => &resourceasteroids.asteroid_ore_xxl,
        5 => &resourceasteroids.asteroid_ore_xl,
        6 => &resourceasteroids.asteroid_ore_l,
        7 => &resourceasteroids.asteroid_ore_m,
        8 => &resourceasteroids.asteroid_ore_s,
        9 => &resourceasteroids.asteroid_ore_xs,
        10 => &resourceasteroids.asteroid_silicon_xl,
        11 => &resourceasteroids.asteroid_silicon_l,
        12 => &resourceasteroids.asteroid_silicon_m,
        13 => &resourceasteroids.asteroid_silicon_s,
        14 => &resourceasteroids.asteroid_silicon_xs,
        15 => &resourceasteroids.asteroid_nividium_l,
        16 => &resourceasteroids.asteroid_nividium_m,
        17 => &resourceasteroids.asteroid_nividium_s,
        18 => &resourceasteroids.asteroid_nividium_xs,
        19 => &resourceasteroids.asteroid_ice_xl,
        20 => &resourceasteroids.asteroid_ice_l,
        21 => &resourceasteroids.asteroid_ice_m,
        22 => &resourceasteroids.asteroid_ice_s,
        23 => &resourceasteroids.asteroid_ice_xs,
        _ => panic!("broken at resourceroids"),
    };
    let density_factor =
        roid_choice.density_factor as f32 * get_variant_in_range(&fields_mods.density_factor);
    let rotation = roid_choice.rotation as f32 * get_variant_in_range(&fields_mods.rotation);
    let rotationvariation =
        roid_choice.rotationvariation as f32 * get_variant_in_range(&fields_mods.rotationvariation);
    let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
    let minnoisevalue =
        roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
    let maxnoisevalue =
        roid_choice.maxnoisevalue as f32 * get_variant_in_range(&fields_mods.maxnoisevalue);
    let distancefactor = roid_choice.distancefactor;
    let field = format!("<asteroid groupref=\"{}\" densityfactor=\"{}\" rotation=\"{}\" rotationvariation=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\" maxnoisevalue=\"{}\" distancefactor=\"{}\" /> \n", roid_choice.name, density_factor, rotation as i32, rotationvariation as i32, noisescale as i32, minnoisevalue, maxnoisevalue, distancefactor).to_string();
    let resource = format!(
        "<resource ware=\"{}\" yield=\"{}\" />\n",
        roid_choice.resource, roid_choice.resource_amount
    )
    .to_string();
    add_strings.push(field);
    add_strings.push(resource);
    add_strings
}

fn get_nonresource_asteroid(fields_mods: &FieldsMods, asteroids: &NonResourceRoids) -> Vec<String> {
    let mut prng = rand::thread_rng();
    let mut add_strings = Vec::new();
    let obj = fields_mods.nonresource_roid_weight.choose(&mut prng).unwrap();
    let roid_choice = match obj {
        1 => &asteroids.asteroid_xenon_xxl,
        2 => &asteroids.asteroid_xenon_xl,
        3 => &asteroids.asteroid_xenon_l,
        4 => &asteroids.asteroid_xenon_m,
        5 => &asteroids.asteroid_xenon_s,
        6 => &asteroids.asteroid_xenon_xs,
        _ => panic!("broken at nonresourceroids"),
    };
    let density_factor =
        roid_choice.density_factor as f32 * get_variant_in_range(&fields_mods.density_factor);
    let rotation = roid_choice.rotation as f32 * get_variant_in_range(&fields_mods.rotation);
    let rotationvariation =
        roid_choice.rotationvariation as f32 * get_variant_in_range(&fields_mods.rotationvariation);
    let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
    let minnoisevalue =
        roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
    let maxnoisevalue =
        roid_choice.maxnoisevalue as f32 * get_variant_in_range(&fields_mods.maxnoisevalue);
    let distancefactor = roid_choice.distancefactor;
    let field = format!("<asteroid groupref=\"{}\" densityfactor=\"{}\" rotation=\"{}\" rotationvariation=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\" maxnoisevalue=\"{}\" distancefactor=\"{}\" /> \n", roid_choice.name, density_factor, rotation as i32, rotationvariation as i32, noisescale as i32, minnoisevalue, maxnoisevalue, distancefactor).to_string();
    add_strings.push(field);
    add_strings
}

fn get_debris(fields_mods: &FieldsMods, debris: &Debris) -> Vec<String> {
    let mut prng = rand::thread_rng();
    let mut add_strings = Vec::new();
    let obj = fields_mods.debris_weight.choose(&mut prng).unwrap();
    let roid_choice = match obj {
        1 => &debris.debris_xl,
        2 => &debris.debris_l,
        3 => &debris.debris_m,
        4 => &debris.debris_s,
        5 => &debris.debris_station_l,
        6 => &debris.env_debris_station_l_05,
        7 => &debris.debris_teladi_xl,
        8 => &debris.debris_paranid_xl,
        9 => &debris.debris_paranid_l,
        10 => &debris.debris_split_xl,
        11 => &debris.debris_split_s,
        12 => &debris.debris_xenon_xl,
        13 => &debris.debris_xenon_l,
        14 => &debris.debris_xenon_m,
        _ => panic!("broken at debris"),
    };
    let density_factor =
        roid_choice.density_factor as f32 * get_variant_in_range(&fields_mods.density_factor);
    let rotation = roid_choice.rotation as f32 * get_variant_in_range(&fields_mods.rotation);
    let rotationvariation =
        roid_choice.rotationvariation as f32 * get_variant_in_range(&fields_mods.rotationvariation);
    let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
    let minnoisevalue =
        roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
    let maxnoisevalue =
        roid_choice.maxnoisevalue as f32 * get_variant_in_range(&fields_mods.maxnoisevalue);
    let distancefactor = roid_choice.distancefactor;
    let field = format!("<debris groupref=\"{}\" densityfactor=\"{}\" rotation=\"{}\" rotationvariation=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\" maxnoisevalue=\"{}\" distancefactor=\"{}\" /> \n", roid_choice.name, density_factor, rotation as i32, rotationvariation as i32, noisescale as i32, minnoisevalue, maxnoisevalue, distancefactor).to_string();
    add_strings.push(field);
    add_strings
}

fn get_lockbox(fields_mods: &FieldsMods, lockbox: &Lockbox) -> Vec<String> {
    let mut prng = rand::thread_rng();
    let mut add_strings = Vec::new();
    let obj = fields_mods.lockbox_weight.choose(&mut prng).unwrap();
    let roid_choice = match obj {
        1 => &lockbox.lockboxes_rare,
        2 => &lockbox.lockboxes_extra,
        _ => panic!("broken at lockbox"),
    };
    let density_factor =
        roid_choice.density_factor as f32 * get_variant_in_range(&fields_mods.density_factor);
    let rotation = roid_choice.rotation as f32 * get_variant_in_range(&fields_mods.rotation);
    let rotationvariation =
        roid_choice.rotationvariation as f32 * get_variant_in_range(&fields_mods.rotationvariation);
    let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
    let minnoisevalue =
        roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
    let maxnoisevalue =
        roid_choice.maxnoisevalue as f32 * get_variant_in_range(&fields_mods.maxnoisevalue);
    let distancefactor = roid_choice.distancefactor;
    let field = format!("<object groupref=\"{}\" densityfactor=\"{}\" rotation=\"{}\" rotationvariation=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\" maxnoisevalue=\"{}\" distancefactor=\"{}\" /> \n", roid_choice.name, density_factor, rotation as i32, rotationvariation as i32, noisescale as i32, minnoisevalue, maxnoisevalue, distancefactor).to_string();
    add_strings.push(field);
    add_strings
}

fn get_resource_nebula(fields_mods: &FieldsMods, nebula: &Nebula) -> Vec<String> {
    let mut prng = rand::thread_rng();
    let mut add_strings = Vec::new();
    let obj = fields_mods.resource_neb_weight.choose(&mut prng).unwrap();
    let roid_choice = match obj {
        1 => &nebula.fog_smallstones_v1_macro,
        2 => &nebula.fogpattern_v2_macro,
        3 => &nebula.fogtest_nebula_vol_macro,
        4 => &nebula.fogvolume_small_macro,
        _ => panic!("broken at resource nebulas"),
    };
    let localred = roid_choice.localred;
    let localgreen = roid_choice.localgreen;
    let localblue = roid_choice.localblue;
    let localdensity = roid_choice.localdensity;
    let uniformred = roid_choice.uniformred;
    let uniformgreen = roid_choice.uniformgreen;
    let uniformblue = roid_choice.uniformblue;
    let uniformdensity = roid_choice.uniformdensity;
    let backgroundfog = roid_choice.backgroundfog;
    let resources = &roid_choice.resources;
    let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
    let minnoisevalue =
        roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
    let maxnoisevalue =
        roid_choice.maxnoisevalue as f32 * get_variant_in_range(&fields_mods.maxnoisevalue);
    let field = format!("<nebula ref=\"{}\" localred=\"{}\" localgreen=\"{}\" localblue=\"{}\" localdensity=\"{}\" uniformred=\"{}\" uniformgreen=\"{}\" uniformblue=\"{}\" uniformdensity=\"{}\" backgroundfog=\"{}\" resources=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\"  maxnoisevalue=\"{}\" /> \n", roid_choice.name, localred, localgreen, localblue, localdensity, uniformred, uniformgreen, uniformblue, uniformdensity, backgroundfog, resources, noisescale as i32, minnoisevalue,  maxnoisevalue).to_string();
    let resource = format!(
        "<resource ware=\"{}\" yield=\"{}\" />\n",
        roid_choice.resources,
        ["high", "medium"].choose(&mut prng).unwrap()
    )
    .to_string();
    add_strings.push(field);
    add_strings.push(resource);
    add_strings
}

fn get_nonresource_nebula(fields_mods: &FieldsMods, positionals: &Positionals) -> Vec<String> {
    let mut prng = rand::thread_rng();
    let mut add_strings = Vec::new();
    let objone = fields_mods.positional_weight.choose(&mut prng).unwrap();
    let objtwo = fields_mods.positional_weight.choose(&mut prng).unwrap();
    let objects = [objone, objtwo];
    let mut field = "".to_string();
    for obj in objects.iter() {
        let roid_choice = match obj {
            1 => &positionals.fog_outside_set3_macro,  //safe
            2 => &positionals.fog_outside_set1_whiteblue_macro,  //ok
            3 => &positionals.fog_outside_set1_lightbrown_macro,  //dead to me
            4 => &positionals.fog_outside_set1_big_lightorange_macro,  //ugly
            // 5 => &positionals.fog_outside_set1_lightorange_macro,  //broken
            // 6 => &positionals.fog_outside_set1_lightblue_macro,  //broken
            7 => &positionals.fog_outside_set1_big_lightpurple_macro,  //ok i guess, but resource heavy and probably needs lower density
            8 => &positionals.fog_outside_set1_blue_macro,  //ok
            9 => &positionals.fog_outside_set1_burgundy_macro,  // safe
            10 => &positionals.fog_outside_set1_green_macro,  //ok
            11 => &positionals.fog_outside_set1_darkblue_macro,  // safe
            12 => &positionals.fog_outside_set1_red_macro,  // safe
            // 13 => &positionals.fog_outside_set1_grey_macro,  // broken
            14 => &positionals.fog_outside_set1_dust_macro,  // ok but heavy
            // 15 => &positionals.fog_outside_set1_lightbrown2_macro,  // broken
            _ => panic!("broken at plain fog"),
        };
        let lodrule = &roid_choice.lodrule;
        let density_factor =
            roid_choice.density_factor as f32 * get_variant_in_range(&fields_mods.density_factor);
        let rotation = roid_choice.rotation as f32 * get_variant_in_range(&fields_mods.rotation);
        let rotationvariation =
            roid_choice.rotationvariation as f32 * get_variant_in_range(&fields_mods.rotationvariation);
        let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
        let minnoisevalue =
            roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
        let maxnoisevalue =
            roid_choice.maxnoisevalue as f32 * get_variant_in_range(&fields_mods.maxnoisevalue);
        let distancefactor = roid_choice.distancefactor;
        field.push_str(format!("<positional ref=\"{}\" lodrule=\"{}\" densityfactor=\"{}\" rotation=\"{}\" rotationvariation=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\" maxnoisevalue=\"{}\" distancefactor=\"{}\" /> \n", roid_choice.name, lodrule, density_factor, rotation as i32, rotationvariation as i32, noisescale as i32, minnoisevalue, maxnoisevalue, distancefactor).as_str());
    }
    add_strings.push(field);
    add_strings
}

fn get_sound_region(fields_mods: &FieldsMods, sounds: &Sounds) -> Vec<String> {
    let mut prng = rand::thread_rng();
    let mut add_strings = Vec::new();
    let obj = fields_mods.sound_weight.choose(&mut prng).unwrap();
    let roid_choice = match obj {
        1 => &sounds.zoned_sound_01,
        2 => &sounds.zoned_sound_02,
        3 => &sounds.zoned_sound_03,
        4 => &sounds.zoned_sound_04,
        5 => &sounds.zoned_sound_05,
        6 => &sounds.zoned_sound_06,
        7 => &sounds.zoned_sound_07,
        8 => &sounds.legacy_music_pool,
        _ => panic!("broken at sound"),
    };
    let noisescale = roid_choice.noisescale as f32 * get_variant_in_range(&fields_mods.noisescale);
    let minnoisevalue =
        roid_choice.minnoisevalue as f32 * get_variant_in_range(&fields_mods.minnoisevalue);
    let playtime = roid_choice.playtime;
    let field = format!(
        "<ambientsound soundid=\"{}\" noisescale=\"{}\" minnoisevalue=\"{}\" playtime=\"{}\" /> \n",
        roid_choice.name, noisescale as i32, minnoisevalue, playtime
    )
    .to_string();
    add_strings.push(field);
    add_strings
}

fn convert_to_clusters_file (connections_str: String, clusters_str: String) -> String {
    // let connections_str =
    //     include_str!("E:/Rust/Projects/OutPut Files/Regions/connections.xml");
    let mut convec: Vec<String> = vec![];
    let mut out_string = "".to_string();
    let mut regioncount:usize = 0;
    for connections in connections_str.split_terminator(" new region") {
        convec.push(connections.to_string());
    }
    for cluster in clusters_str.split_terminator("</connection>") {
        if cluster.contains("<component connection=\"space\"") | cluster.contains("ref=\"sectors\"")
        {
            out_string.push_str(cluster);
            out_string.push_str("</connection>");
        }
        if cluster.contains("<connections>") {
             out_string.push_str(&convec[regioncount]);
             regioncount += 1;
        }
    }
    out_string.push_str(
        "          </connections>
    </macro>
  </add>
</diff>",
    );
    out_string
    // let mut outputfile =
    //     File::create("E:/Rust/Projects/OutPut Files/Regions/clusters.xml")
    //         .unwrap();
    // outputfile
    // outputfile.write_all(out_string.as_bytes()).unwrap();
}
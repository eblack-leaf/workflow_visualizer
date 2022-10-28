use std::path::{Path, PathBuf};

use regex::Regex;

// const Info for stability in shaders and programs
// application level layout for bindings
pub mod bindings {
    pub const VIEWPORT: u32 = 0;
    pub const RASTERIZATION: u32 = 1;
}

pub mod shader_locations {}

pub mod buffers {
    pub const TEXT_VERTEX: u32 = 0;
    pub const TEXT_INSTANCE: u32 = 1;
}

pub mod attributes {
    pub const TEXT_VERTEX: u32 = 0;
    pub const TEXT_INSTANCE: u32 = 1;
    pub const TEXT_COLOR: u32 = 0;
    pub const TEXT_POSITION: u32 = 1;
    pub const TEXT_AREA: u32 = 2;
    pub const TEXT_DEPTH: u32 = 3;
    pub const TEXT_RASTERIZATION_KEY: u32 = 4;
}

pub fn generate_wgsl(template: PathBuf) {
    let template_text = std::fs::read_to_string(template.as_path()).expect("could not read template");
    let regex = Regex::new(r"(\{\{\w+\}\})").expect("could not construct regex");
    for cap in regex.captures_iter(template_text.as_str()) {
        dbg!(&cap[0]);
        let template_text = template_text.replace(&cap[1], match_replacement(cap[1].to_string()).as_str());
    }
    let template_parent = template.parent().expect("no parent from template").parent().expect("no parent could be found");
    let generated_path = template_parent.join("generated").join(template.file_name().expect("no template file name"));
    dbg!(&generated_path);
    std::fs::write(generated_path, template_text.as_bytes()).expect("could not write generated shader");
}

pub fn match_replacement(input: String) -> String {
    match input.as_str() {
        "{{bindings::VIEWPORT}}" => bindings::VIEWPORT.to_string(),
        "{{bindings::RASTERIZATION}}" => bindings::RASTERIZATION.to_string(),
        "{{attributes::TEXT_COLOR}}" => attributes::TEXT_COLOR.to_string(),
        "{{attributes::TEXT_POSITION}}" => attributes::TEXT_POSITION.to_string(),
        "{{attributes::TEXT_AREA}}" => attributes::TEXT_AREA.to_string(),
        "{{attributes::TEXT_DEPTH}}" => attributes::TEXT_DEPTH.to_string(),
        "{{attributes::TEXT_RASTERIZATION_KEY}}" => attributes::TEXT_RASTERIZATION_KEY.to_string(),
        _ => "".to_string()
    }
}

// #[test]
// pub fn test() {
//     generate_wgsl(PathBuf::from("/home/omi-voshuli/note-ifications/app/shaders/templates/text.wgsl"));
// }

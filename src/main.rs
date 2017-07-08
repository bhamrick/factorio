#[macro_use]
extern crate lazy_static;
extern crate nom;
extern crate rulinalg;

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::str;
use std::str::FromStr;
use std::io::prelude::*;
use std::vec::Vec;

use nom::*;

use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Resource {
    Coal,
    Water,
    SolidFuel,
    HeavyOil,
    LightOil,
    Petroleum,
    Steam,
}

lazy_static! {
    static ref RESOURCE_NAMES : Vec<(Resource, &'static str)> = vec![
        (Resource::Coal, "Coal"),
        (Resource::Water, "Water"),
        (Resource::SolidFuel, "Solid fuel"),
        (Resource::HeavyOil, "Heavy oil"),
        (Resource::LightOil, "Light oil"),
        (Resource::Petroleum, "Petroleum"),
        (Resource::Steam, "Steam"),
    ];

    static ref PROTO_BUILDINGS : Vec<ProtoBuilding<'static>> = vec![
        ProtoBuilding {
            name: "Boiler",
            recipes: vec![
                "Burning solid fuel",
            ],
            energy_consumption: 0.0,
            crafting_speed: 1.0,
        },
        ProtoBuilding {
            name: "Oil Refinery",
            recipes: vec![
                "Coal liquefaction",
            ],
            energy_consumption: 420.0,
            crafting_speed: 1.0,
        },
        ProtoBuilding {
            name: "Chemical Plant",
            recipes: vec![
                "Heavy oil cracking",
                "Solid fuel (Light oil)",
                "Solid fuel (Petroleum)",
            ],
            energy_consumption: 210.0,
            crafting_speed: 1.25,
        },
    ];

    static ref PROTO_RECIPES : Vec<ProtoRecipe<'static>> = vec![
        // Recipes required for the coal liquefaction line
        ProtoRecipe {
            name: "Coal liquefaction",
            inputs: vec![
                (Resource::Coal, 10.0),
                (Resource::HeavyOil, 25.0),
                (Resource::Steam, 50.0),
            ],
            outputs: vec![
                (Resource::HeavyOil, 35.0),
                (Resource::LightOil, 15.0),
                (Resource::Petroleum, 20.0),
            ],
            time: 5.0,
        },
        ProtoRecipe {
            name: "Heavy oil cracking",
            inputs: vec![
                (Resource::HeavyOil, 40.0),
                (Resource::Water, 30.0),
            ],
            outputs: vec![
                (Resource::LightOil, 30.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Solid fuel (Light oil)",
            inputs: vec![
                (Resource::LightOil, 10.0),
            ],
            outputs: vec![
                (Resource::SolidFuel, 1.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Solid fuel (Petroleum)",
            inputs: vec![
                (Resource::Petroleum, 20.0),
            ],
            outputs: vec![
                (Resource::SolidFuel, 1.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Burning solid fuel",
            inputs: vec![
                (Resource::Water, 60.0),
                (Resource::SolidFuel, 0.144),
            ],
            outputs: vec![
                (Resource::Steam, 60.0),
            ],
            time: 1.0,
        },
    ];

    static ref MODULE_NAMES : Vec<(Module, &'static str)> = vec![
        (Module::Productivity1, "Productivity 1"),
        (Module::Productivity1, "Productivity1"),
        (Module::Productivity1, "Productivity"),
        (Module::Productivity2, "Productivity 2"),
        (Module::Productivity2, "Productivity2"),
        (Module::Productivity3, "Productivity 3"),
        (Module::Productivity3, "Productivity3"),
        (Module::Speed1, "Speed 1"),
        (Module::Speed1, "Speed1"),
        (Module::Speed1, "Speed"),
        (Module::Speed2, "Speed 2"),
        (Module::Speed2, "Speed2"),
        (Module::Speed3, "Speed 3"),
        (Module::Speed3, "Speed3"),
        (Module::Efficiency1, "Efficiency 1"),
        (Module::Efficiency1, "Efficiency1"),
        (Module::Efficiency1, "Efficiency"),
        (Module::Efficiency2, "Efficiency 2"),
        (Module::Efficiency2, "Efficiency2"),
        (Module::Efficiency3, "Efficiency 3"),
        (Module::Efficiency3, "Efficiency3"),
    ];
}

impl Resource {
    fn from_str(name: &str) -> Result<Resource, InputError> {
        for &(resource, resource_name) in RESOURCE_NAMES.iter() {
            if name.to_lowercase() == resource_name.to_lowercase() {
                return Ok(resource);
            }
        }
        Err(InputError::new("Unknown resource"))
    }
}

#[derive(Debug, Clone)]
struct ResourceLine<'a> {
    name: &'a str,
    resource_type: Resource,
    index: usize,
}

#[derive(Debug, Clone)]
struct Building<'a> {
    name: &'a str,
    recipe: Recipe<'a>,
    energy_consumption: f32,
    crafting_speed: f32,
    modules: Vec<(Module, i16)>,
    index: usize,
}

impl<'a> Building<'a> {
    fn analysis_coefficients(&self) -> HashMap<usize, f32> {
        let mut coefficients = HashMap::new();
        let modified_recipe = self.modified_recipe();
        // Inputs have positive coefficients and outputs have negative coefficients.
        // This is so that the sign of the resource line is positive for outputs and
        // negative for inputs (since we set the diagonal coefficient to be 1).
        for &(ref line, qty) in modified_recipe.inputs.iter() {
            match coefficients.entry(line.index) {
                Entry::Occupied(mut ent) => {
                    let mut coeff : &mut f32 = ent.get_mut();
                    *coeff += qty / modified_recipe.time;
                },
                Entry::Vacant(ent) => {
                    ent.insert(qty / modified_recipe.time);
                },
            }
        }
        for &(ref line, qty) in modified_recipe.outputs.iter() {
            match coefficients.entry(line.index) {
                Entry::Occupied(mut ent) => {
                    let mut coeff : &mut f32 = ent.get_mut();
                    *coeff += -qty / modified_recipe.time;
                },
                Entry::Vacant(ent) => {
                    ent.insert(-qty / modified_recipe.time);
                },
            }
        }
        coefficients
    }

    fn modified_recipe(&self) -> Recipe<'a> {
        let modifiers = Modifiers::from_modules(&self.modules);
        let mut modified_outputs = Vec::new();
        for &(ref line, amount) in self.recipe.outputs.iter() {
            modified_outputs.push((line.clone(), amount * (1.0 + modifiers.productivity)));
        }
        let modified_crafting_speed = self.crafting_speed * (1.0 + modifiers.speed);
        let modified_time = self.recipe.time / modified_crafting_speed;
        Recipe {
            name: self.recipe.name,
            inputs: self.recipe.inputs.clone(),
            outputs: modified_outputs,
            time: modified_time,
        }
    }

    fn modified_energy_consumption(&self) -> f32 {
        let mut modifiers = Modifiers::from_modules(&self.modules);
        if modifiers.energy < 0.2 {
            modifiers.energy = 0.2;
        }
        self.energy_consumption * modifiers.energy
    }
}

#[derive(Debug, Clone, Copy)]
enum Module {
    Productivity1,
    Productivity2,
    Productivity3,
    Speed1,
    Speed2,
    Speed3,
    Efficiency1,
    Efficiency2,
    Efficiency3,
}

impl Module {
    fn display_name(&self) -> &'static str {
        match *self {
            Module::Productivity1 => "Productivity 1",
            Module::Productivity2 => "Productivity 2",
            Module::Productivity3 => "Productivity 3",
            Module::Speed1 => "Speed 1",
            Module::Speed2 => "Speed 2",
            Module::Speed3 => "Speed 3",
            Module::Efficiency1 => "Efficiency 1",
            Module::Efficiency2 => "Efficiency 2",
            Module::Efficiency3 => "Efficiency 3",
        }
    }

    fn from_name(name: &str) -> Result<Module, InputError> {
        for &(module_type, module_name) in MODULE_NAMES.iter() {
            if name.to_lowercase() == module_name.to_lowercase() {
                return Ok(module_type)
            }
        }
        Err(InputError::new("Unknown module"))
    }
}

#[derive(Debug, Clone)]
struct Modifiers {
    speed: f32,
    productivity: f32,
    energy: f32,
}

impl Modifiers {
    fn new() -> Modifiers {
        Modifiers {
            speed: 0.0,
            productivity: 0.0,
            energy: 1.0,
        }
    }

    fn from_modules<I>(modules: &[(Module, I)]) -> Modifiers where I: Copy + Into<f32> {
        let mut modifiers = Modifiers::new();
        for &(module, count) in modules {
            match module {
                Module::Productivity1 => {
                    modifiers.speed += (-0.15) * count.into();
                    modifiers.productivity += 0.04 * count.into();
                    modifiers.energy += 0.40 * count.into();
                },
                Module::Productivity2 => {
                    modifiers.speed += (-0.15) * count.into();
                    modifiers.productivity += 0.06 * count.into();
                    modifiers.energy += 0.60 * count.into();
                },
                Module::Productivity3 => {
                    modifiers.speed += (-0.15) * count.into();
                    modifiers.productivity += 0.10 * count.into();
                    modifiers.energy += 0.80 * count.into();
                },
                Module::Speed1 => {
                    modifiers.speed += 0.20 * count.into();
                    modifiers.energy += 0.50 * count.into();
                },
                Module::Speed2 => {
                    modifiers.speed += 0.30 * count.into();
                    modifiers.energy += 0.60 * count.into();
                },
                Module::Speed3 => {
                    modifiers.speed += 0.50 * count.into();
                    modifiers.energy += 0.70 * count.into();
                },
                Module::Efficiency1 => {
                    modifiers.energy += (-0.30) * count.into();
                },
                Module::Efficiency2 => {
                    modifiers.energy += (-0.40) * count.into();
                },
                Module::Efficiency3 => {
                    modifiers.energy += (-0.50) * count.into();
                },
            }
        }
        modifiers
    }
}

#[derive(Debug, Clone)]
struct ProtoBuilding<'a> {
    name: &'a str,
    recipes: Vec<&'a str>,
    energy_consumption: f32,
    crafting_speed: f32,
}

impl<'a> ProtoBuilding<'a> {
    fn from_name(name: &'a str) -> Option<ProtoBuilding<'a>> {
        for ref proto in PROTO_BUILDINGS.iter() {
            if proto.name.to_lowercase() == name.to_lowercase() {
                return Some((*proto).clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Recipe<'a> {
    name: &'a str,
    inputs: Vec<(ResourceLine<'a>, f32)>,
    outputs: Vec<(ResourceLine<'a>, f32)>,
    time: f32,
}

#[derive(Debug, Clone)]
struct ProtoRecipe<'a> {
    name: &'a str,
    inputs: Vec<(Resource, f32)>,
    outputs: Vec<(Resource, f32)>,
    time: f32,
}

impl<'a> ProtoRecipe<'a> {
    fn from_name(name: &'a str) -> Option<ProtoRecipe<'a>> {
        for ref proto in PROTO_RECIPES.iter() {
            if proto.name.to_lowercase() == name.to_lowercase() {
                return Some((*proto).clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct InputError {
    message: String,
}

impl From<std::str::Utf8Error> for InputError {
    fn from(_ : std::str::Utf8Error) -> InputError {
        InputError::new("Unicode error")
    }
}

impl InputError {
    fn new(msg: &str) -> InputError {
        InputError {
            message: String::from_str(msg).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct AnalyzeError {
    message: String,
}

impl AnalyzeError {
    fn new(msg: &str) -> AnalyzeError {
        AnalyzeError {
            message: String::from_str(msg).unwrap(),
        }
    }
}

impl From<rulinalg::error::Error> for AnalyzeError {
    fn from(_ : rulinalg::error::Error) -> AnalyzeError {
        AnalyzeError::new("Error solving system")
    }
}

#[derive(Debug, Clone)]
struct Design<'a> {
    resource_lines: HashMap<&'a str, ResourceLine<'a>>,
    buildings: Vec<Building<'a>>,
    input_lines: HashSet<&'a str>,
    output_lines: HashSet<&'a str>,
    normalized_var: Option<usize>,
    normalized_value: f32,
    next_index: usize,
}

impl<'a> Design<'a> {
    fn new() -> Design<'a> {
        Design {
            resource_lines: HashMap::new(),
            buildings: Vec::new(),
            input_lines: HashSet::new(),
            output_lines: HashSet::new(),
            normalized_var: None,
            normalized_value: 1.0,
            next_index: 0,
        }
    }

    fn get_line(&mut self, resource_type: Resource, name: &'a str) -> Result<ResourceLine<'a>, InputError> {
        match self.resource_lines.entry(name) {
            Entry::Occupied(ent) => {
                let existing_line = ent.get();
                if existing_line.resource_type == resource_type {
                    Ok(existing_line.clone())
                } else {
                    Err(InputError::new("Resource type mismatch"))
                }
            },
            Entry::Vacant(ent) => {
                let new_line = ResourceLine {
                    name: name,
                    resource_type: resource_type,
                    index: self.next_index,
                };
                self.next_index += 1;
                ent.insert(new_line.clone());
                Ok(new_line)
            },
        }
    }

    fn from_data(data: Vec<Data<'a>>) -> Result<Design<'a>, InputError> {
        let mut design = Design::new();
        for datum in data {
            if datum.value == "Inputs" {
                // Read input lines
                for input_datum in datum.children {
                    let resource_type = Resource::from_str(input_datum.value)?;
                    for input_line in input_datum.children {
                        if !input_line.children.is_empty() {
                            return Err(InputError::new("Unexpected child of input line name"));
                        }
                        let resource_line = design.get_line(resource_type, input_line.value)?;
                        design.input_lines.insert(resource_line.name);
                    }
                }
            } else if datum.value == "Outputs" {
                // Read output lines
                for output_datum in datum.children {
                    let resource_type = Resource::from_str(output_datum.value)?;
                    for output_line in output_datum.children {
                        if !output_line.children.is_empty() {
                            return Err(InputError::new("Unexpected child of output line name"));
                        }
                        let resource_line = design.get_line(resource_type, output_line.value)?;
                        design.output_lines.insert(resource_line.name);
                        if design.normalized_var == None {
                            design.normalized_var = Some(resource_line.index);
                        }
                    }
                }
            } else {
                // Read a building description
                let proto_building = ProtoBuilding::from_name(datum.value)
                    .ok_or(InputError::new("Unknown building"))?;
                if datum.children.is_empty() {
                    return Err(InputError::new("Found building with no recipe"));
                }
                let recipe_name = datum.children[0].value;
                let proto_recipe = ProtoRecipe::from_name(recipe_name)
                    .ok_or(InputError::new("Unknown recipe"))?;
                let mut required_inputs : HashMap<Resource, f32> = proto_recipe.inputs.iter().cloned().collect();
                let mut required_outputs : HashMap<Resource, f32> = proto_recipe.outputs.iter().cloned().collect();
                let mut line_inputs = Vec::new();
                let mut line_outputs = Vec::new();
                let mut modules = Vec::new();
                for property_datum in datum.children[1..].iter() {
                    if property_datum.value == "Inputs" {
                        for input_datum in property_datum.children.iter() {
                            let resource_type = Resource::from_str(input_datum.value)?;

                            if input_datum.children.len() != 1 {
                                return Err(InputError::new("Only single line per input is supported."));
                            }
                            let line_name = input_datum.children[0].value;

                            let resource_line = design.get_line(resource_type, line_name)?;

                            match required_inputs.remove(&resource_type) {
                                Some(qty) => {
                                    line_inputs.push((resource_line, qty));
                                },
                                None => {
                                    return Err(InputError::new("Invalid recipe input"));
                                },
                            }
                        }
                    } else if property_datum.value == "Outputs" {
                        for output_datum in property_datum.children.iter() {
                            let resource_type = Resource::from_str(output_datum.value)?;

                            if output_datum.children.len() != 1 {
                                return Err(InputError::new("Only single line per output is supported."));
                            }
                            let line_name = output_datum.children[0].value;

                            let resource_line = design.get_line(resource_type, line_name)?;

                            match required_outputs.remove(&resource_type) {
                                Some(qty) => {
                                    line_outputs.push((resource_line, qty));
                                },
                                None => {
                                    return Err(InputError::new("Invalid recipe output"));
                                },
                            }
                        }
                    } else if property_datum.value == "Modules" {
                        for module_datum in property_datum.children.iter() {
                            let module_type = Module::from_name(module_datum.value)?;

                            let module_count : i16;
                            if module_datum.children.len() == 0 {
                                module_count = 1;
                            } else if module_datum.children.len() == 1 {
                                module_count = match module_datum.children[0].value.parse() {
                                    Ok(n) => n,
                                    Err(_) => return Err(InputError::new("Invalid module count")),
                                };
                            } else {
                                return Err(InputError::new("Invalid module count"));
                            }

                            modules.push((module_type, module_count));
                        }
                    }
                }

                if !required_inputs.is_empty() {
                    return Err(InputError::new("Not all inputs are filled"));
                }
                if !required_outputs.is_empty() {
                    return Err(InputError::new("Not all outputs are filled"));
                }

                let recipe = Recipe {
                    name: proto_recipe.name,
                    inputs: line_inputs,
                    outputs: line_outputs,
                    time: proto_recipe.time,
                };

                let building_index = design.next_index;
                design.next_index += 1;

                let building = Building {
                    name: proto_building.name,
                    recipe: recipe,
                    energy_consumption: proto_building.energy_consumption,
                    crafting_speed: proto_building.crafting_speed,
                    modules: modules,
                    index: building_index,
                };
                design.buildings.push(building);
            }
        }
        Ok(design)
    }

    fn analyze(&self) -> Result<Vec<f32>, AnalyzeError> {
        // If the specified design is fully specified, then there will be one
        // set of nonzero rates (up to scalar factors) that determines how fast
        // each of the parts is working. The vector returned in that case will
        // have the following values:
        //
        // For each building, how many copies of that building are operating
        // For each resource line, how much of that resource is netted per second
        //
        // The location of these values is defined by the `index` attribute of
        // each building and resource line.
        //
        // To compute these, we build up a system of linear equations. There are
        // two types of equations:
        //
        // 1) Each resource line variable is the sum of all contributions of attached
        //    buildings (positive contribution for outputs, negative for inputs).
        // 2) For each resource line that is not an input or output, that resource line
        //    must net 0.
        let mut io_equations : HashMap<usize, Vec<f32>> = HashMap::new();
        let num_variables = self.next_index;
        for line in self.resource_lines.values() {
            let mut equation = Vec::new();
            equation.resize(num_variables, 0.0);
            equation[line.index] = 1.0;
            io_equations.insert(line.index, equation);
        }
        for building in self.buildings.iter() {
            let coefficients = building.analysis_coefficients();
            for (&line_index, &line_coeff) in coefficients.iter() {
                if let Some(eq) = io_equations.get_mut(&line_index) {
                    eq[building.index] = line_coeff;
                }
            }
        }

        let mut matrix_data : Vec<f32> = Vec::new();
        let mut rhs_data : Vec<f32> = Vec::new();

        // Main balance equations
        for eq in io_equations.values() {
            matrix_data.extend(eq.iter().cloned());
            rhs_data.push(0.0);
        }
        // Equations to force all non-input/output lines to 0
        for line in self.resource_lines.values() {
            if !self.input_lines.contains(line.name) && !self.output_lines.contains(line.name) {
                let mut equation = Vec::new();
                equation.resize(num_variables, 0.0);
                equation[line.index] = 1.0;

                matrix_data.extend(equation.iter().cloned());
                rhs_data.push(0.0);
            }
        }
        // Equation to normalize the result
        let mut norm_eq = Vec::new();
        let norm_var_index = self.normalized_var.ok_or(AnalyzeError::new("No normalized variable set"))?;
        norm_eq.resize(num_variables, 0.0);
        norm_eq[norm_var_index] = 1.0;
        matrix_data.extend(norm_eq.iter().cloned());
        rhs_data.push(self.normalized_value);

        let matrix = Matrix::new(rhs_data.len(), num_variables, matrix_data);
        let rhs = Vector::new(rhs_data);
        let result = matrix.solve(rhs)?;

        Ok(result.into_vec())
    }

    fn print_results(&self, analysis : Vec<f32>) {
        println!("Inputs:");
        for input_name in self.input_lines.iter() {
            let input_line = self.resource_lines.get(input_name).unwrap();
            println!("    {}: {} per sec", input_line.name, -analysis[input_line.index]);
        }
        println!("");
        println!("Outputs:");
        for output_name in self.output_lines.iter() {
            let output_line = self.resource_lines.get(output_name).unwrap();
            println!("    {}: {} per sec", output_line.name, analysis[output_line.index]);
        }
        let mut total_energy = 0.0;
        for building in self.buildings.iter() {
            println!("");
            println!("{}", building.name);
            println!("    {}", building.recipe.name);
            println!("    Modules:");
            for &(module_type, module_count) in building.modules.iter() {
                println!("        {}: {}", module_type.display_name(), module_count);
            }
            let building_count = analysis[building.index];
            println!("    Count: {}", building_count);
            let building_energy = building.modified_energy_consumption() * building_count;
            println!("    Energy cost: {} kW", building_energy);
            total_energy += building_energy;

            let modified_recipe = building.modified_recipe();
            println!("    Inputs:");
            for &(ref input_line, qty) in modified_recipe.inputs.iter() {
                let input_rate = qty * building_count / modified_recipe.time;
                println!("        {}: {} per sec", input_line.name, input_rate);
            }

            println!("    Outputs:");
            for &(ref output_line, qty) in modified_recipe.outputs.iter() {
                let output_rate = qty * building_count / modified_recipe.time;
                println!("        {}: {} per sec", output_line.name, output_rate);
            }
        }
        println!("");
        println!("Total energy cost: {} kW", total_energy);
    }
}

#[derive(Debug, Clone)]
struct Data<'a> {
    value: &'a str,
    children: Vec<Data<'a>>,
}

impl<'a> Data<'a> {
    fn leaf(value: &'a str) -> Data<'a> {
        return Data {
            value: value,
            children: Vec::new(),
        }
    }

    fn from_bytes(input: &'a [u8]) -> Result<Vec<Data<'a>>, Err<u32>> {
        let iresult = complete!(input, many0!(call!(node, b"")));
        iresult.to_result()
    }
}

// Remove comments (any characters after a # in a line),
// trailing whitespace, and any pure-whitespace lines.
fn clean(data: Vec<u8>) -> Vec<u8> {
    let mut cleaned_contents = Vec::new();
    let mut spaces = Vec::new();
    let mut in_comment = false;
    let mut line_filled = false;

    for b in data {
        if b == b'\n' {
            if line_filled {
                cleaned_contents.push(b'\n');
            }
            spaces.clear();
            in_comment = false;
            line_filled = false;
        } else if b == b' ' || b == b'\t' {
            if !in_comment {
                spaces.push(b);
            }
        } else if b == b'#' {
            in_comment = true;
        } else {
            if !in_comment {
                cleaned_contents.extend(spaces.iter());
                cleaned_contents.push(b);
                line_filled = true;
                spaces.clear();
            }
        }
    }

    return cleaned_contents;
}

fn match_indentation<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], ()> {
    do_parse!(input,
        verify!(opt!(is_a!(" \t")), |line_ind: Option<&[u8]>| line_ind.unwrap_or(&[]) == indentation) >>
        (())
    )
}

fn deeper_indentation<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], &'a [u8]> {
    do_parse!(input,
        new_indentation: verify!(
            map!(opt!(is_a!(" \t")), |x: Option<&'a [u8]>| x.unwrap_or(b"")),
            |line_ind: &[u8]| line_ind.starts_with(indentation) && line_ind != indentation
        ) >>
        (new_indentation)
    )
}

fn inline_node<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], Data<'a>> {
    do_parse!(input, 
        call!(match_indentation, indentation) >>
        value: map_res!(is_not!(":\n"), std::str::from_utf8) >>
        tag!(":") >>
        is_a!(" \t") >>
        children: separated_list!(
                do_parse!(
                    tag!(",") >>
                    opt!(is_a!(" \t")) >>
                    (())
                ),
                map!(map_res!(is_not!(",\n"), std::str::from_utf8), Data::leaf)
            ) >>
        tag!("\n") >>
        (Data {
            value: value,
            children: children,
        })
    )
}

fn nested_node<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], Data<'a>> {
    do_parse!(input,
        call!(match_indentation, indentation) >>
        value: map_res!(is_not!(":\n"), std::str::from_utf8) >>
        tag!("\n") >>
        children: opt!(do_parse!(
            new_indentation: peek!(call!(deeper_indentation, indentation)) >>
            children: many1!(call!(node, new_indentation)) >>
            (children)
        )) >>
        (Data {
            value: value,
            children: children.unwrap_or_else(Vec::new),
        })
    )
}

fn node<'a, 'b>(input: &'a [u8], indentation: &'b [u8]) -> IResult<&'a [u8], Data<'a>> {
    alt!(input, call!(inline_node, indentation) | call!(nested_node, indentation))
}

fn main() {
    let mut contents = Vec::new();
    let mut input = File::open("coal_to_solid").unwrap();
    input.read_to_end(&mut contents).unwrap();

    let clean_contents = clean(contents);

    let parsed_data = Data::from_bytes(&clean_contents).unwrap();
    let design = Design::from_data(parsed_data).unwrap();

    let analysis = design.analyze().unwrap();

    design.print_results(analysis);
}

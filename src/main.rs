#[macro_use]
extern crate lazy_static;
extern crate nom;
extern crate rulinalg;

use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
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
}

impl Resource {
    fn from_str(name: &str) -> Result<Resource, InputError> {
        for &(resource, resource_name) in RESOURCE_NAMES.iter() {
            // TODO: Make this comparison case insensitive
            if name == resource_name {
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
    index: usize,
}

impl<'a> Building<'a> {
    fn analysis_coefficients(&self) -> HashMap<usize, f32> {
        let mut coefficients = HashMap::new();
        // TODO: Support modules
        let recipe_multiplier = self.crafting_speed / self.recipe.time;
        // Inputs have positive coefficients and outputs have negative coefficients.
        // This is so that the sign of the resource line is positive for outputs and
        // negative for inputs (since we set the diagonal coefficient to be 1).
        for &(ref line, qty) in self.recipe.inputs.iter() {
            match coefficients.entry(line.index) {
                Entry::Occupied(mut ent) => {
                    let mut coeff : &mut f32 = ent.get_mut();
                    *coeff += recipe_multiplier * qty;
                },
                Entry::Vacant(ent) => {
                    ent.insert(recipe_multiplier * qty);
                },
            }
        }
        for &(ref line, qty) in self.recipe.outputs.iter() {
            match coefficients.entry(line.index) {
                Entry::Occupied(mut ent) => {
                    let mut coeff : &mut f32 = ent.get_mut();
                    *coeff += -recipe_multiplier * qty;
                },
                Entry::Vacant(ent) => {
                    ent.insert(-recipe_multiplier * qty);
                },
            }
        }
        coefficients
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
            // TODO: Make this case insensitive
            if proto.name == name {
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
            // TODO: Make this case insensitive
            if proto.name == name {
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
            if datum.value == b"Inputs" {
                // Read input lines
                for input_datum in datum.children {
                    let resource_type_str = std::str::from_utf8(input_datum.value)?;
                    let resource_type = Resource::from_str(resource_type_str)?;
                    for input_line in input_datum.children {
                        if !input_line.children.is_empty() {
                            return Err(InputError::new("Unexpected child of input line name"));
                        }
                        let input_line_name = std::str::from_utf8(input_line.value)?;
                        let resource_line = design.get_line(resource_type, input_line_name)?;
                        design.input_lines.insert(resource_line.name);
                    }
                }
            } else if datum.value == b"Outputs" {
                // Read output lines
                for output_datum in datum.children {
                    let resource_type_str = std::str::from_utf8(output_datum.value)?;
                    let resource_type = Resource::from_str(resource_type_str)?;
                    for output_line in output_datum.children {
                        if !output_line.children.is_empty() {
                            return Err(InputError::new("Unexpected child of output line name"));
                        }
                        let output_line_name = std::str::from_utf8(output_line.value)?;
                        let resource_line = design.get_line(resource_type, output_line_name)?;
                        design.output_lines.insert(resource_line.name);
                        if design.normalized_var == None {
                            design.normalized_var = Some(resource_line.index);
                        }
                    }
                }
            } else {
                // Read a building description
                let building_name = std::str::from_utf8(datum.value)?;
                let proto_building = ProtoBuilding::from_name(building_name)
                    .ok_or(InputError::new("Unknown building"))?;
                if datum.children.is_empty() {
                    return Err(InputError::new("Found building with no recipe"));
                }
                let recipe_name = std::str::from_utf8(datum.children[0].value)?;
                let proto_recipe = ProtoRecipe::from_name(recipe_name)
                    .ok_or(InputError::new("Unknown recipe"))?;
                let mut required_inputs : HashMap<Resource, f32> = proto_recipe.inputs.iter().cloned().collect();
                let mut required_outputs : HashMap<Resource, f32> = proto_recipe.outputs.iter().cloned().collect();
                let mut line_inputs = Vec::new();
                let mut line_outputs = Vec::new();
                for property_datum in datum.children[1..].iter() {
                    if property_datum.value == b"Inputs" {
                        for input_datum in property_datum.children.iter() {
                            let resource_type_str = std::str::from_utf8(input_datum.value)?;
                            let resource_type = Resource::from_str(resource_type_str)?;

                            if input_datum.children.len() != 1 {
                                return Err(InputError::new("Only single line per input is supported."));
                            }
                            let line_name = std::str::from_utf8(input_datum.children[0].value)?;

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
                    } else if property_datum.value == b"Outputs" {
                        for output_datum in property_datum.children.iter() {
                            let resource_type_str = std::str::from_utf8(output_datum.value)?;
                            let resource_type = Resource::from_str(resource_type_str)?;

                            if output_datum.children.len() != 1 {
                                return Err(InputError::new("Only single line per output is supported."));
                            }
                            let line_name = std::str::from_utf8(output_datum.children[0].value)?;

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
            let building_count = analysis[building.index];
            println!("    Count: {}", building_count);
            let building_energy = building.energy_consumption * building_count;
            println!("    Energy cost: {} kW", building_energy);
            total_energy += building_energy;

            println!("    Inputs:");
            for &(ref input_line, qty) in building.recipe.inputs.iter() {
                let input_rate = qty * building_count * building.crafting_speed / building.recipe.time;
                println!("        {}: {} per sec", input_line.name, input_rate);
            }

            println!("    Outputs:");
            for &(ref output_line, qty) in building.recipe.outputs.iter() {
                let output_rate = qty * building_count * building.crafting_speed / building.recipe.time;
                println!("        {}: {} per sec", output_line.name, output_rate);
            }
        }
        println!("");
        println!("Total energy cost: {} kW", total_energy);
    }
}

#[derive(Debug, Clone)]
struct Data<'a> {
    value: &'a [u8],
    children: Vec<Data<'a>>,
}

impl<'a> Data<'a> {
    fn leaf(value: &'a [u8]) -> Data<'a> {
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
        value: is_not!(":\n") >>
        tag!(":") >>
        is_a!(" \t") >>
        children: separated_list!(
                do_parse!(
                    tag!(",") >>
                    opt!(is_a!(" \t")) >>
                    (())
                ),
                map!(is_not!(",\n"), Data::leaf)
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
        value: is_not!(":\n") >>
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

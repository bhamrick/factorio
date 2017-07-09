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
    // Base Resources
    RawWood,
    Coal,
    IronOre,
    CopperOre,
    UraniumOre,
    Stone,
    RawFish,
    Water,
    CrudeOil,

    // Intermediate Products
    Wood,
    IronPlate,
    CopperPlate,
    SteelPlate,
    StoneBrick,
    Sulfur,
    PlasticBar,
    Battery,
    IronStick,
    IronGearWheel,
    CopperCable,
    ElectronicCircuit,
    AdvancedCircuit,
    ProcessingUnit,
    EngineUnit,
    ElectricEngineUnit,
    FlyingRobotFrame,
    SciencePack1,
    SciencePack2,
    SciencePack3,
    MilitarySciencePack,
    ProductionSciencePack,
    HighTechSciencePack,
    SpaceSciencePack,
    EmptyBarrel,
    Explosives,

    // Chemicals
    Petroleum,
    LightOil,
    HeavyOil,
    SulfuricAcid,
    Lubricant,
    Steam,

    // Fluid Barrels
    CrudeOilBarrel,
    HeavyOilBarrel,
    LightOilBarrel,
    LubricantBarrel,
    PetroleumBarrel,
    SulfuricAcidBarrel,
    WaterBarrel,

    // Player Equipment
    IronAxe,
    SteelAxe,

    // Weapons
    Pistol,
    SubmachineGun,
    Shotgun,
    CombatShotgun,
    RocketLauncher,
    Flamethrower,
    LandMine,
    Grenade,
    ClusterGrenade,
    DefenderCapsule,
    PoisonCapsule,
    SlowdownCapsule,
    DistractorCapsule,
    DestroyerCapsule,
    DischargeDefenseRemote,
    Car,
    Tank,
 
    // Ammo
    FirearmMagazine,
    PiercingRoundsMagazine,
    ShotgunShells,
    PiercingShotgunShells,
    Rocket,
    ExplosiveRocket,
    FlamethrowerAmmo,
    CannonShell,
    ExplosiveCannonShell,

    // Armor
    LightArmor,
    HeavyArmor,
    ModularArmor,
    PowerArmor,
    PowerArmor2,

    // Armor components
    NightVision,
    BatteryMk1,
    BatteryMk2,
    EnergyShield,
    EnergyShield2,
    PortableSolarPanel,
    PortableFusionReactor,
    PersonalLaserDefense,
    DischargeDefense,
    Exoskeleton,
    PersonalRoboport,

    // Special
    LogisticRobot,
    ConstructionRobot,
    Roboport,
    SolidFuel,

    // Placeable items
    TransportBelt,
    UndergroundBelt,
    Splitter,
    FastTransportBelt,
    FastUndergroundBelt,
    FastSplitter,
    ExpressTransportBelt,
    ExpressUndergroundBelt,
    ExpressSplitter,

    // Inserters
    BurnerInserter,
    Inserter,
    LongHandedInserter,
    FastInserter,
    FilterInserter,
    StackInserter,
    StackFilterInserter,

    // Storage
    WoodenChest,
    IronChest,
    SteelChest,
    ActiveProviderChest,
    PassiveProviderChest,
    StorageChest,
    RequesterChest,

    // Defensive Structures
    Wall,
    Gate,
    GunTurret,
    LaserTurret,
    FlamethrowerTurret,

    // Machines
    BurnerMiningDrill,
    ElectricMiningDrill,
    StoneFurnace,
    SteelFurnace,
    ElectricFurnace,
    AssemblingMachine1,
    AssemblingMachine2,
    AssemblingMachine3,
    Lab,
    Beacon,
    Radar,

    // Modules
    EfficiencyModule1,
    EfficiencyModule2,
    EfficiencyModule3,
    ProductivityModule1,
    ProductivityModule2,
    ProductivityModule3,
    SpeedModule1,
    SpeedModule2,
    SpeedModule3,

    // Electric Network
    SmallElectricPole,
    MediumElectricPole,
    BigElectricPole,
    Substation,
    Boiler,
    SteamEngine,
    SolarPanel,
    Accumulator,

    // Railway Network
    StraightRail,
    TrainStop,
    RailSignal,
    RailChainSignal,
    DieselLocomotive,
    CargoWagon,
    FluidWagon,

    // Liquid Network
    Pipe,
    PipeToGround,
    OffshorePump,
    StorageTank,
    OilRefinery,
    ChemicalPlant,
    Pumpjack,
    SmallPump,

    // Circuit Network
    Lamp,
    RedWire,
    GreenWire,
    ArithmeticCombinator,
    DeciderCombinator,
    ConstantCombinator,
    PowerSwitch,
    ProgrammableSpeaker,

    // Rocket Components
    RocketSilo,
    LowDensityStructure,
    RocketControlUnit,
    RocketFuel,
    RocketPart,
    Satellite,
}

lazy_static! {
    static ref RESOURCE_NAMES : Vec<(Resource, &'static str)> = vec![
        // Base Resources
        (Resource::RawWood, "Raw Wood"),
        (Resource::Coal, "Coal"),
        (Resource::IronOre, "Iron Ore"),
        (Resource::CopperOre, "Copper Ore"),
        (Resource::UraniumOre, "Uranium Ore"),
        (Resource::Stone, "Stone"),
        (Resource::RawFish, "Raw Fish"),
        (Resource::Water, "Water"),
        (Resource::CrudeOil, "Crude Oil"),

        // Intermediate Products
        (Resource::Wood, "Wood"),
        (Resource::IronPlate, "Iron Plate"),
        (Resource::IronPlate, "Iron"),
        (Resource::CopperPlate, "Copper Plate"),
        (Resource::CopperPlate, "Copper"),
        (Resource::SteelPlate, "Steel Plate"),
        (Resource::SteelPlate, "Steel"),
        (Resource::StoneBrick, "Stone Brick"),
        (Resource::StoneBrick, "Brick"),
        (Resource::Sulfur, "Sulfur"),
        (Resource::PlasticBar, "Plastic Bar"),
        (Resource::PlasticBar, "Plastic"),
        (Resource::Battery, "Battery"),
        (Resource::IronStick, "Iron Stick"),
        (Resource::IronGearWheel, "Iron Gear Wheel"),
        (Resource::IronGearWheel, "Iron Gear"),
        (Resource::IronGearWheel, "Gear"),
        (Resource::IronGearWheel, "Gear Wheel"),
        (Resource::CopperCable, "Copper Cable"),
        (Resource::CopperCable, "Copper Wire"),
        (Resource::ElectronicCircuit, "Electronic Circuit"),
        (Resource::ElectronicCircuit, "Green Circuit"),
        (Resource::AdvancedCircuit, "Advanced Circuit"),
        (Resource::AdvancedCircuit, "Red Circuit"),
        (Resource::ProcessingUnit, "Processing Unit"),
        (Resource::ProcessingUnit, "Blue Unit"),
        (Resource::EngineUnit, "Engine Unit"),
        (Resource::EngineUnit, "Engine"),
        (Resource::ElectricEngineUnit, "Electric Engine Unit"),
        (Resource::ElectricEngineUnit, "Electric Engine"),
        (Resource::FlyingRobotFrame, "Flying Robot Frame"),
        (Resource::FlyingRobotFrame, "Robot Frame"),
        (Resource::SciencePack1, "Science Pack 1"),
        (Resource::SciencePack1, "Red Science"),
        (Resource::SciencePack2, "Science Pack 2"),
        (Resource::SciencePack2, "Green Science"),
        (Resource::SciencePack3, "Science Pack 3"),
        (Resource::SciencePack3, "Blue Science"),
        (Resource::MilitarySciencePack, "Military Science Pack"),
        (Resource::MilitarySciencePack, "Military Science"),
        (Resource::MilitarySciencePack, "Grey Science"),
        (Resource::MilitarySciencePack, "Gray Science"),
        (Resource::ProductionSciencePack, "Production Science Pack"),
        (Resource::ProductionSciencePack, "Production Science"),
        (Resource::ProductionSciencePack, "Purple Science"),
        (Resource::HighTechSciencePack, "High Tech Science Pack"),
        (Resource::HighTechSciencePack, "High Tech Science"),
        (Resource::HighTechSciencePack, "Yellow Science"),
        (Resource::SpaceSciencePack, "Space Science Pack"),
        (Resource::SpaceSciencePack, "Space Science"),
        (Resource::SpaceSciencePack, "White Science"),
        (Resource::EmptyBarrel, "Empty Barrel"),
        (Resource::Explosives, "Explosives"),

        // Chemicals
        (Resource::Petroleum, "Petroleum"),
        (Resource::Petroleum, "Petroleum Gas"),
        (Resource::LightOil, "Light Oil"),
        (Resource::HeavyOil, "Heavy Oil"),
        (Resource::SulfuricAcid, "Sulfuric Acid"),
        (Resource::Lubricant, "Lubricant"),
        (Resource::Steam, "Steam"),

        // Fluid Barrels
        (Resource::CrudeOilBarrel, "Crude Oil Barrel"),
        (Resource::HeavyOilBarrel, "Heavy Oil Barrel"),
        (Resource::LightOilBarrel, "Light Oil Barrel"),
        (Resource::LubricantBarrel, "Lubricant Barrel"),
        (Resource::PetroleumBarrel, "Petroleum Barrel"),
        (Resource::PetroleumBarrel, "Petroleum Gas Barrel"),
        (Resource::SulfuricAcidBarrel, "Sulfuric Acid Barrel"),
        (Resource::WaterBarrel, "Water Barrel"),

        // Player Equipment
        (Resource::IronAxe, "Iron Axe"),
        (Resource::SteelAxe, "Steel Axe"),

        // Weapons
        (Resource::Pistol, "Pistol"),
        (Resource::SubmachineGun, "Submachine Gun"),
        (Resource::Shotgun, "Shotgun"),
        (Resource::CombatShotgun, "Combat Shotgun"),
        (Resource::RocketLauncher, "Rocket Launcher"),
        (Resource::Flamethrower, "Flamethrower"),
        (Resource::LandMine, "Land Mine"),
        (Resource::Grenade, "Basic Grenade"),
        (Resource::Grenade, "Grenade"),
        (Resource::ClusterGrenade, "Cluster Grenade"),
        (Resource::DefenderCapsule, "Defender Capsule"),
        (Resource::PoisonCapsule, "Poison Capsule"),
        (Resource::SlowdownCapsule, "Slowdown Capsule"),
        (Resource::DistractorCapsule, "Distractor Capsule"),
        (Resource::DestroyerCapsule, "Destroyer Capsule"),
        (Resource::DischargeDefenseRemote, "Discharge Defense Remote"),
        (Resource::Car, "Car"),
        (Resource::Tank, "Tank"),

        // Ammo
        (Resource::FirearmMagazine, "Firearm Magazine"),
        (Resource::FirearmMagazine, "Regular Magazine"),
        (Resource::FirearmMagazine, "Magazine"),
        (Resource::FirearmMagazine, "Ammo"),
        (Resource::PiercingRoundsMagazine, "Piercing Rounds Magazine"),
        (Resource::PiercingRoundsMagazine, "Piercing Magazine"),
        (Resource::PiercingRoundsMagazine, "Piercing Ammo"),
        (Resource::ShotgunShells, "Shotgun Shells"),
        (Resource::PiercingShotgunShells, "Piercing Shotgun Shells"),
        (Resource::Rocket, "Rocket"),
        (Resource::ExplosiveRocket, "Explosive Rocket"),
        (Resource::FlamethrowerAmmo, "Flamethrower Ammo"),
        (Resource::CannonShell, "Cannon Shell"),
        (Resource::ExplosiveCannonShell, "Explosive Cannon Shell"),

        // Armor
        (Resource::LightArmor, "Light Armor"),
        (Resource::HeavyArmor, "Heavy Armor"),
        (Resource::ModularArmor, "Modular Armor"),
        (Resource::ModularArmor, "Basic Modular Armor"),
        (Resource::PowerArmor, "Power Armor"),
        (Resource::PowerArmor2, "Power Armor Mk 2"),
        (Resource::PowerArmor2, "Power Armor Mk2"),
        (Resource::PowerArmor2, "Power Armor 2"),

        // Armor Components
        (Resource::NightVision, "Night Vision"),
        (Resource::BatteryMk1, "Battery Mk1"),
        (Resource::BatteryMk2, "Battery Mk2"),
        (Resource::EnergyShield, "Energy Shield"),
        (Resource::EnergyShield2, "Energy Shield Mk2"),
        (Resource::EnergyShield2, "Energy Shield 2"),
        (Resource::PortableSolarPanel, "Portable Solar Panel"),
        (Resource::PortableFusionReactor, "Portable Fusion Reactor"),
        (Resource::PersonalLaserDefense, "Personal Laser Defense"),
        (Resource::DischargeDefense, "Discharge Defense"),
        (Resource::Exoskeleton, "Exoskeleton"),
        (Resource::Exoskeleton, "Basic Exoskeleton Equipment"),
        (Resource::PersonalRoboport, "Personal Roboport"),

        // Special
        (Resource::LogisticRobot, "Logistic Robot"),
        (Resource::ConstructionRobot, "Construction Robot"),
        (Resource::Roboport, "Roboport"),
        (Resource::SolidFuel, "Solid Fuel"),

        // Placeable items
        (Resource::TransportBelt, "Transport Belt"),
        (Resource::TransportBelt, "Yellow Belt"),
        (Resource::UndergroundBelt, "Underground Belt"),
        (Resource::UndergroundBelt, "Underground"),
        (Resource::UndergroundBelt, "Yellow Underground Belt"),
        (Resource::UndergroundBelt, "Yellow Underground"),
        (Resource::Splitter, "Splitter"),
        (Resource::Splitter, "Yellow Splitter"),
        (Resource::FastTransportBelt, "Fast Transport Belt"),
        (Resource::FastTransportBelt, "Red Belt"),
        (Resource::FastUndergroundBelt, "Fast Underground Belt"),
        (Resource::FastUndergroundBelt, "Fast Underground"),
        (Resource::FastUndergroundBelt, "Red Underground Belt"),
        (Resource::FastUndergroundBelt, "Red Underground"),
        (Resource::FastSplitter, "Fast Splitter"),
        (Resource::FastSplitter, "Red Splitter"),
        (Resource::ExpressTransportBelt, "Express Transport Belt"),
        (Resource::ExpressTransportBelt, "Blue Belt"),
        (Resource::ExpressUndergroundBelt, "Express Underground Belt"),
        (Resource::ExpressUndergroundBelt, "Express Underground"),
        (Resource::ExpressUndergroundBelt, "Blue Underground Belt"),
        (Resource::ExpressUndergroundBelt, "Blue Underground"),
        (Resource::ExpressSplitter, "Express Splitter"),
        (Resource::ExpressSplitter, "Blue Splitter"),

        // Inserters
        (Resource::BurnerInserter, "Burner Inserter"),
        (Resource::Inserter, "Inserter"),
        (Resource::LongHandedInserter, "Long Handed Inserter"),
        (Resource::LongHandedInserter, "Long Inserter"),
        (Resource::FastInserter, "Fast Inserter"),
        (Resource::FilterInserter, "Filter Inserter"),
        (Resource::StackInserter, "Stack Inserter"),
        (Resource::StackFilterInserter, "Stack Filter Inserter"),

        // Storage
        (Resource::WoodenChest, "Wooden Chest"),
        (Resource::WoodenChest, "Wood Chest"),
        (Resource::IronChest, "Iron Chest"),
        (Resource::SteelChest, "Steel Chest"),
        (Resource::ActiveProviderChest, "Active Provider Chest"),
        (Resource::PassiveProviderChest, "Passive Provider Chest"),
        (Resource::StorageChest, "Storage Chest"),
        (Resource::RequesterChest, "Requester Chest"),

        // Defensive Structures
        (Resource::Wall, "Wall"),
        (Resource::Gate, "Gate"),
        (Resource::GunTurret, "Gun Turret"),
        (Resource::LaserTurret, "Laser Turret"),
        (Resource::FlamethrowerTurret, "Flamethrower Turret"),

        // Machines
        (Resource::BurnerMiningDrill, "Burner Mining Drill"),
        (Resource::ElectricMiningDrill, "Electric Mining Drill"),
        (Resource::StoneFurnace, "Stone Furnace"),
        (Resource::SteelFurnace, "Steel Furnace"),
        (Resource::ElectricFurnace, "Electric Furnace"),
        (Resource::AssemblingMachine1, "Assembling Machine 1"),
        (Resource::AssemblingMachine2, "Assembling Machine 2"),
        (Resource::AssemblingMachine3, "Assembling Machine 3"),
        (Resource::Lab, "Lab"),
        (Resource::Beacon, "Beacon"),
        (Resource::Radar, "Radar"),

        // Modules
        (Resource::EfficiencyModule1, "Efficiency Module 1"),
        (Resource::EfficiencyModule2, "Efficiency Module 2"),
        (Resource::EfficiencyModule3, "Efficiency Module 3"),
        (Resource::ProductivityModule1, "Productivity Module 1"),
        (Resource::ProductivityModule2, "Productivity Module 2"),
        (Resource::ProductivityModule3, "Productivity Module 3"),
        (Resource::SpeedModule1, "Speed Module 1"),
        (Resource::SpeedModule2, "Speed Module 2"),
        (Resource::SpeedModule3, "Speed Module 3"),

        // Electric Network
        (Resource::SmallElectricPole, "Small Electric Pole"),
        (Resource::SmallElectricPole, "Small Pole"),
        (Resource::MediumElectricPole, "Medium Electric Pole"),
        (Resource::MediumElectricPole, "Medium Pole"),
        (Resource::BigElectricPole, "Big Electric Pole"),
        (Resource::BigElectricPole, "Big Pole"),
        (Resource::Substation, "Substation"),
        (Resource::Boiler, "Boiler"),
        (Resource::SteamEngine, "Steam Engine"),
        (Resource::SolarPanel, "Solar Panel"),
        (Resource::Accumulator, "Accumulator"),

        // Railway Network
        (Resource::StraightRail, "Straight Rail"),
        (Resource::StraightRail, "Rail"),
        (Resource::TrainStop, "Train Stop"),
        (Resource::RailSignal, "Rail Signal"),
        (Resource::RailChainSignal, "Rail Chain Signal"),
        (Resource::DieselLocomotive, "Diesel Locomotive"),
        (Resource::DieselLocomotive, "Locomotive"),
        (Resource::CargoWagon, "Cargo Wagon"),
        (Resource::FluidWagon, "Fluid Wagon"),

        // Liquid Network
        (Resource::Pipe, "Pipe"),
        (Resource::PipeToGround, "Pipe To Ground"),
        (Resource::PipeToGround, "Pipe-To-Ground"),
        (Resource::OffshorePump, "Offshore Pump"),
        (Resource::StorageTank, "Storage Tank"),
        (Resource::OilRefinery, "Oil Refinery"),
        (Resource::ChemicalPlant, "Chemical Plant"),
        (Resource::Pumpjack, "Pumpjack"),
        (Resource::SmallPump, "Small Pump"),

        // Circuit Network
        (Resource::Lamp, "Lamp"),
        (Resource::RedWire, "Red Wire"),
        (Resource::GreenWire, "Green Wire"),
        (Resource::ArithmeticCombinator, "Arithmetic Combinator"),
        (Resource::DeciderCombinator, "Decider Combinator"),
        (Resource::ConstantCombinator, "Constant Combinator"),
        (Resource::PowerSwitch, "Power Switch"),
        (Resource::ProgrammableSpeaker, "Programmable Speaker"),

        // Rocket Components
        (Resource::RocketSilo, "Rocket Silo"),
        (Resource::LowDensityStructure, "Low Density Structure"),
        (Resource::RocketControlUnit, "Rocket Control Unit"),
        (Resource::RocketFuel, "Rocket Fuel"),
        (Resource::RocketPart, "Rocket Part"),
        (Resource::Satellite, "Satellite"),
    ];

    static ref PROTO_BUILDINGS : Vec<ProtoBuilding<'static>> = vec![
        ProtoBuilding {
            name: "Assembling Machine 1",
            energy_consumption: 90.0,
            crafting_speed: 0.5,
        },
        ProtoBuilding {
            name: "Assembling Machine 2",
            energy_consumption: 150.0,
            crafting_speed: 0.75,
        },
        ProtoBuilding {
            name: "Assembling Machine 3",
            energy_consumption: 210.0,
            crafting_speed: 1.25,
        },
        ProtoBuilding {
            name: "Boiler",
            energy_consumption: 0.0,
            crafting_speed: 1.0,
        },
        ProtoBuilding {
            name: "Chemical Plant",
            energy_consumption: 210.0,
            crafting_speed: 1.25,
        },
        ProtoBuilding {
            name: "Oil Refinery",
            energy_consumption: 420.0,
            crafting_speed: 1.0,
        },
    ];

    static ref PROTO_RECIPES : Vec<ProtoRecipe<'static>> = vec![
        ProtoRecipe {
            name: "Boiling (Solid Fuel)",
            aliases: vec![],
            inputs: vec![
                (Resource::Water, 60.0),
                (Resource::SolidFuel, 0.144),
            ],
            outputs: vec![
                (Resource::Steam, 60.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Coal liquefaction",
            aliases: vec![],
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
            aliases: vec![],
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
            name: "Solid Fuel (Heavy Oil)",
            aliases: vec![
                "Solid Fuel (Heavy)",
            ],
            inputs: vec![
                (Resource::HeavyOil, 20.0),
            ],
            outputs: vec![
                (Resource::SolidFuel, 1.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Solid Fuel (Light Oil)",
            aliases: vec![
                "Solid Fuel (Light)",
            ],
            inputs: vec![
                (Resource::LightOil, 10.0),
            ],
            outputs: vec![
                (Resource::SolidFuel, 1.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Solid Fuel (Petroleum)",
            aliases: vec![
                "Solid Fuel (Petroleum Gas)",
            ],
            inputs: vec![
                (Resource::Petroleum, 20.0),
            ],
            outputs: vec![
                (Resource::SolidFuel, 1.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Wood",
            aliases: vec![],
            inputs: vec![
                (Resource::RawWood, 1.0),
            ],
            outputs: vec![
                (Resource::Wood, 2.0),
            ],
            time: 0.5,
        },
        // TODO: Smelting recipes
        ProtoRecipe {
            name: "Sulfur",
            aliases: vec![],
            inputs: vec![
                (Resource::Petroleum, 30.0),
                (Resource::Water, 30.0),
            ],
            outputs: vec![
                (Resource::Sulfur, 2.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Plastic bar",
            aliases: vec![
                "Plastic",
            ],
            inputs: vec![
                (Resource::Coal, 1.0),
                (Resource::Petroleum, 20.0),
            ],
            outputs: vec![
                (Resource::PlasticBar, 2.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Battery",
            aliases: vec![],
            inputs: vec![
                (Resource::CopperPlate, 1.0),
                (Resource::IronPlate, 1.0),
                (Resource::SulfuricAcid, 20.0),
            ],
            outputs: vec![
                (Resource::Battery, 1.0),
            ],
            time: 5.0,
        },
        ProtoRecipe {
            name: "Iron Stick",
            aliases: vec![],
            inputs: vec![
                (Resource::IronPlate, 1.0),
            ],
            outputs: vec![
                (Resource::IronStick, 2.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Iron Gear Wheel",
            aliases: vec![
                "Iron Gear",
            ],
            inputs: vec![
                (Resource::IronPlate, 2.0),
            ],
            outputs: vec![
                (Resource::IronGearWheel, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Copper Cable",
            aliases: vec![
                "Copper Wire",
            ],
            inputs: vec![
                (Resource::CopperPlate, 1.0),
            ],
            outputs: vec![
                (Resource::CopperCable, 2.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Electronic Circuit",
            aliases: vec![
                "Green Circuit",
            ],
            inputs: vec![
                (Resource::CopperCable, 3.0),
                (Resource::IronPlate, 1.0),
            ],
            outputs: vec![
                (Resource::ElectronicCircuit, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Advanced Circuit",
            aliases: vec![
                "Red Circuit",
            ],
            inputs: vec![
                (Resource::CopperCable, 4.0),
                (Resource::ElectronicCircuit, 2.0),
                (Resource::PlasticBar, 2.0),
            ],
            outputs: vec![
                (Resource::AdvancedCircuit, 1.0),
            ],
            time: 6.0,
        },
        ProtoRecipe {
            name: "Processing Unit",
            aliases: vec![
                "Blue Circuit",
            ],
            inputs: vec![
                (Resource::AdvancedCircuit, 2.0),
                (Resource::ElectronicCircuit, 20.0),
                (Resource::SulfuricAcid, 5.0),
            ],
            outputs: vec![
                (Resource::ProcessingUnit, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Engine Unit",
            aliases: vec![
                "Engine",
            ],
            inputs: vec![
                (Resource::IronGearWheel, 1.0),
                (Resource::Pipe, 2.0),
                (Resource::SteelPlate, 1.0),
            ],
            outputs: vec![
                (Resource::EngineUnit, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Electric Engine Unit",
            aliases: vec![
                "Electric Engine",
            ],
            inputs: vec![
                (Resource::ElectronicCircuit, 2.0),
                (Resource::EngineUnit, 1.0),
                (Resource::Lubricant, 15.0),
            ],
            outputs: vec![
                (Resource::ElectricEngineUnit, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Flying Robot Frame",
            aliases: vec![
                "Robot Frame",
            ],
            inputs: vec![
                (Resource::Battery, 2.0),
                (Resource::ElectricEngineUnit, 1.0),
                (Resource::ElectronicCircuit, 3.0),
                (Resource::SteelPlate, 1.0),
            ],
            outputs: vec![
                (Resource::FlyingRobotFrame, 1.0),
            ],
            time: 20.0,
        },
        ProtoRecipe {
            name: "Science Pack 1",
            aliases: vec![
                "Red Science",
            ],
            inputs: vec![
                (Resource::CopperPlate, 1.0),
                (Resource::IronGearWheel, 1.0),
            ],
            outputs: vec![
                (Resource::SciencePack1, 1.0),
            ],
            time: 5.0,
        },
        ProtoRecipe {
            name: "Science Pack 2",
            aliases: vec![
                "Green Science",
            ],
            inputs: vec![
                (Resource::Inserter, 1.0),
                (Resource::TransportBelt, 1.0),
            ],
            outputs: vec![
                (Resource::SciencePack2, 1.0),
            ],
            time: 6.0,
        },
        ProtoRecipe {
            name: "Science Pack 3",
            aliases: vec![
                "Blue Science",
            ],
            inputs: vec![
                (Resource::AdvancedCircuit, 1.0),
                (Resource::ElectricMiningDrill, 1.0),
                (Resource::EngineUnit, 1.0),
            ],
            outputs: vec![
                (Resource::SciencePack3, 1.0),
            ],
            time: 12.0,
        },
        ProtoRecipe {
            name: "Military Science Pack",
            aliases: vec![
                "Military Science",
                "Grey Science",
                "Gray Science",
            ],
            inputs: vec![
                (Resource::Grenade, 1.0),
                (Resource::GunTurret, 1.0),
                (Resource::PiercingRoundsMagazine, 1.0),
            ],
            outputs: vec![
                (Resource::MilitarySciencePack, 2.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Production Science Pack",
            aliases: vec![
                "Production Science",
                "Purple Science",
            ],
            inputs: vec![
                (Resource::AssemblingMachine1, 1.0),
                (Resource::ElectricEngineUnit, 1.0),
                (Resource::ElectricFurnace, 1.0),
            ],
            outputs: vec![
                (Resource::ProductionSciencePack, 2.0),
            ],
            time: 14.0,
        },
        ProtoRecipe {
            name: "High Tech Science Pack",
            aliases: vec![
                "High Tech Science",
                "Yellow Science",
            ],
            inputs: vec![
                (Resource::Battery, 1.0),
                (Resource::CopperCable, 30.0),
                (Resource::ProcessingUnit, 3.0),
                (Resource::SpeedModule1, 1.0),
            ],
            outputs: vec![
                (Resource::HighTechSciencePack, 2.0),
            ],
            time: 14.0,
        },
        ProtoRecipe {
            name: "Rocket Launch",
            aliases: vec![],
            inputs: vec![
                (Resource::RocketPart, 100.0),
                (Resource::Satellite, 1.0),
            ],
            outputs: vec![
                (Resource::SpaceSciencePack, 1000.0),
            ],
            time: 5.0,
        },
        ProtoRecipe {
            name: "Barrel",
            aliases: vec![
                "Empty Barrel",
            ],
            inputs: vec![
                (Resource::SteelPlate, 1.0),
            ],
            outputs: vec![
                (Resource::EmptyBarrel, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Fill Crude Oil Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::CrudeOil, 250.0),
            ],
            outputs: vec![
                (Resource::CrudeOilBarrel, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Fill Heavy Oil Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::HeavyOil, 250.0),
            ],
            outputs: vec![
                (Resource::HeavyOilBarrel, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Fill Light Oil Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::LightOil, 250.0),
            ],
            outputs: vec![
                (Resource::LightOilBarrel, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Fill Lubricant Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::Lubricant, 250.0),
            ],
            outputs: vec![
                (Resource::LubricantBarrel, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Fill Petroleum Gas Barrel",
            aliases: vec![
                "Fill Petroleum Barrel",
            ],
            inputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::Petroleum, 250.0),
            ],
            outputs: vec![
                (Resource::PetroleumBarrel, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Fill Sulfuric Acid Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::SulfuricAcid, 250.0),
            ],
            outputs: vec![
                (Resource::SulfuricAcidBarrel, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Fill Water Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::Water, 250.0),
            ],
            outputs: vec![
                (Resource::WaterBarrel, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Empty Crude Oil Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::CrudeOilBarrel, 1.0),
            ],
            outputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::CrudeOil, 250.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Empty Heavy Oil Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::HeavyOilBarrel, 1.0),
            ],
            outputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::HeavyOil, 250.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Empty Light Oil Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::LightOilBarrel, 1.0),
            ],
            outputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::LightOil, 250.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Empty Lubricant Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::LubricantBarrel, 1.0),
            ],
            outputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::Lubricant, 250.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Empty Petroleum Gas Barrel",
            aliases: vec![
                "Empty Petroleum Barrel",
            ],
            inputs: vec![
                (Resource::PetroleumBarrel, 1.0),
            ],
            outputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::Petroleum, 250.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Empty Sulfuric Acid Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::SulfuricAcidBarrel, 1.0),
            ],
            outputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::SulfuricAcid, 250.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Empty Water Barrel",
            aliases: vec![],
            inputs: vec![
                (Resource::WaterBarrel, 1.0),
            ],
            outputs: vec![
                (Resource::EmptyBarrel, 1.0),
                (Resource::Water, 250.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Explosives",
            aliases: vec![],
            inputs: vec![
                (Resource::Coal, 1.0),
                (Resource::Sulfur, 1.0),
                (Resource::Water, 10.0),
            ],
            outputs: vec![
                (Resource::Explosives, 1.0),
            ],
            time: 5.0,
        },
        ProtoRecipe {
            name: "Iron Axe",
            aliases: vec![],
            inputs: vec![
                (Resource::IronPlate, 3.0),
                (Resource::IronStick, 2.0),
            ],
            outputs: vec![
                (Resource::IronAxe, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Steel Axe",
            aliases: vec![],
            inputs: vec![
                (Resource::IronStick, 2.0),
                (Resource::SteelPlate, 5.0),
            ],
            outputs: vec![
                (Resource::SteelAxe, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Pistol",
            aliases: vec![],
            inputs: vec![
                (Resource::CopperPlate, 5.0),
                (Resource::IronPlate, 5.0),
            ],
            outputs: vec![
                (Resource::Pistol, 1.0),
            ],
            time: 5.0,
        },
        ProtoRecipe {
            name: "Submachine Gun",
            aliases: vec![],
            inputs: vec![
                (Resource::CopperPlate, 5.0),
                (Resource::IronGearWheel, 10.0),
                (Resource::IronPlate, 10.0),
            ],
            outputs: vec![
                (Resource::SubmachineGun, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Shotgun",
            aliases: vec![],
            inputs: vec![
                (Resource::CopperPlate, 10.0),
                (Resource::IronGearWheel, 5.0),
                (Resource::IronPlate, 15.0),
                (Resource::Wood, 5.0),
            ],
            outputs: vec![
                (Resource::Shotgun, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Combat Shotgun",
            aliases: vec![],
            inputs: vec![
                (Resource::CopperPlate, 10.0),
                (Resource::IronGearWheel, 5.0),
                (Resource::SteelPlate, 15.0),
                (Resource::Wood, 10.0),
            ],
            outputs: vec![
                (Resource::CombatShotgun, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Rocket Launcher",
            aliases: vec![],
            inputs: vec![
                (Resource::ElectronicCircuit, 5.0),
                (Resource::IronGearWheel, 5.0),
                (Resource::IronPlate, 5.0),
            ],
            outputs: vec![
                (Resource::RocketLauncher, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Flamethrower",
            aliases: vec![],
            inputs: vec![
                (Resource::IronGearWheel, 10.0),
                (Resource::SteelPlate, 5.0),
            ],
            outputs: vec![
                (Resource::Flamethrower, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Land Mine",
            aliases: vec![],
            inputs: vec![
                (Resource::Explosives, 2.0),
                (Resource::SteelPlate, 1.0),
            ],
            outputs: vec![
                (Resource::LandMine, 4.0),
            ],
            time: 5.0,
        },
        ProtoRecipe {
            name: "Grenade",
            aliases: vec![
                "Basic Grenade",
            ],
            inputs: vec![
                (Resource::Coal, 10.0),
                (Resource::IronPlate, 5.0),
            ],
            outputs: vec![
                (Resource::Grenade, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Cluster Grenade",
            aliases: vec![],
            inputs: vec![
                (Resource::Explosives, 5.0),
                (Resource::Grenade, 7.0),
                (Resource::SteelPlate, 5.0),
            ],
            outputs: vec![
                (Resource::ClusterGrenade, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Defender Capsule",
            aliases: vec![],
            inputs: vec![
                (Resource::ElectronicCircuit, 2.0),
                (Resource::IronGearWheel, 3.0),
                (Resource::PiercingRoundsMagazine, 1.0),
            ],
            outputs: vec![
                (Resource::DefenderCapsule, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Poison Capsule",
            aliases: vec![],
            inputs: vec![
                (Resource::Coal, 10.0),
                (Resource::ElectronicCircuit, 3.0),
                (Resource::SteelPlate, 3.0),
            ],
            outputs: vec![
                (Resource::PoisonCapsule, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Slowdown Capsule",
            aliases: vec![],
            inputs: vec![
                (Resource::Coal, 5.0),
                (Resource::ElectronicCircuit, 2.0),
                (Resource::SteelPlate, 2.0),
            ],
            outputs: vec![
                (Resource::SlowdownCapsule, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Distractor Capsule",
            aliases: vec![],
            inputs: vec![
                (Resource::AdvancedCircuit, 3.0),
                (Resource::DefenderCapsule, 4.0),
            ],
            outputs: vec![
                (Resource::DistractorCapsule, 1.0),
            ],
            time: 15.0,
        },
        ProtoRecipe {
            name: "Destroyer Capsule",
            aliases: vec![],
            inputs: vec![
                (Resource::DistractorCapsule, 4.0),
                (Resource::SpeedModule1, 1.0),
            ],
            outputs: vec![
                (Resource::DestroyerCapsule, 1.0),
            ],
            time: 15.0,
        },
        ProtoRecipe {
            name: "Discharge Defense Remote",
            aliases: vec![],
            inputs: vec![
                (Resource::ElectronicCircuit, 1.0),
            ],
            outputs: vec![
                (Resource::DischargeDefenseRemote, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Car",
            aliases: vec![],
            inputs: vec![
                (Resource::EngineUnit, 8.0),
                (Resource::IronPlate, 20.0),
                (Resource::SteelPlate, 5.0),
            ],
            outputs: vec![
                (Resource::Car, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Tank",
            aliases: vec![],
            inputs: vec![
                (Resource::AdvancedCircuit, 10.0),
                (Resource::EngineUnit, 32.0),
                (Resource::IronGearWheel, 15.0),
                (Resource::SteelPlate, 50.0),
            ],
            outputs: vec![
                (Resource::Tank, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Firearm Magazine",
            aliases: vec![
                "Regular Magazine",
                "Ammo",
            ],
            inputs: vec![
                (Resource::IronPlate, 4.0),
            ],
            outputs: vec![
                (Resource::FirearmMagazine, 1.0),
            ],
            time: 1.0,
        },
        ProtoRecipe {
            name: "Piercing Rounds Magazine",
            aliases: vec![
                "Piercing Magazine",
                "Piercing Ammo",
                "Piercing Rounds",
            ],
            inputs: vec![
                (Resource::CopperPlate, 5.0),
                (Resource::FirearmMagazine, 1.0),
                (Resource::SteelPlate, 1.0),
            ],
            outputs: vec![
                (Resource::PiercingRoundsMagazine, 1.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Shotgun Shells",
            aliases: vec![],
            inputs: vec![
                (Resource::CopperPlate, 2.0),
                (Resource::IronPlate, 2.0),
            ],
            outputs: vec![
                (Resource::ShotgunShells, 1.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Piercing Shotgun Shells",
            aliases: vec![],
            inputs: vec![
                (Resource::CopperPlate, 5.0),
                (Resource::ShotgunShells, 2.0),
                (Resource::SteelPlate, 2.0),
            ],
            outputs: vec![
                (Resource::PiercingShotgunShells, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Rocket",
            aliases: vec![],
            inputs: vec![
                (Resource::ElectronicCircuit, 1.0),
                (Resource::Explosives, 1.0),
                (Resource::IronPlate, 2.0),
            ],
            outputs: vec![
                (Resource::Rocket, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Explosive Rocket",
            aliases: vec![],
            inputs: vec![
                (Resource::Explosives, 2.0),
                (Resource::Rocket, 1.0),
            ],
            outputs: vec![
                (Resource::ExplosiveRocket, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Flamethrower Ammo",
            aliases: vec![],
            inputs: vec![
                (Resource::HeavyOil, 50.0),
                (Resource::LightOil, 50.0),
                (Resource::SteelPlate, 5.0),
            ],
            outputs: vec![
                (Resource::FlamethrowerAmmo, 1.0),
            ],
            time: 6.0,
        },
        ProtoRecipe {
            name: "Cannon Shell",
            aliases: vec![],
            inputs: vec![
                (Resource::Explosives, 1.0),
                (Resource::PlasticBar, 2.0),
                (Resource::SteelPlate, 2.0),
            ],
            outputs: vec![
                (Resource::CannonShell, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Explosive Cannon Shell",
            aliases: vec![],
            inputs: vec![
                (Resource::Explosives, 2.0),
                (Resource::PlasticBar, 2.0),
                (Resource::SteelPlate, 2.0),
            ],
            outputs: vec![
                (Resource::ExplosiveCannonShell, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Light Armor",
            aliases: vec![
                "Iron Armor",
            ],
            inputs: vec![
                (Resource::IronPlate, 40.0),
            ],
            outputs: vec![
                (Resource::LightArmor, 1.0),
            ],
            time: 3.0,
        },
        ProtoRecipe {
            name: "Heavy Armor",
            aliases: vec![],
            inputs: vec![
                (Resource::CopperPlate, 100.0),
                (Resource::SteelPlate, 50.0),
            ],
            outputs: vec![
                (Resource::HeavyArmor, 1.0),
            ],
            time: 8.0,
        },
        ProtoRecipe {
            name: "Modular Armor",
            aliases: vec![
                "Basic Mdoular Armor",
            ],
            inputs: vec![
                (Resource::AdvancedCircuit, 30.0),
                (Resource::SteelPlate, 50.0),
            ],
            outputs: vec![
                (Resource::ModularArmor, 1.0),
            ],
            time: 15.0,
        },
        ProtoRecipe {
            name: "Power Armor",
            aliases: vec![],
            inputs: vec![
                (Resource::ElectricEngineUnit, 20.0),
                (Resource::ProcessingUnit, 40.0),
                (Resource::SteelPlate, 40.0),
            ],
            outputs: vec![
                (Resource::PowerArmor, 1.0),
            ],
            time: 20.0,
        },
        ProtoRecipe {
            name: "Power Armor Mk2",
            aliases: vec![
                "Power Armor 2",
                "Power Armor Mk 2",
            ],
            inputs: vec![
                (Resource::EfficiencyModule3, 5.0),
                (Resource::ProcessingUnit, 40.0),
                (Resource::SpeedModule3, 5.0),
                (Resource::SteelPlate, 40.0),
            ],
            outputs: vec![
                (Resource::PowerArmor2, 1.0),
            ],
            time: 25.0,
        },
        ProtoRecipe {
            name: "Night Vision",
            aliases: vec![],
            inputs: vec![
                (Resource::AdvancedCircuit, 5.0),
                (Resource::SteelPlate, 10.0),
            ],
            outputs: vec![
                (Resource::NightVision, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Battery Mk1",
            aliases: vec![
                "Battery Mk 1",
            ],
            inputs: vec![
                (Resource::Battery, 5.0),
                (Resource::SteelPlate, 10.0),
            ],
            outputs: vec![
                (Resource::BatteryMk1, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Battery Mk2",
            aliases: vec![
                "Battery Mk 2",
            ],
            inputs: vec![
                (Resource::BatteryMk1, 10.0),
                (Resource::ProcessingUnit, 20.0),
            ],
            outputs: vec![
                (Resource::BatteryMk2, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Energy Shield",
            aliases: vec![],
            inputs: vec![
                (Resource::AdvancedCircuit, 5.0),
                (Resource::SteelPlate, 10.0),
            ],
            outputs: vec![
                (Resource::EnergyShield, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Energy Shield Mk2",
            aliases: vec![
                "Energy Shield 2",
            ],
            inputs: vec![
                (Resource::EnergyShield, 10.0),
                (Resource::ProcessingUnit, 10.0),
            ],
            outputs: vec![
                (Resource::EnergyShield2, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Portable Solar Panel",
            aliases: vec![],
            inputs: vec![
                (Resource::AdvancedCircuit, 1.0),
                (Resource::SolarPanel, 5.0),
                (Resource::SteelPlate, 5.0),
            ],
            outputs: vec![
                (Resource::PortableSolarPanel, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Portable Fusion Reactor",
            aliases: vec![],
            inputs: vec![
                (Resource::ProcessingUnit, 250.0),
            ],
            outputs: vec![
                (Resource::PortableFusionReactor, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Personal Laser Defense",
            aliases: vec![],
            inputs: vec![
                (Resource::LaserTurret, 5.0),
                (Resource::ProcessingUnit, 1.0),
                (Resource::SteelPlate, 5.0),
            ],
            outputs: vec![
                (Resource::PersonalLaserDefense, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Discharge Defense",
            aliases: vec![],
            inputs: vec![
                (Resource::LaserTurret, 10.0),
                (Resource::ProcessingUnit, 5.0),
                (Resource::SteelPlate, 20.0),
            ],
            outputs: vec![
                (Resource::DischargeDefense, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Exoskeleton",
            aliases: vec![
                "Basic Exoskeleton Equipment",
            ],
            inputs: vec![
                (Resource::ElectricEngineUnit, 30.0),
                (Resource::ProcessingUnit, 10.0),
                (Resource::SteelPlate, 20.0),
            ],
            outputs: vec![
                (Resource::Exoskeleton, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Personal Roboport",
            aliases: vec![],
            inputs: vec![
                (Resource::AdvancedCircuit, 10.0),
                (Resource::Battery, 45.0),
                (Resource::IronGearWheel, 40.0),
                (Resource::SteelPlate, 20.0),
            ],
            outputs: vec![
                (Resource::PersonalRoboport, 1.0),
            ],
            time: 10.0,
        },
        ProtoRecipe {
            name: "Logistic Robot",
            aliases: vec![],
            inputs: vec![
                (Resource::AdvancedCircuit, 2.0),
                (Resource::FlyingRobotFrame, 1.0),
            ],
            outputs: vec![
                (Resource::LogisticRobot, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Construction Robot",
            aliases: vec![],
            inputs: vec![
                (Resource::ElectronicCircuit, 2.0),
                (Resource::FlyingRobotFrame, 1.0),
            ],
            outputs: vec![
                (Resource::ConstructionRobot, 1.0),
            ],
            time: 0.5,
        },
        ProtoRecipe {
            name: "Roboport",
            aliases: vec![],
            inputs: vec![
                (Resource::AdvancedCircuit, 45.0),
                (Resource::IronGearWheel, 45.0),
                (Resource::SteelPlate, 45.0),
            ],
            outputs: vec![
                (Resource::Roboport, 1.0),
            ],
            time: 10.0,
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
    aliases: Vec<&'a str>,
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
            for alias in proto.aliases.iter() {
                if name.to_lowercase() == alias.to_lowercase() {
                    return Some((*proto).clone());
                }
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
                // TODO: Check that recipe is allowed in the building

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

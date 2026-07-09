use rand::prelude::*;
use rand_distr::WeightedAliasIndex;
use ctw_machine::{MachineSpec, MachineClass, WeightClass};
use crate::{FleetConfig, Machine, Capability};

fn get_capabilities(class: MachineClass) -> Vec<Capability> {
    match class {
        MachineClass::Excavator => vec![Capability::Motion, Capability::Pose, Capability::ExcavatorKinematics, Capability::Health],
        MachineClass::WheelLoader => vec![Capability::Motion, Capability::Pose, Capability::Health],
        MachineClass::HaulTruck => vec![Capability::Motion, Capability::Health],
        MachineClass::ArticulatedDumpTruck => vec![Capability::Motion, Capability::Health],
        MachineClass::CrawlerCrane => vec![Capability::Motion, Capability::Pose, Capability::CraneKinematics, Capability::Health],
        MachineClass::TowerCrane => vec![Capability::Motion, Capability::Pose, Capability::CraneKinematics, Capability::Health],
        MachineClass::MobileCrane => vec![Capability::Motion, Capability::Pose, Capability::CraneKinematics, Capability::Health],
        MachineClass::Dozer => vec![Capability::Motion, Capability::Health],
        MachineClass::Grader => vec![Capability::Motion, Capability::Health],
        MachineClass::CompactTrackLoader => vec![Capability::Motion, Capability::Pose, Capability::Health],
        MachineClass::SkidSteerLoader => vec![Capability::Motion, Capability::Pose, Capability::Health],
        MachineClass::Backhoe => vec![Capability::Motion, Capability::Pose, Capability::ExcavatorKinematics, Capability::Health],
        MachineClass::Telehandler => vec![Capability::Motion, Capability::Pose, Capability::Health],
        MachineClass::RollerCompactor => vec![Capability::Motion, Capability::Health],
        MachineClass::Paver => vec![Capability::Motion, Capability::Health],
        MachineClass::DrillingRig => vec![Capability::Motion, Capability::Health],
        MachineClass::PileDrivingRig => vec![Capability::Motion, Capability::Health],
        MachineClass::ConcretePump => vec![Capability::Motion, Capability::Health],
        MachineClass::ConcreteMixer => vec![Capability::Motion, Capability::Health],
        MachineClass::AerialWorkPlatform => vec![Capability::Motion, Capability::Health],
    }
}

fn generate_machine_spec(rng: &mut StdRng, class: MachineClass) -> MachineSpec {
    let weight_class = match rng.gen_range(0..6) {
        0 => WeightClass::Mini,
        1 => WeightClass::Light,
        2 => WeightClass::Medium,
        3 => WeightClass::Heavy,
        4 => WeightClass::ExtraHeavy,
        _ => WeightClass::SuperHeavy,
    };
    let operating_weight_kg = match weight_class {
        WeightClass::Mini => rng.gen_range(1000.0..6000.0),
        WeightClass::Light => rng.gen_range(6000.0..15000.0),
        WeightClass::Medium => rng.gen_range(15000.0..30000.0),
        WeightClass::Heavy => rng.gen_range(30000.0..50000.0),
        WeightClass::ExtraHeavy => rng.gen_range(50000.0..80000.0),
        WeightClass::SuperHeavy => rng.gen_range(80000.0..200000.0),
    };
    let max_rated_capacity_kg = 1000.0; // Simplified
    let year = rng.gen_range(2010..2024);
    let make = "Generic".to_string();
    let model = format!("{:?}", class);
    MachineSpec {
        class,
        weight_class,
        operating_weight_kg,
        max_rated_capacity_kg,
        year,
        make,
        model,
    }
}

pub fn generate_fleet(rng: &mut StdRng, config: &FleetConfig) -> Vec<Machine> {
    let dist = WeightedAliasIndex::new(config.class_weights.clone()).unwrap();
    let mut machines = Vec::with_capacity(config.n_machines);
    for id in 0..config.n_machines {
        let class_idx = dist.sample(rng);
        let class = config.machine_classes[class_idx];
        let spec = generate_machine_spec(rng, class);
        let capabilities = get_capabilities(class);
        machines.push(Machine { id: id as u32, spec, capabilities });
    }
    machines
}
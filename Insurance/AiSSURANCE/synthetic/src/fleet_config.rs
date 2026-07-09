//! Deterministic fleet configuration used by demos and tests.

use contracts::{Degrees, MachineId, Meters, MetersPerSecond, SiteId, WorkerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: Degrees,
    pub longitude: Degrees,
    pub elevation: Meters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
    pub location: Location,
    pub area_sqm: f32,
    pub terrain_difficulty: f32,
    pub typical_workers: u32,
    pub operating_hours: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MachineType {
    Excavator,
    Bulldozer,
    Loader,
    Crane,
    DumpTruck,
    Grader,
    Backhoe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub id: MachineId,
    pub site_id: SiteId,
    pub machine_type: MachineType,
    pub max_speed: MetersPerSecond,
    pub weight_kg: f32,
    pub fuel_capacity_l: f32,
    pub daily_hours: f32,
    pub age_years: f32,
    pub maintenance_status: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerRole {
    Operator,
    Supervisor,
    Laborer,
    SafetyOfficer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub id: WorkerId,
    pub site_id: SiteId,
    pub role: WorkerRole,
    pub experience_years: f32,
    pub safety_cert_level: u8,
    pub shift_hours: f32,
    pub fatigue_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetConfig {
    pub sites: Vec<Site>,
    pub machines: Vec<Machine>,
    pub workers: Vec<Worker>,
    pub global: GlobalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub simulation_days: u32,
    pub time_step_seconds: f32,
    pub random_seed: u64,
    pub base_risk_level: f32,
}

impl FleetConfig {
    pub fn load_from_file(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Self::from_yaml(path)
    }

    pub fn from_yaml(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let contents = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&contents)?;
        config.validate()?;
        Ok(config)
    }

    pub fn to_yaml(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), String> {
        for site in &self.sites {
            if !(0.0..=1.0).contains(&site.terrain_difficulty) {
                return Err(format!(
                    "site {} terrain_difficulty must be in [0,1]",
                    site.id
                ));
            }
        }

        for machine in &self.machines {
            if !self.sites.iter().any(|site| site.id == machine.site_id) {
                return Err(format!(
                    "machine {} references unknown site {}",
                    machine.id, machine.site_id
                ));
            }
            if !(0.0..=1.0).contains(&machine.maintenance_status) {
                return Err(format!(
                    "machine {} maintenance_status must be in [0,1]",
                    machine.id
                ));
            }
        }

        for worker in &self.workers {
            if !self.sites.iter().any(|site| site.id == worker.site_id) {
                return Err(format!(
                    "worker {} references unknown site {}",
                    worker.id, worker.site_id
                ));
            }
            if !(0.0..=1.0).contains(&worker.fatigue_level) {
                return Err(format!(
                    "worker {} fatigue_level must be in [0,1]",
                    worker.id
                ));
            }
        }

        if !(0.0..=1.0).contains(&self.global.base_risk_level) {
            return Err("global base_risk_level must be in [0,1]".to_string());
        }

        Ok(())
    }

    pub fn machines_at_site(&self, site_id: &SiteId) -> Vec<&Machine> {
        self.machines
            .iter()
            .filter(|machine| &machine.site_id == site_id)
            .collect()
    }

    pub fn get_machine(&self, machine_id: &MachineId) -> Option<&Machine> {
        self.machines
            .iter()
            .find(|machine| &machine.id == machine_id)
    }
}

impl Default for FleetConfig {
    fn default() -> Self {
        let site_id = SiteId::test_id(1);
        let machine_a = MachineId::test_id(1);
        let machine_b = MachineId::test_id(2);
        let worker_id = WorkerId::test_id(1);

        Self {
            sites: vec![Site {
                id: site_id,
                name: "Demo Site".to_string(),
                location: Location {
                    latitude: Degrees(37.7749),
                    longitude: Degrees(-122.4194),
                    elevation: Meters(12.0),
                },
                area_sqm: 10_000.0,
                terrain_difficulty: 0.35,
                typical_workers: 8,
                operating_hours: 8.0,
            }],
            machines: vec![
                Machine {
                    id: machine_a,
                    site_id,
                    machine_type: MachineType::Excavator,
                    max_speed: MetersPerSecond(8.0),
                    weight_kg: 22_000.0,
                    fuel_capacity_l: 220.0,
                    daily_hours: 7.0,
                    age_years: 3.0,
                    maintenance_status: 0.85,
                },
                Machine {
                    id: machine_b,
                    site_id,
                    machine_type: MachineType::Loader,
                    max_speed: MetersPerSecond(10.0),
                    weight_kg: 18_000.0,
                    fuel_capacity_l: 180.0,
                    daily_hours: 6.0,
                    age_years: 2.0,
                    maintenance_status: 0.92,
                },
            ],
            workers: vec![Worker {
                id: worker_id,
                site_id,
                role: WorkerRole::Laborer,
                experience_years: 4.0,
                safety_cert_level: 3,
                shift_hours: 8.0,
                fatigue_level: 0.25,
            }],
            global: GlobalConfig {
                simulation_days: 90,
                time_step_seconds: 60.0,
                random_seed: 42,
                base_risk_level: 0.3,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        assert!(FleetConfig::default().validate().is_ok());
    }

    #[test]
    fn rejects_unknown_machine_site() {
        let mut config = FleetConfig::default();
        config.machines[0].site_id = SiteId::test_id(99);
        assert!(config.validate().is_err());
    }
}

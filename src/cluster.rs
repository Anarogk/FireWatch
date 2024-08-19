use serde::{Deserialize, Serialize};
use core::cmp::Ordering;

// Define the structure for a sensor, which includes coordinates and sensor data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sensor {
    pub id: u32,
    pub x: f32,     // X coordinate (e.g., horizontal location)
    pub y: f32,     // Y coordinate (e.g., vertical location)
    pub floor: u32, // Floor number in the building
    pub room: u32,  // Room number
    pub temperature: f32,
    pub smoke_level: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cluster {
    pub sensors: Vec<Sensor>,
    pub floor: u32,
    pub room: u32,
}

impl Cluster {
    pub fn new(floor: u32, room: u32) -> Self {
        Cluster {
            sensors: Vec::new(),
            floor,
            room,
        }
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    // Calculate the average temperature and smoke level in the cluster
    pub fn average_data(&self) -> (f32, f32) {
        let total_sensors = self.sensors.len() as f32;
        let total_temperature: f32 = self.sensors.iter().map(|s| s.temperature).sum();
        let total_smoke: f32 = self.sensors.iter().map(|s| s.smoke_level).sum();

        (
            total_temperature / total_sensors,
            total_smoke / total_sensors,
        )
    }

    // Check if this cluster is affected by fire
    pub fn is_affected(&self, temp_threshold: f32, smoke_threshold: f32) -> bool {
        let (avg_temp, avg_smoke) = self.average_data();
        avg_temp >= temp_threshold && avg_smoke >= smoke_threshold
    }
}

// Function to perform clustering of sensors based on their location (floor and room)
pub fn cluster_sensors(sensors: Vec<Sensor>) -> Vec<Cluster> {
    let mut clusters: Vec<Cluster> = Vec::new();

    for sensor in sensors {
        // Check if there's already a cluster for this floor and room
        let mut cluster_found = false;
        for cluster in clusters.iter_mut() {
            if cluster.floor == sensor.floor && cluster.room == sensor.room {
                cluster.add_sensor(sensor.clone());
                cluster_found = true;
                break;
            }
        }
        // If no cluster exists for this floor and room, create a new one
        if !cluster_found {
            let mut new_cluster = Cluster::new(sensor.floor, sensor.room);
            new_cluster.add_sensor(sensor);
            clusters.push(new_cluster);
        }
    }

    clusters
}

// Sorting clusters to prioritize affected areas based on severity (higher temperature and smoke levels)
pub fn sort_clusters_by_severity(clusters: &mut Vec<Cluster>) {
    clusters.sort_by(|a, b| {
        let (a_temp, a_smoke) = a.average_data();
        let (b_temp, b_smoke) = b.average_data();

        if a_temp > b_temp || (a_temp == b_temp && a_smoke > b_smoke) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_sensors() {
        let sensors = vec![
            Sensor { id: 1, x: 1.0, y: 1.0, floor: 1, room: 101, temperature: 70.0, smoke_level: 0.8 },
            Sensor { id: 2, x: 2.0, y: 2.0, floor: 1, room: 101, temperature: 75.0, smoke_level: 0.9 },
            Sensor { id: 3, x: 3.0, y: 3.0, floor: 1, room: 102, temperature: 65.0, smoke_level: 0.7 },
        ];

        let clusters = cluster_sensors(sensors);

        assert_eq!(clusters.len(), 2); // Should create 2 clusters (one for room 101 and one for room 102)
        assert_eq!(clusters[0].sensors.len(), 2); // Room 101 should have 2 sensors
        assert_eq!(clusters[1].sensors.len(), 1); // Room 102 should have 1 sensor
    }

    #[test]
    fn test_is_affected() {
        let sensors = vec![
            Sensor { id: 1, x: 1.0, y: 1.0, floor: 1, room: 101, temperature: 100.0, smoke_level: 1.2 },
            Sensor { id: 2, x: 2.0, y: 2.0, floor: 1, room: 101, temperature: 95.0, smoke_level: 1.1 },
        ];

        let mut cluster = Cluster::new(1, 101);
        for sensor in sensors {
            cluster.add_sensor(sensor);
        }

        assert!(cluster.is_affected(90.0, 1.0)); // The cluster should be marked as affected
    }
}

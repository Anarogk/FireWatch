pub struct FireDetection {
    temperature_threshold: f32,
    smoke_threshold: u16,
}

impl FireDetection {
    pub fn new(temperature_threshold: f32, smoke_threshold: u16) -> Self {
        Self {
            temperature_threshold,
            smoke_threshold,
        }
    }

    pub fn detect_fire(&self, temperature: f32, smoke_level: u16) -> bool {
        temperature > self.temperature_threshold || smoke_level > self.smoke_threshold
    }

    pub fn identify_affected_cluster(
        &self,
        location: &str,
        cluster: &SensorCluster
    ) -> Option<Vec<&'static str>> {
        cluster.find_cluster(location).cloned()
    }
}

pub struct SensorCluster {
    clusters: std::collections::HashMap<&'static str, Vec<&'static str>>,
}

impl SensorCluster {
    pub fn new() -> Self {
        let mut clusters = std::collections::HashMap::new();
        clusters.insert("Floor 1", vec!["Room A", "Room B"]);
        clusters.insert("Floor 2", vec!["Room C", "Room D"]);
        Self { clusters }
    }

    pub fn find_cluster(&self, location: &str) -> Option<&Vec<&'static str>> {
        self.clusters.get(location)
    }
}

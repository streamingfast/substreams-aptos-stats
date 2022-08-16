pub mod aptos {
    pub mod extractor {
        // @@protoc_insertion_point(attribute:aptos.extractor.v1)
        pub mod v1 {
            include!("aptos.extractor.v1.rs");
            // @@protoc_insertion_point(aptos.extractor.v1)
        }
    }
    pub mod util {
        // @@protoc_insertion_point(attribute:aptos.util.timestamp)
        pub mod timestamp {
            include!("aptos.util.timestamp.rs");
            // @@protoc_insertion_point(aptos.util.timestamp)
        }
    }
}

#[path = "aptos.stats.v1.rs"]
#[allow(dead_code)]
pub mod stats;

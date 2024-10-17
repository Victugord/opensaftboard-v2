use std::collections::HashMap;
use std::sync::OnceLock;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

type TaxonomyHashMap = HashMap<String, Vec<Taxonomy>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taxonomies {
    credits: TaxonomyHashMap,
    debits: TaxonomyHashMap,
    classes: TaxonomyHashMap,
}

static BASE_TAXONOMIES: OnceLock<Taxonomies> = OnceLock::new();
static MICRO_TAXONOMIES: OnceLock<Taxonomies> = OnceLock::new();

static BASE_DATA: &str = include_str!("base.json");
static MICRO_DATA: &str = include_str!("micro.json");

// Optimized JSON loading function
fn load_taxonomies(json_file: &str) -> Taxonomies {
    let start = Instant::now();

    // Use a more efficient match and avoid checking the file type repeatedly.
    let json_str = match json_file {
        "base.json" => BASE_DATA,
        "micro.json" => MICRO_DATA,
        _ => panic!("Invalid JSON file"),
    };

    debug!("{} JSON loading time: {:?}", json_file, start.elapsed());

    // Parse JSON into a vector of Taxonomy structs
    let data: Vec<Taxonomy> = serde_json::from_str(json_str).expect("JSON was not well-formatted");

    let mut credits = HashMap::with_capacity(data.len());
    let mut debits = HashMap::with_capacity(data.len());
    let mut classes = HashMap::with_capacity(data.len());

    // Combine the lowercase conversion and population in a single pass.
    for mut taxonomy in data {
        taxonomy.debit = taxonomy.debit.trim().to_lowercase();
        taxonomy.credit = taxonomy.credit.trim().to_lowercase();
        taxonomy.class = taxonomy.class.trim().to_lowercase();

        credits.entry(taxonomy.credit.clone()).or_insert_with(Vec::new).push(taxonomy.clone());
        debits.entry(taxonomy.debit.clone()).or_insert_with(Vec::new).push(taxonomy.clone());
        classes.entry(taxonomy.class.clone()).or_insert_with(Vec::new).push(taxonomy);
    }

    debug!("{} JSON total time: {:?}", json_file, start.elapsed());


    Taxonomies { credits, debits, classes }
}

// Load base taxonomies once and reuse.
fn get_base_taxonomies() -> &'static Taxonomies {
    BASE_TAXONOMIES.get_or_init(|| load_taxonomies("base.json"))
}

// Load micro taxonomies once and reuse.
fn get_micro_taxonomies() -> &'static Taxonomies {
    MICRO_TAXONOMIES.get_or_init(|| load_taxonomies("micro.json"))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Taxonomy {
    pub taxonomy_code: u32,
    #[serde(rename = "SNCSVAT")]
    pub sncsvat: String,
    #[serde(rename = "Classe")]
    pub class: String,
    pub debit: String,
    pub credit: String,
}

pub enum TaxonomyType {
    Base,
    Micro,
}

impl Taxonomies {
    // Factory method to get taxonomies by type.
    pub fn new(taxonomy_type: TaxonomyType) -> &'static Taxonomies {
        match taxonomy_type {
            TaxonomyType::Base => get_base_taxonomies(),
            TaxonomyType::Micro => get_micro_taxonomies(),
        }
    }

    // Retrieve taxonomies based on a provided string (`dr`).
    pub fn get_by_dr(&self, dr: &str,class:Option<&str>) -> Vec<&Taxonomy> {
        let start = Instant::now(); // Start a timer to measure execution time.

        let mut taxonomies: Vec<&Taxonomy> = self.credits
            .get(&dr.trim().to_lowercase())
            .into_iter()
            .chain(self.debits.get(&dr.trim().to_lowercase()))
            .flatten()
            .collect();

        // Filter by class if provided
        if let Some(class) = class {
            taxonomies.retain(|t| t.class == class.trim().to_lowercase());
        }

        debug!("get_by_dr: {:?}", start.elapsed());

        taxonomies
    }

    pub fn get_credits_by_dr(&self, dr: &str, class:Option<&str>) -> Option<Vec<&Taxonomy>> {

        let start = Instant::now();

        let key = dr.trim().to_lowercase();


        let taxonomies: Vec<&Taxonomy> = self.credits
            .get(&key)
            .into_iter()
            .flat_map(|v| v.iter())
            .filter(|t| class.map_or(true, |c| t.class == c.trim().to_lowercase()))
            .collect();
        debug!("get_credits_by_dr: {:?}", start.elapsed());

        if taxonomies.is_empty() {
            None
        } else {
            Some(taxonomies)
        }
    }

    pub fn get_debits_by_dr(&self, dr: &str, class:Option<&str>) -> Option<Vec<&Taxonomy>> {

        let start = Instant::now();

        let key = dr.trim().to_lowercase();


        let taxonomies: Vec<&Taxonomy> = self.debits
            .get(&key)
            .into_iter()
            .flat_map(|v| v.iter())
            .filter(|t| class.map_or(true, |c| t.class == c.trim().to_lowercase()))
            .collect();

        debug!("get_debits_by_dr: {:?}", start.elapsed());
        if taxonomies.is_empty() {
            None
        } else {
            Some(taxonomies)
        }

    }

    pub fn get_by_class(&self, class: &str) -> Option<Vec<&Taxonomy>> {
        #[cfg(any(debug_assertions, feature = "logging"))]
        let start = Instant::now();

        let taxonomies: Vec<&Taxonomy> = self.classes
            .get(&class.trim().to_lowercase())
            .into_iter()
            .flatten()
            .collect();

        #[cfg(any(debug_assertions, feature = "logging"))]
        println!("get_by_class: {:?}", start.elapsed());

        if taxonomies.is_empty() {
            None
        } else {
            Some(taxonomies)
        }
    }
}

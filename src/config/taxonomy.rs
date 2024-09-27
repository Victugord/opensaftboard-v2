use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

type TaxonomyHashMap = HashMap<String, Vec<Taxonomy>>;

#[derive( Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taxonomies {
    credits: TaxonomyHashMap,
    debits: TaxonomyHashMap,
}

lazy_static! {
    static ref BASE_TAXONOMIES: Taxonomies = {
        let start = Instant::now();

        // Load the JSON file for credits and debits at compile time
        let json_str = include_str!("base.json");
        println!("BASE JSON loading time: {:?}", start.elapsed());

        let start_parse = Instant::now();
        let mut data: Vec<Taxonomy> = serde_json::from_str(json_str).expect("BASE JSON was not well-formatted");
        println!("BASE JSON parse time: {:?}", start_parse.elapsed());

        let start_lower = Instant::now();
        // lower case all the debit and credit values
        data.iter_mut().for_each(|t| {
            t.debit = t.debit.to_lowercase();
            t.credit = t.credit.to_lowercase();
        });
        println!("BASE JSON lower time: {:?}", start_lower.elapsed());

        let mut credits = HashMap::new();
        let mut debits = HashMap::new();

        let start_populate = Instant::now();
        // Populate the HashMaps
        for taxonomy in data {
            credits.entry(taxonomy.credit.clone()).or_insert_with(Vec::new).push(taxonomy.clone());
            debits.entry(taxonomy.debit.clone()).or_insert_with(Vec::new).push(taxonomy);
        }
        println!("BASE JSON populate time: {:?}", start_populate.elapsed());

        Taxonomies { credits, debits }
    };
}

lazy_static! {
    static ref MICRO_TAXONOMIES: Taxonomies = {
        let start = Instant::now();

        // Load the JSON file for credits and debits at compile time
        let json_str = include_str!("micro.json");
        println!("MICRO JSON loading time: {:?}", start.elapsed());

        let start_parse = Instant::now();
        let mut data: Vec<Taxonomy> = serde_json::from_str(json_str).expect("MICRO JSON was not well-formatted");
        println!("MICRO JSON parse time: {:?}", start_parse.elapsed());

        let start_lower = Instant::now();
        // lower case all the debit and credit values
        data.iter_mut().for_each(|t| {
            t.debit = t.debit.to_lowercase();
            t.credit = t.credit.to_lowercase();
        });
        println!("MICRO JSON lower time: {:?}", start_lower.elapsed());

        let mut credits = HashMap::new();
        let mut debits = HashMap::new();

        let start_populate = Instant::now();
        // Populate the HashMaps
        for taxonomy in data {
            credits.entry(taxonomy.credit.clone()).or_insert_with(Vec::new).push(taxonomy.clone());
            debits.entry(taxonomy.debit.clone()).or_insert_with(Vec::new).push(taxonomy);
        }
        println!("MICRO JSON populate time: {:?}", start_populate.elapsed());

        Taxonomies { credits, debits }
    };
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Taxonomy {
    pub taxonomy_code: String,
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
    pub fn new(taxonomy_type: TaxonomyType) -> &'static Taxonomies {
        match taxonomy_type {
            TaxonomyType::Base => &BASE_TAXONOMIES,
            TaxonomyType::Micro => &MICRO_TAXONOMIES,
        }
    }

    pub fn get_by_dr(&self, dr: &str) -> Vec<Taxonomy> {
        let start = Instant::now();
        let lowered_dr = dr.to_lowercase();

        let credits = self.credits.get(&lowered_dr).unwrap_or(&vec![]).clone();
        let debits = self.debits.get(&lowered_dr).unwrap_or(&vec![]).clone();


        let taxonomies = credits.into_iter().chain(debits.into_iter()).collect();
        println!("get_by_dr: {:?}", start.elapsed());
        taxonomies


    }
}

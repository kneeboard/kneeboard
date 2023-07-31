use crate::calc::{convert_velocity, Degree};

use crate::definition::Plan;
use crate::diversion::create_wind_table;
use crate::route::{convert_leg, create_plog, Leg};
use pdf::{PDFDocument, PDFDocumentBuilder, A5};

pub fn create_planning(plan: &Plan) -> PDFDocument {
    let mut doc_builder = PDFDocumentBuilder::new();

    let details = &plan.detail;

    for route in &plan.routes {
        let legs: Vec<Leg> = route.legs.iter().map(convert_leg).collect();
        {
            let mut current_layer = doc_builder.create_page(A5);
            create_plog(&legs, &route.notes, details, &mut current_layer);
        }

        {
            let mut current_layer = doc_builder.create_page(A5);
            let reverse_legs: Vec<Leg> = legs.into_iter().map(rev_leg).rev().collect();
            create_plog(&reverse_legs, &route.notes, details, &mut current_layer);
        }
    }

    for diversion in &plan.diversions {
        let speed = diversion.aircraft_speed as f64;
        let wind = convert_velocity(&diversion.wind);
        let variation = Degree::new(diversion.variation as f64);
        let mut current_layer = doc_builder.create_page(A5);
        create_wind_table(&mut current_layer, speed, variation, &wind);
    }

    doc_builder.to_doc()
}

fn rev_leg(mut leg: Leg) -> Leg {
    let (from, to) = leg.name;
    leg.name = (to, from);
    leg.course = leg.course.reciprocal();
    leg
}

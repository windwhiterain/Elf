mod common;
use std::{collections::HashMap, sync::Arc};

use common::*;
fn main() {
    let mut ses = Vec::<Arc<Structure>>::new();
    let st1 = Arc::new(Structure::new_T());
    let st2 = Arc::new(Structure::new_T());
    let st3 = Arc::new(Structure::new_T());
    ses.push(st1);
    ses.push(st2);
    let sn1 = Arc::new(Structure::new_N(HashMap::from([
        ("a".into(), ses[0].clone()),
        ("b".into(), ses[1].clone()),
    ])));
    ses.push(sn1);
}

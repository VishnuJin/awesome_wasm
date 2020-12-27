extern crate wascc_actor as actor;
extern crate algos;

#[macro_use]
extern crate log;
extern crate serde;
extern crate wascc_codec;

use actor::prelude::*;
use serde::Serialize;
use algos::numerics::primorial;

actor_handlers! {
    codec::http::OP_HANDLE_REQUEST => primorial_num,
    codec::core::OP_HEALTH_REQUEST => health
}

fn primorial_num(r: codec::http::Request) -> HandlerResult<codec::http::Response> {
    info!("Query String: {}", r.query_string);
    let p = PrimorialNumberResponse {
        original: r.query_string.to_string(),
        primorial_value: {  
            let s = r.query_string.to_string();
            let n = s.trim().parse::<u128>().unwrap();
            format!("{}",primorial::primorial_big(n))
        }
    };
    
    Ok(codec::http::Response::json(p, 200, "OK"))
}

fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
    Ok(())
}

#[derive(Serialize)]
struct PrimorialNumberResponse {
    original: String,
    primorial_value: String,
}

use super::Parser;



pub fn parse_function(parser: &mut Parser, hash: u64) {
    let func = parser.cache.get_fn_from_hash(hash);
    println!("Found func {:?}, type: {}", func.name, func.native);
}



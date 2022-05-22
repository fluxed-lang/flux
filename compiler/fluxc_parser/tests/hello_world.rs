#[test]
fn hello_world() {
	let source = include_str!("./hello_world.flx");
	let parsed = fluxc_parser::parse(source).unwrap();
	assert_eq!(parsed.stmts.len(), 2);
}

fn main() {
    println!("cynic introspect https://swapi-graphql.netlify.app/.netlify/functions/index -o schemas/starwars.graphql");

    cynic_codegen::register_schema("starwars")
        .from_sdl_file("schemas/starwars.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
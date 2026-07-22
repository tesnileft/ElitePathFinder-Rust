fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "elite_pathfinder.gresource",
    );
}

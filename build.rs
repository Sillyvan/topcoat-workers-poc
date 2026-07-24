fn main() {
    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("feather")
        .stage()
        .expect("failed to stage the Feather icon set");

    topcoat::tailwind::BuildConfig::new()
        .input("styles.css")
        .output("styles.generated.css")
        .render()
        .expect("failed to build the Topcoat UI stylesheet");
}

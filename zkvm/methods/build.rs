use std::{collections::HashMap, path::PathBuf};

use risc0_build::{
    embed_methods_with_options, DockerOptionsBuilder, GuestOptionsBuilder,
};

fn main() {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let docker = DockerOptionsBuilder::default()
        .root_dir(repository_root)
        .env(vec![(
            "CARGO_NET_OFFLINE".to_string(),
            "true".to_string(),
        )])
        .build()
        .expect("the fixed Docker build options must be valid");
    let guest = GuestOptionsBuilder::default()
        .use_docker(docker)
        .build()
        .expect("the fixed guest build options must be valid");
    let mut options = HashMap::new();
    options.insert("vericode-guest", guest);

    embed_methods_with_options(options);
}


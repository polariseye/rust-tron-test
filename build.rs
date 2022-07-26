fn main(){
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/tron_grpc")
        //.type_attribute("routeguide.Point", "#[derive(Hash)]")
        .compile(&[
            "proto/api/api.proto",
            "proto/core/Discover.proto",
            "proto/core/Tron.proto",
            "proto/core/contract/smart_contract.proto",
        ], &["proto"])
        .unwrap();
}
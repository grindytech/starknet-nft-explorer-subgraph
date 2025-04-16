use cainome::rs::Abigen;
use std::collections::HashMap;

fn main() {
    // Aliases added from the ABI
    let mut aliases = HashMap::new();
    aliases.insert(
        String::from("openzeppelin_introspection::src5::SRC5Component::Event"),
        String::from("SRC5ComponentEvent"),
    );
    aliases.insert(
        String::from("openzeppelin_access::accesscontrol::accesscontrol::AccessControlComponent::Event"),
        String::from("AccessControlComponentEvent"),
    );
    aliases.insert(
        String::from("openzeppelin_token::erc721::erc721::ERC721Component::Event"),
        String::from("ERC721ComponentEvent"),
    );

    let erc721_abigen =
        Abigen::new("erc721", "./abi/erc721_contract.abi.json").with_types_aliases(aliases).with_derives(vec!["serde::Serialize".to_string(), "serde::Deserialize".to_string()]);

        erc721_abigen
            .generate()
            .expect("Fail to generate bindings")
            .write_to_file("./src/abi/erc721_contract.rs")
            .unwrap();
    // Aliases added from the ABI
    let mut aliases = HashMap::new();
    aliases.insert(
        String::from("openzeppelin_access::accesscontrol::accesscontrol::AccessControlComponent::Event"),
        String::from("AccessControlComponentEvent"),
    );
    aliases.insert(
        String::from("openzeppelin_token::erc1155::erc1155::ERC1155Component::Event"),
        String::from("ERC1155ComponentEvent"),
    );
    aliases.insert(
        String::from("openzeppelin_introspection::src5::SRC5Component::Event"),
        String::from("SRC5ComponentEvent"),
    );

    let erc1155_abigen =
        Abigen::new("erc1155", "./abi/erc1155_contract.abi.json").with_types_aliases(aliases).with_derives(vec!["serde::Serialize".to_string(), "serde::Deserialize".to_string()]);

        erc1155_abigen
            .generate()
            .expect("Fail to generate bindings")
            .write_to_file("./src/abi/erc1155_contract.rs")
            .unwrap();
}
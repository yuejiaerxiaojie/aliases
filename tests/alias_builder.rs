#![feature(plugin,const_fn)]
#![plugin(stainless)]

extern crate aliases;
extern crate yaml_rust;

#[cfg(test)]
mod tests {

    pub use yaml_rust::{YamlLoader};
    pub use aliases::{Alias, AliasBuilder};

    describe! alias_builder {

        describe! from_yaml {

            before_each {
                let yaml_string =
"command: ./super_command.sh
confirm: true
confirmation_message: Are you really really sure??
conditional: /bin/true
unit_test: '[ true = true ]'
";
                let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                let doc = &docs[0];
            }

            describe! with_all_the_field_included {

                before_each {
                    let alias = AliasBuilder::from_yaml("command_name", doc.clone()).build();
                }

                it "creates an Alias with all the fields set" {
                    assert_eq!(alias.name, "command_name");
                    assert_eq!(alias.command, "./super_command.sh");
                    assert_eq!(alias.confirm, true);
                    assert_eq!(alias.confirmation_message, "Are you really really sure??");
                    assert_eq!(alias.conditional, Some("/bin/true".to_string()));
                    assert_eq!(alias.unit_test, Some("[ true = true ]".to_string()));
                }
            }

            describe! when_there_is_no_name {
            }

            describe! when_there_is_no_command {
            }

            describe! when_there_is_no_confirmation {

                before_each {
                let yaml_string =
"command: ./super_command.sh
unit_test: '[ true == true ]'
conditional: /bin/true
confirmation_message: Are you really really sure??
";
                let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                let doc = &docs[0];
                }

                it "builds with confirmation turned off" {
                    let alias = AliasBuilder::from_yaml("command_name", doc.clone()).build();
                    assert_eq!(alias.confirm, false);
                }

            }

            describe! when_there_is_no_confirmation_message {

                before_each {
                let yaml_string =
"command: ./super_command.sh
confirm: true
unit_test: '[ true == true ]'
conditional: /bin/true
";
                let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                let doc = &docs[0];
                }

                it "builds with a default confirmation message" {
                    let alias = AliasBuilder::from_yaml("command_name", doc.clone()).build();
                    assert_eq!(alias.confirmation_message, "About to execute `./super_command.sh`");
                }
            }

            describe! when_there_is_no_conditional {

                before_each {
                let yaml_string =
"command: ./super_command.sh
confirm: true
confirmation_message: Are you really really sure??
unit_test: '[ true == true ]'
";
                let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                let doc = &docs[0];
                }

                it "builds without a conditional" {
                    AliasBuilder::from_yaml("command_name", doc.clone()).build();
                }

            }

            describe! when_there_is_no_unit_test {

                before_each {
                let yaml_string =
"command: ./super_command.sh
confirm: true
confirmation_message: Are you really really sure??
conditional: /bin/true
";
                let docs = YamlLoader::load_from_str(yaml_string).unwrap();
                let doc = &docs[0];
                }

                it "builds without a unit test" {
                    AliasBuilder::from_yaml("command_name", doc.clone()).build();
                }
            }
        }
    }
}
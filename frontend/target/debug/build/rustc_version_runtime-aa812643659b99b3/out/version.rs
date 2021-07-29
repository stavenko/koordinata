
            use rustc_version::{Channel, VersionMeta};
            use semver;
            
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: semver::Version {
                        major: 1,
                        minor: 53,
                        patch: 0,
                        pre: vec![semver::Identifier::AlphaNumeric("nightly".to_owned()), ],
                        build: vec![],
                    },
                    host: "x86_64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.53.0-nightly (42816d61e 2021-04-24)".to_owned(),
                    commit_hash: Some("42816d61ead7e46d462df997958ccfd514f8c21c".to_owned()),
                    commit_date: Some("2021-04-24".to_owned()),
                    build_date: None,
                    channel: Channel::Nightly,
                }
            }
            
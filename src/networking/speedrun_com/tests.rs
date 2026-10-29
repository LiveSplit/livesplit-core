//! Wire-format and URL-policy regressions, with opt-in live API checks.

use super::*;
use serde_json::json;

#[track_caller]
fn category() -> Category {
    serde_json::from_value(json!({
        "id": "any",
        "name": "Any%",
        "type": "per-game",
        "rules": null
    }))
    .unwrap()
}

#[track_caller]
fn variable(scope: &str, category: Option<&str>) -> Variable {
    serde_json::from_value(json!({
        "id": "variable",
        "name": "Difficulty",
        "category": category,
        "scope": {
            "type": scope,
            "level": "level"
        },
        "values": {
            "values": {
                "normal": {
                    "label": "Normal"
                },
                "hard": {
                    "label": "Hard",
                    "rules": null
                }
            },
            "default": null
        },
        "mandatory": true,
        "is-subcategory": false
    }))
    .unwrap()
}

#[test]
fn invalid_user_agent_header_values_are_rejected() {
    for user_agent in ["Timer/1.0\r\nInjected: value", "Timer/1.0\0"] {
        assert!(matches!(
            Client::new(user_agent),
            Err(Error::Http(error)) if error.is_builder()
        ));
    }
}

#[test]
fn variable_scope_and_category_match_the_web_editor() {
    let category = category();

    for scope in ["global", "full-game", "all-levels", "single-level"] {
        let full_game = matches!(scope, "global" | "full-game");

        for id in [None, Some("any"), Some("other")] {
            let variable = variable(scope, id);

            assert_eq!(
                variable.applies_to(Some(&category)),
                full_game && id != Some("other")
            );

            assert_eq!(variable.applies_to(None), full_game && id.is_none());

            assert_eq!(
                variable
                    .values
                    .values
                    .iter()
                    .map(|(_, value)| value.label.as_str())
                    .collect::<Vec<_>>(),
                ["Hard", "Normal"]
            );
        }
    }
}

#[test]
fn variable_values_preserve_wire_order_instead_of_sorting_by_id() {
    // Parse the wire text directly: json! would sort these object keys before
    // the ordered-map deserializer receives them.
    let values: VariableValues = serde_json::from_str(
        r#"{
            "values": {
                "z": {
                    "label": "First"
                },
                "a": {
                    "label": "Second"
                }
            },
            "default": null
        }"#,
    )
    .unwrap();

    assert_eq!(
        values
            .values
            .iter()
            .map(|(_, value)| value.label.as_str())
            .collect::<Vec<_>>(),
        ["First", "Second"]
    );
}

#[test]
fn bulk_headers_do_not_require_twitch_names_or_full_game_fields() {
    let response: Paginated<GameHeader> = serde_json::from_value(json!({
        "data": [
            {
                "id": "game",
                "names": {
                    "international": "ゲーム & Game",
                    "japanese": null
                },
                "abbreviation": "game",
                "weblink": "https://www.speedrun.com/game"
            }
        ],
        "pagination": {
            "links": []
        }
    }))
    .unwrap();

    assert_eq!(response.data[0].names.international, "ゲーム & Game");

    assert!(
        response
            .pagination
            .links
            .is_some_and(|links| links.is_empty())
    );
}

#[test]
fn absent_or_null_pagination_links_end_a_collection() {
    for response in [
        json!({
            "data": [],
            "pagination": {}
        }),
        json!({
            "data": [],
            "pagination": {
                "links": null
            }
        }),
    ] {
        let page: Paginated<GameHeader> = serde_json::from_value(response).unwrap();

        assert!(page.pagination.links.is_none());
    }
}

#[test]
fn optional_artwork_and_embedded_variables_are_distinct() {
    let mut data = json!({
        "id": "game",
        "names": {
            "international": "Game",
            "japanese": null
        },
        "abbreviation": "game",
        "weblink": "https://www.speedrun.com/game",
        "assets": {
            "cover-medium": {
                "uri": "https://www.speedrun.com/images/blankcover.png"
            },
            "icon": {
                "uri": "https://www.speedrun.com/images/1st.png"
            }
        },
        "ruleset": {
            "emulators-allowed": false
        },
        "platforms": [],
        "regions": [],
        "variables": {
            "data": []
        }
    });
    let game: Game = serde_json::from_value(data.clone()).unwrap();

    assert!(game.artwork(Artwork::BoxArt).is_none());
    assert!(game.artwork(Artwork::Icon).is_none());

    data["assets"]["icon"] = serde_json::Value::Null;
    data["assets"]["cover-medium"]["uri"] = "https://cdn.example/cover.png".into();

    let game: Game = serde_json::from_value(data.clone()).unwrap();

    assert!(game.artwork(Artwork::Icon).is_none());
    assert!(game.artwork(Artwork::BoxArt).is_some());

    data.as_object_mut().unwrap().remove("variables");

    assert!(serde_json::from_value::<Game>(data).is_err());
}

#[test]
fn pagination_upgrades_http_but_stays_on_the_same_api_route() {
    let current = Client::url(&["games"]).unwrap();

    assert!(matches!(
        continuation(&current, current.as_str()),
        Err(Error::PaginationCycle)
    ));

    assert_eq!(
        continuation(&current, "http://www.speedrun.com/api/v1/games?offset=1000")
            .unwrap()
            .scheme(),
        "https"
    );

    assert!(continuation(&current, "?offset=1000").is_ok());

    for uri in [
        "https://example.com/api/v1/games",
        "https://www.speedrun.com/api/v1/regions",
        "https://user@www.speedrun.com/api/v1/games",
        "ftp://www.speedrun.com/api/v1/games",
        "https://www.speedrun.com:123/api/v1/games",
        "?offset=1000#fragment",
    ] {
        assert!(continuation(&current, uri).is_err(), "{uri}");
    }
}

#[test]
fn path_segments_and_query_parameters_cannot_inject_another_request() {
    let mut url = Client::url(&["games", "name/with?query#fragment", "categories"]).unwrap();

    assert_eq!(
        url.path(),
        "/api/v1/games/name%2Fwith%3Fquery%23fragment/categories"
    );

    url.query_pairs_mut().append_pair("name", "ゲーム & max=1");

    assert_eq!(
        url.query_pairs().collect::<Vec<_>>(),
        [("name".into(), "ゲーム & max=1".into())]
    );
}

#[test]
fn empty_or_dot_game_ids_cannot_change_the_endpoint() {
    for id in ["", ".", ".."] {
        for segments in [vec!["games", id], vec!["games", id, "categories"]] {
            assert!(
                matches!(Client::url(&segments), Err(Error::InvalidUrl)),
                "{segments:?}"
            );
        }
    }

    // Dots inside a real abbreviation are still an ordinary encoded segment.
    assert_eq!(
        Client::url(&["games", "game.v2"]).unwrap().path(),
        "/api/v1/games/game.v2"
    );
}

#[test]
fn control_characters_cannot_rewrite_game_ids() {
    for id in [".\t.", ".\n.", ".\r.", "\t", "ab\tc", "\0"] {
        assert!(
            matches!(Client::url(&["games", id]), Err(Error::InvalidUrl)),
            "{id:?}"
        );
    }
}

// Tokio's test runtime is a native-only development dependency. Keep live
// checks together so metadata parsing remains testable on browser targets.
#[cfg(not(target_family = "wasm"))]
mod live {
    //! Opt-in checks against the actual metadata and artwork servers.

    use super::*;

    const USER_AGENT: &str = "livesplit-core-tests";

    #[tokio::test]
    #[ignore = "requires network access to the live Speedrun.com API"]
    async fn live_editor_metadata_and_pagination() {
        let client = Client::new(USER_AGENT).unwrap();
        let headers = client.game_headers().await.unwrap();

        assert!(!headers.items.is_empty());

        if let Some(next) = client.next(&headers).await.unwrap() {
            assert!(!next.items.is_empty());
        }

        let game = client.game("sm64").await.unwrap();

        assert!(!game.header.names.international.is_empty());
        assert!(!client.categories(&game.header.id).await.unwrap().is_empty());

        assert!(
            !client
                .collect(client.platforms().await.unwrap())
                .await
                .unwrap()
                .is_empty()
        );

        assert!(
            !client
                .collect(client.regions().await.unwrap())
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    #[ignore = "requires live Speedrun.com artwork access, which may be challenged by Cloudflare"]
    async fn live_game_artwork() {
        let client = Client::new(USER_AGENT).unwrap();
        let game = client.game("sm64").await.unwrap();

        assert!(
            !client
                .artwork(&game, Artwork::BoxArt)
                .await
                .unwrap()
                .is_empty()
        );
    }
}

#[test]
fn rule_requirements_and_markdown_image_urls_are_typed() {
    let rules: Rules = serde_json::from_value(json!({
        "emulators-allowed": true,
        "require-video": true,
        "default-time": "realtime_noloads"
    }))
    .unwrap();

    assert!(rules.require_video);
    assert_eq!(rules.default_time, TimingMethod::RealTimeWithoutLoads);

    for url in [
        "http://example.org/image.png",
        "file:///image.png",
        "https://user@example.org/image.png",
        "javascript:alert(1)",
    ] {
        assert!(image_url(url).is_err());
    }

    assert_eq!(
        image_url("https://example.org/image.png").unwrap().scheme(),
        "https"
    );
}

use std::collections::HashMap;

use valhalla::{
    proto::{costing, lat_lng, options, Api, Costing, LatLng, Location, Options},
    ProtobufActor,
};

#[test]
fn test_proto_route() {
    let actor = ProtobufActor::new("tests/valhalla.json");
    let api = Api {
        options: Some(Options {
            has_id: Some(options::HasId::Id(String::from("kotti_to_hermannplatz"))),
            units: 0, // km
            locations: vec![
                Location {
                    ll: Some(LatLng {
                        has_lat: Some(lat_lng::HasLat::Lat(52.499078)),
                        has_lng: Some(lat_lng::HasLng::Lng(13.418230)),
                    }),
                    name: "Kottbusser Tor".into(),
                    ..Default::default()
                },
                Location {
                    ll: Some(LatLng {
                        has_lat: Some(lat_lng::HasLat::Lat(52.487331)),
                        has_lng: Some(lat_lng::HasLng::Lng(13.425042)),
                    }),
                    name: "Hermannplatz".into(),
                    ..Default::default()
                },
            ],
            costing_type: costing::Type::Auto.into(),
            costings: HashMap::from([(
                costing::Type::Auto.into(),
                Costing {
                    has_options: Some(costing::HasOptions::Options(costing::Options {
                        has_alley_factor: Some(costing::options::HasAlleyFactor::AlleyFactor(1.0)),
                        has_use_highways: Some(costing::options::HasUseHighways::UseHighways(0.5)),
                        has_use_tolls: Some(costing::options::HasUseTolls::UseTolls(0.5)),
                        has_use_distance: Some(costing::options::HasUseDistance::UseDistance(0.)),
                        has_height: Some(costing::options::HasHeight::Height(1.6)),
                        has_width: Some(costing::options::HasWidth::Width(1.9)),
                        has_shortest: Some(costing::options::HasShortest::Shortest(true)),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let r = actor.route(&api).unwrap();
    println!("{:?}", r);
}

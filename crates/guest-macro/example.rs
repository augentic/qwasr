use std::env;

guest_macro::guest!({
    owner: "at",
    http: [
        "/jobs/detector": get(DetectionRequest with_query, DetectionReply),
        "/god-mode/set-trip/{vehicle_id}/{trip_id}": post(SetTripRequest with_body, SetTripReply),
    ],
    messaging: [
        "realtime-r9k.v1": R9kMessage,
    ]
});

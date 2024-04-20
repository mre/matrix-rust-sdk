use matrix_sdk_test::{async_test, ALICE, BOB};
use ruma::{
    events::{
        beacon::unstable_start::{
            NewUnstableBeaconStartEventContent, ReplacementUnstableBeaconStartEventContent,
        },
        AnyMessageLikeEventContent,
    },
    server_name, EventId, OwnedEventId, UserId,
};

use crate::timeline::{
    beacons::BeaconInfo, tests::TestTimeline, EventTimelineItem, TimelineItemContent,
};

#[async_test]
async fn beacon_is_displayed() {
    let timeline = TestTimeline::new();

    timeline.send_beacon_start(&ALICE, fakes::beacon_a()).await;
    let beacon_state = timeline.beacon_state().await;

    assert_beacon_start_eq(&beacon_state.start_event_content.beacon_start, &fakes::beacon_a());
    assert!(beacon_state.response_data.is_empty());
}

impl TestTimeline {
    async fn send_beacon_start_with_id(
        &self,
        sender: &UserId,
        event_id: &EventId,
        content: UnstableBeaconStartContentBlock,
    ) {
        let event_content = AnyMessageLikeEventContent::UnstableBeaconStart(
            NewUnstableBeaconStartEventContent::new(content).into(),
        );
        let event =
            self.event_builder.make_sync_message_event_with_id(sender, event_id, event_content);
        self.handle_live_event(event).await;
    }
}

mod fakes {
    use ruma::events::beacon::unstable_start::{
        NewUnstableBeaconStartEventContent, UnstableBeaconStartContentBlock,
    };

    // pub struct UnstableBeaconStartEventContent {
    //     /// The description of the location.
    //     ///
    //     /// It should be used to label the location on a map.
    //     #[serde(skip_serializing_if = "Option::is_none")]
    //     pub description: Option<String>,

    //     /// `live` is a boolean that should be true when a user starts sharing location.
    //     pub live: bool,

    //     /// `ts` is an optional `MilliSecondsSinceUnixEpoch` that represents the timestamp of the
    //     /// event.
    //     #[serde(rename = "org.matrix.msc3488.ts", skip_serializing_if = "Option::is_none")]
    //     pub ts: Option<MilliSecondsSinceUnixEpoch>,

    //     /// `timeout` represents the length of time in milliseconds that the location
    //     /// will be live. So the location will stop being shared at `m.ts + timeout` milliseconds
    //     /// since the epoch.
    //     #[serde(default, with = "ruma_common::serde::duration::ms")]
    //     pub timeout: Duration,

    //     /// `asset` is an `AssetContent` that this message refers to.
    //     #[serde(
    //         default,
    //         rename = "org.matrix.msc3488.asset",
    //         skip_serializing_if = "ruma_common::serde::is_default"
    //     )]
    //     pub asset: AssetContent,
    // }

    pub fn beacon_a() -> UnstableBeaconStartEventContent {
        UnstableBeaconStartEventContent::new("Central Park".to_string(), Duration::from_secs(60))
    }
}

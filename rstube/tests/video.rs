use rstest::rstest;
use rstube::{
    client::RusTube,
    error::{ExtractionError, UnavailabilityReason},
    models::channel::Verification,
};

#[tokio::test]
async fn video_live() -> anyhow::Result<()> {
    let id = "jfKfPfyJRdk";
    let res = RusTube::new().query().video(id).send().await?;
    assert_eq!(res.id, id);
    assert!(res.is_live);
    assert_eq!(res.title, "lofi hip hop radio 📚 beats to relax/study to");
    assert_eq!(
        Some(true),
        res.description
            .map(|d| d.contains("Listen on Spotify, Apple music and more"))
    );
    assert_eq!(res.channel.id, "UCSJ4gkVC6NrvII8umztf0Ow");
    assert_eq!(res.channel.name, "Lofi Girl");
    assert!(!res.channel.avatar.is_empty(), "no channel avatars");
    Ok(())
}

#[rstest]
#[case("qoWRs7lXtYE", Verification::Artist)]
#[case("XcK35ikbKGk", Verification::Verified)]
#[case("TbAsRmHYpKc", Verification::None)]
#[tokio::test]
async fn channel_verification(
    #[case] video_id: &str,
    #[case] verification: Verification,
) -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client.query().video(video_id).send().await?;
    assert_eq!(res.channel.verification, verification);
    Ok(())
}

#[rstest]
#[case::private("s7_qI6_mIXc", UnavailabilityReason::Private)]
#[case::deleted("64DYi_8ESh0", UnavailabilityReason::Deleted)]
#[case::censored("6SJNVb0GnPI", UnavailabilityReason::Deleted)]
#[tokio::test]
async fn video_unavailable(#[case] video_id: &str, #[case] expect: UnavailabilityReason) {
    use rstube::error::Error;

    let client = RusTube::new();
    let err = client.query().video(video_id).send().await.unwrap_err();

    match err {
        Error::Extraction(ExtractionError::Unavailable { reason, .. }) => {
            assert_eq!(reason, expect, "got {err}")
        }
        _ => panic!("got {err}"),
    }
}

#[rstest]
// This video is geoblocked outside of Japan, so expect this test case to fail when using a Japanese IP address.
#[case::geoblock("sJL6WA-aGkQ")]
#[case::age_restricted("CUO8secmc0g")]
#[case::premium_only("3LvozjEOUxU")]
#[case::members_only("vYmAhoZYg64")]
#[tokio::test]
async fn video_restricted(#[case] id: &str) -> anyhow::Result<()> {
    let client = RusTube::new();
    let res = client.query().video(id).send().await?;

    assert_eq!(res.id, id);
    assert!(!res.title.is_empty());
    assert!(res.duration.is_some());
    assert!(!res.thumbnail.is_empty());
    assert!(res.publish_date.is_some());
    assert!(!res.is_live);
    assert!(!res.is_short);

    assert!(!res.channel.id.is_empty());
    assert!(!res.channel.name.is_empty());
    assert!(!res.channel.avatar.is_empty());

    Ok(())
}

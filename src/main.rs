use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use chromiumoxide::page::ScreenshotParams;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::sleep;

#[cfg(target_os = "windows")]
const CHROME_PATH: &str = "./chrome_win_x64/chrome.exe";
#[cfg(target_os = "linux")]
const CHROME_PATH: &str = "./chrome_linux_x64/chrome";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .chrome_executable(CHROME_PATH)
            .with_head()
            .incognito()
            .args(["--headless=new"])
            .viewport(None)
            .build()?,
    )
    .await?;
    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page = browser
        .new_page("https://buzkaaclicker.pl/download/essa") // cf turnstile challenge
        .await?;
    page.wait_for_navigation().await?;
    // wait for turnstile to load, do something better than that
    sleep(Duration::from_secs(5)).await;
    let html = page.content().await?;
    println!("Waf html?: {html}");
    if !html.contains("challenges.cloudflare.com") {
        panic!("no waf challenge");
    }
    let turnstile = page.find_element("iframe").await?;
    page.click(turnstile.clickable_point().await?).await?;

    sleep(Duration::from_secs(5)).await;
    let html = page.content().await?;
    println!("New html: {html}");
    page.save_screenshot(
        ScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .full_page(true)
            .build(),
        "example.png",
    )
    .await?;

    browser.close().await?;
    let _ = handle.await;
    Ok(())
}

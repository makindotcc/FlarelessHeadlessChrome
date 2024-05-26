# FlarelessHeadlessChrome
Pass cloudflare turnstile challenge using patched chrome binary (Windows/Linux x64). \
Tested & working also at:
- [friendlycaptcha](https://friendlycaptcha.com/)
- [vercel waf](https://vercel.com/docs/security/vercel-firewall)

## How it works
Currently, with [new headless mode](https://developer.chrome.com/articles/new-headless/) the only thing
that stops us from browsing sites behind CF waf is:
```js
console.log(navigator.webdriver) // prints true while using CDP
```
And ```HeadlessChrome``` useragent.

Trying to patch it using javascript is challenging, because it can be detected in 10000 ways. 

So I made simple patches to chrome binary using my tool [fabricbin](https://github.com/makindotcc/fabricbin) which
searches for code patterns and replaces it with defined in config ([windows_patch.yaml](windows_patch.yaml) or [linux_patch.yaml](linux_patch.yaml)).

## Applied patches

- blink::Navigator::webdriver
```c
return 0;
```
- embedder_support::GetUserAgentInternal
![Assembly code of GetUserAgentInternal function with arrow pointed at \
if (base::CommandLine::HasSwitch("headless")) \
with text "nop this :D" using comic sans font](screenshots%2FGetUserAgentPatch.png)

## Usage

1. Copy & paste chrome files to ``./chrome_win_x64/`` or ``./chrome_linux_x64/`` directory.

2. Install [fabricbin](https://github.com/makindotcc/fabricbin)
```bash
cargo install --git https://github.com/makindotcc/fabricbin
```

3. Patch ``chrome`` (on linux) or ``chrome.dll`` (on windows) in our ``./chrome_linux_x64/`` or ``./chrome_win_x64/`` directory.
```bash
# Create original binary copy.
# change version number '118.0.5993.71' to version you own
# linux
cp chrome_linux_x64/chrome chrome_linux_x64/chrome_org
# or windows
cp chrome_win_x64/118.0.5993.71/chrome.dll chrome_win_x64/118.0.5993.71/chrome_org.dll
```
Edit [linux_patch.yaml](linux_patch.yaml) / [windows_patch.yaml](windows_patch.yaml) with right ``input_file`` and ``output_file`` (change version number to your current
chrome version like (on windows) ``./chrome_win_x64/118.0.5993.71/chrome_org.dll`` to ``./chrome_win_x64/110.0.2993.35/chrome_org.dll``)

```bash
fabricbin linux_patch.yaml
# or
fabricbin windows_patch.yaml
```

4. Done, you can browse websites behind cloudflare waf using 
modified chrome build with CDP and new headless (``--headless=new``). \
For example usage see [src/main.rs](src%2Fmain.rs)

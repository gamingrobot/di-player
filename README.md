
# DI Desktop Player

## Supports

- DI.FM
- RadioTunes
- Zen Radio
- Rock Radio
- Classical Radio
- Jazz Radio

## Linux

### Install dependencies

```
sudo dnf install gtk3-devel webkit2gtk4.1-devel
```

### Notes

- Currently requires `WEBKIT_DISABLE_DMABUF_RENDERER=1` under Wayland when using Nvidia drivers
- Setting `GST_PLUGIN_FEATURE_RANK=pulsesink:0,alsasink:MAX` fixes an issue of other pulseaudio streams marked with `module-stream-restore.id = "sink-input-by-media-role:video"` to be muted. This is due to di.fm using NoSleep.js which loads a muted webm video.

---
**This project is not affiliated or endorsed by AudioAddict/DI.FM**

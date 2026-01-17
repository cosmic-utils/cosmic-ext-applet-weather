# Simple weather info applet for cosmic

<p align="center">
    <img alt="Applet Screenshot" src="https://github.com/cosmic-utils/cosmic-ext-applet-weather/blob/main/data/applet_screenshot_1.png">
</p>

<p align="center">
    <img alt="Applet Screenshot" src="https://github.com/cosmic-utils/cosmic-ext-applet-weather/blob/main/data/applet_screenshot_2.png">
</p>

<p align="center">
    <img alt="Applet Screenshot" src="https://github.com/cosmic-utils/cosmic-ext-applet-weather/blob/main/data/applet_screenshot_3.png">
</p>

## Installation

### Flatpak

Depending on how you've installed COSMIC Desktop, the Weather applet may show up in your app store by default. In COSMIC Store it should be under the "COSMIC Applets" category.

If the applet does not show up in your app store, you'll need to add `cosmic-flatpak` as a source:

```sh
flatpak remote-add --if-not-exists --user cosmic https://apt.pop-os.org/cosmic/cosmic.flatpakrepo
```

Then, proceed to your preferred app store and search for Weather applet.

### Manual

The applet can be installed using the following steps:

```sh
sudo apt install libxkbcommon-dev just
git clone https://github.com/cosmic-utils/cosmic-ext-applet-weather.git
cd cosmic-ext-applet-weather
just build
sudo just install
```

`libxkbcommon-dev` is required by `smithay-client-toolkit`

## Configuration

The applet provides a graphical interface for entering coordinates as well as a toggle to get it automatically based on your IP address. There is also a unit selector to choose between Celsius and Fahrenheit.
To refresh the applet simply run `pkill cosmic-panel`

## Uninstall

To uninstall files installed by `just install`, run:

```sh
sudo just uninstall
```

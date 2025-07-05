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

The applet currently does not have a graphical interface for setting the coordinates to fetch and display the temperature for a specific location. To set the current location, specify the latitude and longitude in the configuration files.


_Use IP-API web service (https://ip-api.com/docs/api:json) to retrieve approximate coordinates or alternatively, use mapping platforms like Google Maps to obtain accurate latitude and longitude._


```sh
cd ~/.config/cosmic/io.github.cosmic-utils.cosmic-ext-applet-weather/v1/
```

Create and add latitude in the `latitude` file:

```
touch latitude
echo "12.123163" > latitude
```

Similarly for longitude:

```
touch longitude
echo "23.811234" > longitude
```

The applet refreshes every minute, and the new coordinates will be used only at that time. As a workaround, simply remove and re-add the applet in the panel settings for an instant refresh.

## Uninstall

To uninstall files installed by `just install`, run:

```sh
sudo just uninstall
```

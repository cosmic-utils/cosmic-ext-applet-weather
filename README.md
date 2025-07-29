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

<p align="center">
    <img alt="Applet Screenshot" src="https://github.com/cosmic-utils/cosmic-ext-applet-weather/blob/main/data/applet_screenshot_4.png">
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

The applet provides a graphical interface for entering coordinates and toggling the Fahrenheit scale. For manual configuration, follow the steps below.

_Use the IP-API web service (https://ip-api.com/docs/api:json) to retrieve approximate coordinates, or alternatively, use mapping platforms like Google Maps to obtain accurate latitude and longitude._


```sh
cd ~/.config/cosmic/io.github.cosmic_utils.weather-applet/v1/
```

Add latitude:

```
touch latitude
echo "12.123163" > latitude
```

Add longitude:

```
touch longitude
echo "23.811234" > longitude
```

Toggle Fahrenheit:

```
touch use_fahrenheit
echo "true" > use_fahrenheit
```

To refresh the applet simply run `pkill cosmic-panel`

## Uninstall

To uninstall files installed by `just install`, run:

```sh
sudo just uninstall
```

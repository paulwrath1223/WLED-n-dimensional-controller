# WLED-n-dimensional-controller


Goal:
  Be able to control a string of rgb lights and address them by 3d (or 2 or 1 or even n dimensions) coordinates to allow for cool effects when the string is set up in an in-organic shape.
  Like [this example by mattparker](https://www.youtube.com/watch?v=TvlpIojusBE), but more modular, wirelessly compatible with WLED, and most importantly: in rust. 


## NOT WORKING, VERY WIP.

## TODO:
- [x] way to stream colors to WLED. Thanks @coral
- [x] way to access the WLED JSON API.
- [ ] struct for an n dimension universe (a pairing for a set of leds, the way to set their color, and their coordinates, and the effect specific data like sliders and time)
- [ ] implementations for said structs
- [ ] effects. hopefully include gif support for 2d universes
- [ ] way to map leds to coords with a mobile phone or at least anything except manually measuring physical coordinates for every LED.
- [ ] UI. this is not my expertise and honestly I pray that someone does this part for me when I get to this stage
      

This is the beginning of what will be a design document for bevy_granite.

bevy_granite is a digital CAD & tooling framework, based on the Bevy game engine. This library is meant to be a central design around which a CAD tool is made. Users of one granite-based tool should easily use another one made with bevy_granite. Tools made with this library will share many features and styles, especially around UI.  Naturally, it will be opinionated, almost tyrannically so. To this end, bevy_granite's features will also impose constraints on tool developers using the library, to prevent common design errors present on many older and proprietary CAD software packages.

I find myself creating tools for projects at work and at home, especially to work around the traditional CAD tools, which can be absurdly expensive and difficult to work with. This library is intended to condense, from these tools, all of the favorite features and lessons learned. 

Let's see how this turns out!
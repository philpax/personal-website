+++
title = "Ocufabulous: An Oculus Rift DK2 Review"
short = "Reviewing the second Oculus Rift developer kit and predicting the future for VR."
date = 2014-10-12

[taxonomies]
tags=["xr"]
+++

The [Oculus Rift Development Kit 2](http://www.oculus.com/blog/announcing-the-oculus-rift-development-kit-2-dk2/) is Oculus' latest virtual reality headset available to the "public." As the name suggests, it exists to allow developers to produce and test experiences for the eventual consumer version.

My own DK2 arrived several weeks ago, giving me the chance to try out several interesting experiences and to start work on my own VR applications. To that end, what follows are my thoughts on the headset and a few of the demos and games I've tried out.

<!-- more -->

# The Headset
![Photo of the DK2 box](box.jpg)
![Photo of the contents of the DK2 box](box-contents.jpg)

Where better to start than the packaging? The DK2's box is cardboard; while decently built and protective of its contents, it’s not quite as luxurious as the DK1’s packaging - a sturdy, solid black plastic container with the Oculus logo front and center. This is understandable for reasons of bringing production costs down, but it's still somewhat of a disappointment.

The box itself contains a variety of items; about what you’d expect, really - the headset itself, the positional tracking camera, some cables, and the B set of lenses. Quick tangent: the Rift works by magnifying a per-eye view via lenses, enveloping your field of view with an artificial image. Each eye receives a separate view, which your brain then resolves into a stereoscopic 3D image. However, not everyone has the same level of sightedness. The default lenses for the Rift, the A lenses, are designed for use by normal-sighted and far-sighted users; as a result, the B lenses are provided for moderately near-sighted users. Users who are more near-sighted may need alternative solutions, such as wearing their glasses inside the Rift.

![Photo of the DK2](DSC_8591.jpg)
![Photo of the DK2 positional tracking camera](DSC_8589.jpg)
![Photo of the small box that connects the DK2 to the computer](DSC_8597.jpg)

The DK2 is relatively trivial to set up. Two cables extend from the DK2, merging into a singular cable that terminates at a small box that provides a HDMI cable (for the visual image) and a USB cable (for the tracking). The positional tracking camera has its own USB cable, as well as a sync cable which connects to the aforementioned box.
The camera needs to be placed in a location where it can see the headset; the further away the camera is, the larger the volume in which the headset can be tracked. This means placement of the camera is essential to a decent VR experience - there’s nothing more disorienting then having the world suddenly stop moving with you!

After the initial configuration, the next step is to install the Oculus Runtime, which provides the drivers and configuration utility. From there, software set-up is relatively simple; create a new profile, enter your eye height, and (if necessary) enter your interpupillary distance (distance between your eyes).
At the time of writing, the DK2 supports two modes: Direct(-to-Rift) Mode, and Extended Mode. The former renders images directly to the Rift without any abstraction layers, while the latter presents the Rift as another monitor. Of the two, Direct Mode is the preferable option as it reduces latency, allows for mirroring to another display, and is (theoretically) easier to use. However, it is still problematic, requires support from the application, and is only supported on Windows with applications that use DirectX. As a result, Extended Mode is still in use for older applications, applications which use OpenGL, and applications on OS X.

By default, the DK2 is in Direct Mode. Due to graphics card troubles, I have an unconventional configuration; a monitor and the DK2 are driven by a dedicated graphics card, and another monitor is driven by my computer's integrated graphics. However, this configuration does not interact well with Direct Mode; the demo scene (detailed later) ran at 3-4 FPS and failed to display on the Rift. After disabling the integrated graphics monitor, everything worked fine.

![Photo of the DK2 from a raised rear position, showing the straps](DSC_8607.jpg)
![Photo of the DK2 from the rear, showing the foam and lenses](DSC_8616.jpg)

Ergonomically, the headset is acceptable. It’s not the most comfortable thing - you can definitely feel its presence - but it’s not stifling. Out of the box, it uncomfortably pressed against my nose; adjusting the distance between the screen assembly and eyes via the physical dials (pictured left of the left strap) helped resolve this issue. The straps make it relatively easy to securely mount it to one’s head; they’re noticeable when you put them on, but easier to ignore when actually using the headset. The straps either go under your ears or above; the former can be uncomfortable, while the latter makes it difficult to use headphones.

The biggest issue for me, personally, is the lack of hardware-adjustable interpupillary distance. There are two important factors regarding IPD: software IPD, and hardware IPD. Software IPD controls how far apart the rendered views are, and is crucial to showing objects with the correct scale. Hardware IPD/lens separation is how far apart the lenses are on the Rift; this is fixed to 63.5 mm (average human IPD) on the Rift, as you can see in the photo.

Unfortunately, my IPD is 68.4 mm; this means that the lenses don’t line up exactly with my eyes. As a result, one eye is always slightly blurry. This makes using the Rift an uncomfortable experience at times; I can hold the DK2 in such a way so that the IPD mismatch is evenly distributed between the eyes, but it’s still not particularly ideal. IPD adjusters from [VR-Gear](http://www.vr-gear.com/) claim to resolve this issue, but I haven't tried one myself. Hopefully, Oculus can fix this issue for the consumer version.

![Photo of the DK2 through the left eye lens (not representative)](DSC_8625.jpg)
![Zoomed-in photo of the DK2 through the left eye lens (not representative)](DSC_8624.jpg)
![Photo showing the pixel grid of the panel](DSC_8619.jpg)

These photos are not representative of what you actually see in the DK2; however, they illustrate the screen panel, and demonstrate the effect of the "sweet spot." At the centre of the lenses, there is a small circle in which the view is focused; if you stray from this circle, as these photos illustrate, the image will be blurred around the edges. This ties into the lens separation issue - as my eyes don't line up with the sweet spot of both lenses, there is always some degree of blurriness.

The resolution (960x1080 per eye) is acceptable; it’s considerably better than the DK1, but it’s not something you can quite ignore. The screen-door effect - the result of the screen's grid of pixels being magnified - is still visible, but overall improved over the DK1. Of note is that the panel is a PenTile display; normal displays use a RGB subpixel layout, which means all three colours get equal representation. However, PenTile uses a layout where there are two green subpixels for each red and blue subpixel, resulting in full resolution for green and reduced resolution for red and blue. This can be seen in the third photo; there are considerably more green subpixels than red and blue subpixels. In a VR context, this works better than I was expecting, but it's still quite visible in places (see resolution of blue pixels in second photo.)

When in the Rift, the screen-door effect manifests as a weaving pattern over the visuals. It's definitely visible, no matter what you do, but depending on what content you're viewing, it may become background noise that you can ignore. This is especially most evident in content with a dark aesthetic, where the black grid blends in with the rest of the environment.

Overall, the headset itself is in good form. There are still improvements to be made for the consumer version, but it’s getting closer to mass market adoption. The headset could be more comfortable to use for extended periods of time, especially around the nose and cheeks. Lens separation adjustment (preferably automated) is a necessity for the consumer version; even with my expectations well-in-check as a developer, this one issue almost made the Rift unusable for me.

# Currently Released Applications
Due to my graphics card problems, I haven’t had the chance to develop anything for my Rift. As a result, I’ve been spending a large amount of time trying out other people’s applications and seeing what they’ve managed to do with the Rift. There are a few other applications that I've tried out, but they have significant performance problems on my current graphics card.

The screenshots were unwarped courtesy of [eVRydayVR's ffmpeg-unwarpvr](https://github.com/eVRydayVR/ffmpeg-unwarpvr/releases/tag/v0.2). Without unwarpvr, the screenshots would show barrel distortion and chromatic aberration; these artifacts are cancelled out by the artifacts of the lenses, resulting in a normal image. However, as these are screenshots of the distorted image, unwarpvr is necessary to get the original image.

## Oculus Demo Scene
![Unwarped screenshot of the Oculus Demo Scene, looking at the desk and background](demo1-unwarp.jpg)
![Unwarped screenshot of the Oculus Demo Scene, looking at the tower of cards](demo2-unwarp.jpg)

In the configuration utility, there is a demo scene you can view in order to test your calibration. This is not the famous ‘Tuscany’ demo that shipped with the DK1 (although said demo is still available); instead, it is a much simpler demo that places you in front of a desk with a few intricately modeled items, including a lamp, a potted plant, and a formation of cards.

It serves as a good introduction to the DK2’s new capabilities, especially positional tracking. To this end, there is an option that visualizes the camera tracking frustum. It served as a decent calibration experience and helped me get acclimated to the DK2; being able to look around the table and lean into the objects felt natural.

## Experience/Japan Teaser Trailer
![Unwarped screenshot of the first scene from Experience/Japan overlooking a terminus](experiencejapan1-unwarp.jpg)
![Unwarped screenshot of the last scene from Experience/Japan, overlooking a canal](experiencejapan3-unwarp.jpg)

I’ve always had an interest in going to Japan for a variety of reasons. While I’m quite a ways off from being able to do this for real, Experience/Japan, or at least its teaser trailer, gave me a taste of what it might be like in a very interesting, immersive experience that's practically sold the concept of virtual tourism to me.

[Experience/Japan](https://share.oculusvr.com/app/experiencejapan) is a 360-degree designed-for-VR film that will be released at some point in the future. As part of its announcement, a two-minute teaser trailer in the same style was released that shows a few key scenes from the larger film. As this is a 360-degree video, it is subject to several limitations, including the lack of stereoscopy and positional tracking (both of which require physical camera movement).

With that being said, it's no secret that this is one of my favorite experiences for the Rift so far. Even though it’s quite short, simplistic and has no stereoscopy, the use of real, live-action footage is compelling and helps sell the effect. There’s something surreal about virtual reality footage of a real escalator - it's utterly trivial, and yet quite mesmerizing when juxtaposed with the fact that I'm sitting in my room thousands of kilometres away.

The last two scenes (second screenshot) in the teaser trailer have you go through a city (which is presumably Tokyo) at night, lit up by bright electric banners. These two scenes were absolutely breathtaking when I first went through them; here I was, viewing a real city, with real people living their lives. It’ll be a while before anything computer-generated can get close to this.

## Proton Pulse
[Proton Pulse](http://www.vanguardv.com/?page_id=5) quickly rose to attention in the VR world when it was initially released for the DK1 before being taken down for unspecified reasons. Now, a year later, it’s back for the DK2, and it’s a damned good implementation. It’s a stylized VR version of Breakout in which you use your head to control the paddle; its core gameplay mechanic is addictive and well utilizes the VR format.

I found it quite entertaining, but quickly had to stop playing after about 5-10 minutes. To be fair, this was through no fault of the game itself. It relies on bouncing the ball around in your near field of view, but due to the IPD mismatch issue, this became quite painful as one of my eyes was trying to constantly focus on a blurry view.

To get an idea of what it was like, I had other people with closer IPDs try it out, and they were able to play it for extended periods of time without issues. In any case, I look forward to revisiting it when I can resolve the IPD issue.

## SightLine - The Chair
![Unwarped screenshot of the first scene from SightLine - The Chair; cubes in a shallow sea of water](sightline1-unwarp.jpg)
![Unwarped screenshot of the second-last scene from SightLine - The Chair; tendrils and tubes very close to the eye](sightline5-unwarp.jpg)

[SightLine - The Chair](https://share.oculusvr.com/app/sightline-the-chair) is another introductory VR experience that aims to ease the user into VR by utilizing the advantages of the medium. In it, the core mechanic is that the world changes when you’re not directly looking at it.

Performance was a little lacking, but that’s to be expected considering the graphics card I was using. The concept itself is fascinating; it seemed a little unreliable at times, but when it worked, it was magical; looking away from cubes submerged in an infinite sea, looking back, and being confronted with a lush forest was a unique experience.

Generally, it was a pleasant introduction to VR; there were a few scenes that came close to legitimately inducing some kind of emotion in me. There was a scene near the end that has you in an organically growing web of wires (see second screenshot), and it felt rather constrictive - even though it was clearly not real, I felt some level of primal fear.

## Virtual Desktop
![Unwarped screenshot of the Windows desktop, as seen through Virtual Desktop](vd1-unwarp.jpg)
![Unwarped screenshot of Super Hexagon, as seen through Virtual Desktop](vd2-unwarp.jpg)

[Virtual Desktop](http://www.vrdesktop.net/) is an application quite simple in concept, but at the same time one of the most useful applications available for the Rift. It takes your primary monitor and projects it on a (optionally curved) virtual screen.

It’s one of the most flexible applications I have used, as you can consume normal 2D content in an isolated environment. You can easily play a game, for example, on a massive screen that fills up your field of vision.

The developer is quite active and readily fixes any bugs that get reported. In general, I’ve had a very positive experience using it, and it’s one of my favorite applications to use on the Rift, simply because it makes certain tasks (such as launching another VR application) very convenient.

## Welcome to Oculus
![Unwarped screenshot of Welcome to Oculus' first room](wto1-unwarp.jpg)
![Unwarped screenshot of Welcome to Oculus' living room](wto2-unwarp.jpg)

[Welcome to Oculus](http://treyte.ch/oculus/) is another introductory demo that takes you through the history of media in order to contextualize the rapid advance of technology and the creation of consumer-friendly virtual reality. It does this by showing a variety of scenes, overlaid by the sweet, dulcet tones of what I can only assume to be text-to-speech. (That's something that could be significantly improved!)

While SightLine - The Chair attempted to take advantage of a line-of-sight mechanic to communicate the possibilities of VR, Welcome to Oculus takes this contextualization of media and reframes it in the perspective of VR, giving the user an insight into how VR developed and what it can be used for.

One of its central concessions is that it’s being viewed on a headset that’s capable of inducing presence (the term used to describe the subconscious feeling of actually being there in the world); one of the scenes it takes the user through explicitly assumes this, which slightly detracted from the experience when viewed on a DK2 (which is not quite there yet.)

Its audience seems to be somewhat targeted; the middle of the demo involves a character known widely to the Internet at hand, but with limited reach otherwise. It's understandable that the author(s) wanted to showcase how traditional media can translate to VR, but this particular example limits the potential reach of the demo. A potential improvement may have been to show the traditional media equivalent of this character, and then their VR equivalent - not only would this explain the character to those without prior background, it'd also communicate the effectiveness of VR with a direct comparison to traditional media.

However, in general, it was an excellent demo and I’m more than happy to recommend it as a starter introduction to what’s possible in VR. Throughout the demo, it emphasizes that VR isn’t just the latest and greatest medium: it’s capable of completely consuming and replacing all previous mediums.

## Radial-G
![Unwarped screenshot of the log-in screen for Radial-G](radialg1-unwarp.jpg)
![Unwarped screenshot of Radial-G gameplay](radialg2-unwarp.jpg)

[Radial-G](http://radial-g.com/) is an in-development racing game based around radial movement around a cylindrical track. A single-player demo has been released in order to demonstrate the concept, and it's been specifically designed with VR in mind.

Out of the VR games I’ve tested so far, this is by far my favorite one. The demo itself is a clean, polished concept and clearly demonstrates what they’re going for; it’s an engaging experience, and it plays very well into the strengths of VR. It’s easy to lose track of time while playing this on the Rift; its core gameplay mechanic is so utterly addictive that I’d end up spending half an hour just trying to beat my lap time without ever realizing the time spent at any point.

Its levels of polish are clear from the first moment you get to interact with the game. A clean, easy-to-use, *optional* log-in screen headlines the game, allowing you to store and compare lap records with others. For now, I'm playing non-competitively, so I was able to play offline without ever having to log in. Other racing games (see: TrackMania 2) could learn a thing or two from this.

Being able to look into corners and determine where I should go next is one of the essential features that I never really realized I needed in a racing game. It completely absorbs you and you begin to ignore the DK2’s flaws; it managed to distill the pure feeling of speed, and do it well. Even on the highest speed level, not once did I end up feeling motion sick.

This is something I’d highly recommend trying out if you’re at all a fan of racing games and you’re ready to move on from introductory VR experiences. Even though it’s early days yet for the game, it’s a fantastic demonstration of what VR can do for gaming; it’s completely natural and it is by far one of the best VR experiences I’ve had so far.

# Conclusion
The DK2 is hardly perfect; in fact, it’s already been superseded by both internal and external prototypes ([Crescent Bay](http://www.oculus.com/blog/oculus-connect-2014/)). However, this is irrelevant to its purpose: to allow developers to produce and test content. In this regard, it does very well - it provides almost all of the features that developers can expect to see in the consumer version, even if they're not at their best.

Virtual reality is the future. It’s the ultimate medium: it’s capable of showing everything prior to it, and there is no greater medium than being able to substitute reality itself. The DK2 is another taste of what’s to come: truly consumer-affordable, truly effective consumer virtual reality headsets that bring science fiction stories to life.

What you can do with virtual reality is literally limitless. I’ve only sampled and discussed a few key applications here, but it’s already evident that anything is possible. From revitalizing old game mechanics in VR (Proton Pulse) to virtual tourism (Experience/Japan) to emulation of old mediums (Virtual Desktop), there’s nothing to stop creators from imagining completely new worlds.

We are about to witness a renaissance in creativity. Our previous mediums had limitations that had to be adhered to; but with virtual reality, anything is possible. The revolution has begun.
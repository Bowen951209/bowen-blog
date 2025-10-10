---
layout: post
title:  "Fixing a broken audio that sounds normal on some devices but broken on others"
date:   2024-03-29 +0800
categories: audio
---

## The Problem
Here's an audio I [got](https://drive.google.com/file/d/19lPfO9cObfgaboDs60bwRG0c7Gg-69Wb/view?usp=sharing). It played 
normally on my PC, but sounded broken when playing on my phone. Try to play it on your 
devices and see if you encounter the same issue as mine.

## Why did that happen? — Destructive Interference
After diving into this problem, in my case, I found that it was caused by a physical phenomenon called 
**destructive interference**.
Interferences are divided into two types — constructive and destructive. When multiple waves add together, they interfere
with each other. The final wave is equal to the addition of those waves. When all the waves are identity, the final wave
will sound as same as the original, but the amplitude is bigger, which makes the sound louder, and that is the 
constructive interference. On the contrary, if two waves are opposite (one's crest happens at another's trough), the
addition of them becomes a flat pattern, making no sound, which is called the destructive interference.

Here is a figure showing this:

![interference]({{ site.baseurl }}/assets/images/fix_audio/interference.jpg)

[source](https://www.google.com/url?sa=i&url=https%3A%2F%2Fkids.britannica.com%2Fstudents%2Fassembly%2Fview%2F53869&psig=AOvVaw33kaGuH7MmnhUDcbA5C7f3&ust=1711793717527000&source=images&cd=vfe&opi=89978449&ved=0CBIQjRxqFwoTCNiT9tuemYUDFQAAAAAdAAAAABAE)

The waveform screenshot of my audio in iZotope RX 10:
![waveform]({{ site.baseurl }}/assets/images/fix_audio/oWave.jpg)
You can see that the two waves are almost completely opposite.

As you may have known, most of the audios contain two sound channels — the left and the right. And somehow some encoding
errors happened, and the two channels became opposite, which eventually causes the destructive interference if the 
speakers are put at a specific angle and position.

## How to Fix?
Of course, you can re-record it from other devices that don't have an interference, but I want to show you how to fix it
in an audio editor. I'll do it in RX10. If you don't have the software, others like [Audacity](https://www.audacityteam.org/)
should be similar.

### Select one of the channels and copy it with Ctrl + C
![copying]({{ site.baseurl }}/assets/images/fix_audio/copying.jpg)

### Delete both channel
![deleting]({{ site.baseurl }}/assets/images/fix_audio/deleting.jpg)

### Select both channel and paste with Ctrl + V
![pasting]({{ site.baseurl }}/assets/images/fix_audio/pasting.jpg)
As you can see, the two waves are now identity.

### Export the audio and done
![pasting]({{ site.baseurl }}/assets/images/fix_audio/exporting.jpg)
Export with whatever settings you want.

## Story About The Audio
This audio came from my Chinese teacher. It was a poem called **錯誤**, sung by his classmate when he was a high school 
student. The classmate, singer of this audio, has a miserable story. She was a victim of the "White Terror" in Taiwan. 
Her dad was sent to jail, and her relatives kept distance from her family. When we learned this poem in the Chinese class,
our teacher shared this song to us. Sadly, it wasn't able to play through our classroom's speakers.

I've once listened to a music on YouTube, and it had the same problem that my PC played fine, but my phone played with
broken sound. I can't find it now, but the teacher's audio seemed to have a same problem. I was really curious about what
the problem is, wanting to fix the audio. A day when I was taking a nap, there was a flash of inspiration, I tested it, 
and it really was that cause. So I wrote this post hoping that someone encountered the same problem can have a solution.
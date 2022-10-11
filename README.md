# Old Bear, New Tricks

## Credits
SUM fighter-code-edits channel: invaluable help
HDR Dev Team: Opensource macros used to help make this possible

## Uptilt
A new uptilt with a Banjo hitbox, and a Kazooie hitbox. This is an extremely unorthodox move with two different hitboxes serving two unique purposes. If you can master sliding uptilt and pivot tilts, this move might prove very useful.

Banjo's hitbox launches opponents, KOing at about the same percent dash attack does. It's pretty quick and also fairly safe on shield. It doesn't hit buried opponents, but might be good for catching them when they mash out. 

Kazooie's hitbox juggles opponents into the air. While it is unsafe on shield, it is also decently disjointed. At low percents it combos into itself and at mid percents, into upair and nair. There is also a very tight window where it combos into upsmash and mid percent.

Advantages:
- Hits on both sides
- Better for juggling
Disadvantages:
- Doesn't anti-air as well
- Worse for KOing
- Slightly less safe on shield

```diff
Specs:
+ FAF: 32 -> 30
- Shield Advantage: -11 (Banjo), -17 (Kazooie)
! Active: 6-12 (Banjo sourspot starts on 10)
! KBG (Banjo): 85,85
! BKB (Banjo): 72,42
! KBG (Kazooie): 78
! BKB (Kazooie): 57
! Damage: 10 (Banjo), 6.8 (Kazooie,Banjo Sour), 5.8 (Kazooie Sour)
! Angle: 70 (Banjo), 122 (Banjo Sour), 105 (Kazooie)
```

## Upsmash
Up smash is now Banjo's uptilt. It has been slightly nerfed as well, but the main reason is because most of your kit (nair second sweetspot,sometimes uptilt and upthrow, landing upair, breegull blaster) can combo into it, as well as being able to charge it to bait out an airdodge.

Advantages:
- No longer a multihits
- Stronger than the original upsmash
- Has a launch hitbox at the bottom, hitting small characters and burried foes
- Kills about 10% earlier than the original uptilt

Disadvantages:
- FAF is later, making this move less spammable
- Less safe on shield

```diff
Specs (Changes based on original Uptilt):
- Total Frames: +7
- FAF: 32 -> 48
- Startup: 11->13
- Active Frames: 4->3
- Shield Advantage: -10 -> -21
! KBG: 110 -> 90
! BKB: 42 -> 70
+ Damage: 10 -> 2,13
+ Launch hitbox added. Active in front of Banjo during frames 10-12. Attempts to launch opponents into the main hitbox
```

## Nair
Banjo now has an extremely versatile nair! Banjo swings his backpack 360 degrees, hitting in front, behind and then in front again. The bag is the sweetspot, while Banjo's arms are the sourspot. The first hit is the strongest, comparable to a slightly weaker Samus nair. The back hit is noticeably weaker, but gets opponents off of you. For the final hit, the sweetspot pops opponents up which can lead into fair/upsmash, but is harder to land.

Advantages:
- Safer
- Can combo into fair and upsmash
- KOs earlier
- Less landing lag

Disadvantages:
- No more dragdown nair
- Edgeguarding and 2-framing ledge recoveries is much more difficult
- Weaker hitboxes don't lead into much

Specs:
```diff
! Hitboxes on frames 8-12, 15-19,25-29
- Autocancellable on frames 1-6, 42+
+ Landing lag: 8
- Final Frame: 54
```

## Upair
Upair gets a much needed damage buff, and becomes easier to combo into itself

```diff
Specs:
+ FAF: 34->29
+ Damage: 5.8->7.8
```
## Fthrow
FThrow is now a much better "get off me" throw, as opponents now get sent further away. Tech chasing with wonderwing at low percents has also been removed, as well as the other low percent combos.

```diff
Specs:
+ Damage: 3/5->4/6
! Angle: 48->35
! KBG: 76->87
! BKG: 68->75
```
## Upthrow
Upthrow becomes a more useful juggle throw. 

```diff
Specs:
+ FAF: 56->46 (animation has been speed up as well, making FAF even sooner)
! BKB: 72->60
! KBG: 64->60
```

## Breegull Blaster (Neutral Special Stance)
Eggs no longer decay from spamming (they still deal less damage than Megaman pellets), and the FAF from cancelling is now sooner.
When inputting a smash attack during this stance, you will now perform a (angled) side tilt. This also removes Smash Turnaround Cancels, and by extension, TAS Shot

Advantages:
- Can launch opponents away during stance
- Egg confirms are now more lenient
- Eggs no longer lose strength while spamming

Disadvantages:
- No more TAS Shot
- Smash Turnaround Cancel -> Wonderwing also gone

```diff
Specs:
+ FAF from cancel: 12->10
! FTilt Startup (From Stance): 7->9
! FTilt FAF (From Stance): 31->27
```

## Shock Spring Pad (Up Special)
Falling hitbox size increased to more accurately reflect the model

## Wonderwing (Side Special Ground)
After using a gold feather, you can guard-cancel the move into the breaking animation from frame 15 onward, making the move slightly safer on whiff. Guard-canceling will deactivate the hitbox.

## Beakbomb (Side Special Air)
A new aerial side special that acts like an overall nerf to wonderwing, but also helps Banjo! This move has no invisibility and a smaller hitbox. It deals less damage, so grenegg into sideB doesn't really kill until higher percents. The move also has brief 7% armor on startup, and will send Banjo further and costs no gold feathers to use. Do note that this move has a 3 second cooldown on completion. 

You can also chose which angle to fly in by holding up or down on the control stick before taking off! Take care while flying, as landing on the ground too early will deal 10% damage and put you into the fail animation. Hitting sheilds is ill-advised, as you will recoil (akin to Flare Blitz). If you find yourself about to hit a wall, you can press the shield button within the first 4 frames of hitting a wall to perform a tech! Holding up lets you perform a walljump tech.

This move can also be guard-canceled like wonderwing, in case you accidentally b-reverse it and are headed towards the blastzone. Also, just because it's pretty cool, you can now Zdrop items while in the dash animation, transforming Kazooie into the Beak Bomber!

```diff
Specs:
- Startup: 18 -> 22
- FAF: 30 -> 45
- Invincibility removed
- Hitbox size decreased
- Self damage on landing (during certain frames)
- Damage: 22/16 -> 16/10
- Recoil on shield
- 3 second cooldown
! 7% Armor on frames 1-4
! KBG: 64->70
+ Items can be ZDropped during dash animation
+ No longer uses Gold Feathers
+ Airspeed: 4.5 -> 5.0
+ Shield Damage: 0->4
+ Guard-cancel during dash
+ Trajectory control (on startup)
+ Landing hitbox (Active for 2 frames on self damage)
+ Can tech walls
+ Bonk FAF (wall only): 37->20 
```

# Summary

With these specific changes, Banjo becomes much less of a linear character, especially when trying to seal a stock. His gameplan can now revolve around properly trapping opponents into uncomfortable situations, and pushing them offstage where his arsenal of tricks can gimp them, or deny them of returning to the stage. His lackluster ability to juggle opponents has been slightly remedied, but catching landing opponents still proves difficult. His more infamous tools have been greatly toned down, which should help playing against Banjo become a little more enjoyable. 

## Buffs
His new nair helps push opponents further back, leading them offstage where Banjo shines best. If your spacing is on point, you can also confirm a kill of the second sweetspot of nair, similar to Marth/Lucina/Chrom/Roy, but much more telegraphed. His original uptilt moves to a smash attack which now packs some serious damage on grounded foes. His new uptilt, while extremely unorthodox, serves several different purposes. His changed throws help him juggle opponents easier, or send them out of range if you need some space.

## Nerfs
His new aerial side B might be more versatile as a recovery move, but it lacks any invincibility which allows opponents to spike and edgeguard him without fear. Opponents can also land on the stage without the fear of buffering an airdodge/attack right into an invincible kill move. The Beak Bayonet (Neutral B Stance Smash Attack) has the unique function of removing TAS Shot and Smash Turnaround Cancels (which could true combo into wonderwing). His new uptilt, as useful as it is, is less safe on shield and doesn't kill as early, while the new upsmash is less safe on shield and slightly slower than it's original incarnation.

## Competitive Placement
Without having to struggle to take a stock or deal damage, Banjo escapes the realm of bottom tiers. However, these changes, as well as not touching up his anti-air game, prevent him from dominating a meta where this was the Banjo we got. Fast paced characters like Fox and Sheik will still shut him down, and characters that mitigate his juggling like Wario and Yoshi will still prove challenging. Though now, he has the tools to stand against bad matchups, and possibly sit comfortably as a mid or high tier.
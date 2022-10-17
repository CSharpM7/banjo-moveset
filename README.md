# Old Bear, New Tricks
(note shield advantage calculated via [smash ultimate calculator](https://rubendal.github.io/SSBU-Calculator/) and can be off by a few frames based on the hit frame)

## Credits
SUM fighter-code-edits channel: invaluable help
HDR Dev Team: Open-source macros used to help make this possible

## Base stats
Banjo is now much more mobile on the ground and in the air, but Kazooie has a slower run speed to compensate

```diff
Specs:
+ Initial Dash: 1.64->1.84
+ Walk Accel (mul/add): 0.04/0.06->0.08/0.12
- Run Speed Max: 2.18->2.05
+ Initial Dash: 1.64->1.84
+ Ground Jump: 29.8->31.0
! Air Jump: 32.0->33.0
+ Air Jump Y Velocity: 2.18/2.12->2.25/2.20
+ Air Jump X Velocity Multiplier: 0.8->1.0
+ Air Jump Frame: 30->24
```

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
- Shield Advantage: -10 (Banjo), -15 (Kazooie)
! Active: 6-12 (Banjo sourspot starts on 10)
! KBG (Banjo): 85,85
! BKB (Banjo): 72,42
! KBG (Kazooie): 78
! BKB (Kazooie): 57
! Damage: 10 (Banjo), 6.8 (Kazooie,Banjo Sour), 5.8 (Kazooie Sour)
! Angle: 70 (Banjo), 122 (Banjo Sour), 105 (Kazooie)
```

## Upsmash
Up smash is now Banjo's uptilt. It has been slightly nerfed as well, but the main reason is because most of your kit (nair second sweetspot,sometimes uptilt and upthrow, landing upair, breegull blaster) can combo into it, as well as having a low profile charge.

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
- Active Frames: 4->3
- Shield Advantage: -10 -> -21
! Startup: 11->13 (10 for bottom launch hitbox)
! KBG: 110 -> 90
! BKB: 42 -> 70
! Charge on Frame 7
+ Damage: 10 -> 4/13
+ Launch hitbox added. Active in front of Banjo during frames 13-14. Attempts to launch opponents into the main hitbox
```

## Nair
Banjo now has an extremely versatile nair! Banjo swings his backpack 360 degrees, hitting in front, behind and then in front again. The bag is the sweetspot, while Banjo's arms are the sourspot. The first hit is the strongest, comparable to a slightly weaker Lucina nair. The back hit is slightly weaker, but gets opponents off of you. For the final hit, the sweetspot pops opponents up which can lead into fair/upsmash, but is harder to land.

Advantages:
- Safer
- Can combo into fair, uptilt, upair and upsmash
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
+ Damage: 10/8, 9/7, 5/3
+ Shield Advantage: -7 (HEAVILY varies)
```

## Upair
Upair gets a slight damage buff, and becomes easier to combo into itself. Best used at low percents

```diff
Specs:
+ FAF: 34->29
+ Damage: 5.8->7.8
+ Autocancel: 30->26
+ Landing Lag: 12->9
! KBG: 80->100
! BKG: 42->37
```

## Back air
Back air gets a slight damage buff

```diff
Specs:
+ Damage: 1.6/4.8->2.6/5.8
! KBG: 136->96
! BKB: 72->68
```
## Down air
Down air gets a damage buff, lasts longer and will spike sooner, and can now bounce off of opponents. However, you will also fall faster.

```diff
Specs:
+ Damage (Spike): 10->13
+ BKB: 20->45
+ Active Frames (Spike): 15-17->15-20
+ Landing Lag: 27->22
+ Hitbox Size: 4.8/5.2 -> 5.8
+ Landing hitbox radius: 8->9
+ FAF: 57->50?
+ Bounce on hit/shield
- Fall Velocity: 3.8->4.0
```

## Fthrow
FThrow is now a much better "get off me" throw, as opponents now get sent further away. Tech chasing with wonderwing at low percents has also been removed, as well as the other low percent combos.

```diff
Specs:
+ Damage: 3/5->4/6
! Angle: 48->42
! KBG: 76->77
! BKG: 68->65
```
## Upthrow
Upthrow becomes a more useful juggle throw, comboing into upair until 60%+, and comboing into backair as well.

```diff
Specs:
+ FAF: 56->46 (animation has been speed up as well, making FAF even sooner)
+Angle: 88->95
! BKB: 72->60
! KBG: 64->60
```

## Breegull Blaster (Neutral Special Stance)
The FAF from cancelling is now sooner, and egg's power only decreases after the 15th egg. Remember to cancel the stance to refresh the egg's power.
When inputting a smash attack during this stance, you will now perform an (angled) side tilt. This also removes TAS Shot

Advantages:
- Can launch opponents away during stance
- Egg confirms are now more lenient
- Much more mobile

Disadvantages:
- No more TAS Shot
- Smash Turnaround Cancel doesn't really work

```diff
Specs:
+ Speed Multiplier: 0.5->1
+ Jump Frame: 32->30
+ Weaker Eggs: 6th egg/12th egg->15th egg
+ Visual indicator of weak eggs
+ FAF from cancel: 12->10
! FTilt Startup (From Stance): 7->9
! FTilt FAF (From Stance): 31->27
```


## Wonderwing (Side Special Ground)
After using a gold feather, you can guard-cancel the move into the breaking animation from frame 10 onward. Guard-canceling will deactivate the hitbox. Speed is also no longer lost on hit.

## Beakbomb (Side Special Air)
A new aerial side special that acts like an overall nerf to wonderwing, but also helps Banjo! This move has no invisibility and a smaller hitbox. It deals less damage, so grenegg into sideB doesn't really kill until higher percents. The move also has brief 7% armor on startup, and will send Banjo further and costs no gold feathers to use. Do note that this move has a 3 second cooldown on completion. 

You can also choose which angle to fly in by holding up or down on the control stick before taking off! Take care while flying, as landing on the ground too early will deal 10% damage and put you into the fail animation. Hitting sheilds is ill-advised, as you will recoil (akin to Flare Blitz), and if someone parries this, you are almost certainly dead! 

If you find yourself about to hit a wall, you can press the shield button within the first 4 frames of hitting a wall to perform a tech! Holding up lets you perform a walljump tech. This move can also be guard-canceled like wonderwing, in case you accidentally b-reverse it and are headed towards the blastzone.

Advantages:
- Easier to recover with
- No longer forced to lose a gold feather when trying to recover
- Can tech walls if you miss the very small ledge window
- Can avoid a potential ledge trap if your opponent commits too hard

Disadvantages:
- VERY vulnerable
- Does not kill until MUCH later
- Has a cooldown, so you'll have less resources to work with when trying to recover

```diff
Specs:
- Startup: 18 -> 22
- FAF: 30 -> 45
- FAF (Wall): 47->52 
- Invincibility removed
- Hitbox size decreased
- Kazooie Hurtbox active
- Self damage on landing (during certain frames)
- Damage: 22/16 -> 16/10
- Recoil on shield
- 7.5 second cooldown (cooldown speeds up when on ground)
! 7% Armor on frames 16-22
! BKB: 66->76
! KBG: 64->70
+ No longer uses Gold Feathers
+ Airspeed: 4.5 -> 4.6
+ Shield Damage: 0->4
+ Guard-cancel during dash
+ Trajectory control (on startup)
+ Can tech walls
```
## Shock Spring Pad (Up Special)
Falling hitbox size increased to more accurately reflect the model. You can now release the move earlier, but the maximum height you'll gain from this move has been reduced to compensate for Banjo's better aerial jumps.

```diff
Specs:
+ Minimum Charge: 15->10
- Max Height Multiplier: 1.28->1.25
```
## Grenegg (Down Special)
Greneggs now last longer. Hold down the special button to automatically equip the egg. While being able to easily equip grenegg is EXTREMELY helpful for recovering/stalling, the longer grenegg lifetime is needed to help balance this out. While the damage is an overall nerf, it can make grenegg into sideB confirm at later percents.

```diff
Specs:
+ Max Hops: 1->2
+ Frame 11 Equip (when holding down special)
- Damage: 8->6.5
! Life: 135->195
```

# Summary

With these specific changes, Banjo becomes much less of a linear character, especially when trying to seal a stock. His gameplan can now revolve around properly trapping opponents into uncomfortable situations, and pushing them offstage where his arsenal of tricks can gimp them, or deny them of returning to the stage. His lackluster ability to juggle opponents has been slightly remedied, but catching landing opponents still proves difficult. His more infamous tools have been greatly toned down, which should help playing against Banjo become a little more enjoyable. 

## Buffs
His new nair helps push opponents further back, leading them offstage where Banjo shines best. If your spacing is on point, you can also confirm a kill of the second sweetspot of nair, similar to Marth/Lucina/Chrom/Roy, but much more telegraphed. His original uptilt moves to a smash attack which now packs some serious damage on grounded foes. His new uptilt, while extremely unorthodox, serves several different purposes. His new upthrow and upair create a new combo at low percents to quickly tack 35%+ damage. His forward throw and back air now help establish stage control.

## Nerfs
His new aerial side B might be more versatile as a recovery move, but it lacks any invincibility which allows opponents to spike and edgeguard him without fear. Opponents can also land on the stage without the fear of buffering an airdodge/attack right into an invincible kill move. If someone lands a parry on Beakbomb, it's game over. The Beak Bayonet (Neutral B Stance Smash Attack) has the unique function of removing TAS Shot. His new uptilt, as useful as it is, is less safe on shield and doesn't kill as early, while the new upsmash is less safe on shield and slightly slower than it's original incarnation.

## Competitive Placement
Without having to struggle to take a stock or deal damage, Banjo escapes the realm of bottom tiers. He has competent damage output now, and shouldn't struggle to find a kill. However, these changes, as well as not touching up his anti-air game, prevent him from dominating a meta where this was the Banjo we got. Fast paced characters like Fox and Sheik will still shut him down, and characters that mitigate his juggling like Wario and Yoshi will still prove challenging. 
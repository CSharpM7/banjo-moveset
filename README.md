# Old Bear, New Tricks

## Credits
SUM fighter-code-edits channel: invaluable help
HDR Dev Team: Opensource macros used to help make this possible

## Uptilth
A new uptilt with a Banjo hitbox, and a Kazooie hitbox

Advantages:

Disadvantages:

```diff
Specs:
+ FAF: 32 -> 30
```

## Upsmash
Up smash is now Banjo's uptilt. 

Advantages:
- No longer a multihit
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
- Shield Advantage: -10 -> -22
! KBG: 110 -> 87
! BKB: 42 -> 40
+ Damage: 10 -> 15/16
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

## Breegull Blaster (Neutral Special Stance)
Eggs no longer decay from spamming (However, they are still affected by stale moves), and the FAF from cancelling is now sooner
When inputting a smash attack during this stance, you will now perform a (angled) side tilt. This also removes Smash Turnaround Cancels, and by extension, TAS Shot

Advantages:
- Can launch opponents away during stance
- Egg confirms more lenient

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
A new aerial side special that acts like an overall nerf to wonderwing, but also helps Banjo! This move has no invisibility and a smaller hitbox, but has brief 7% armor on startup, and will send Banjo further and costs no gold feathers to use. Banjo can also slightly control his trajectory, but will bail out if he lands on the ground too early. You can also Zdrop items while in the dash animation, transforming Kazooie into the Beak Bomber!

```diff
Specs:
- Startup: 18 -> 22
- FAF: 30 -> 45
- Invincibility removed
- Hitbox size decreased
- Self damage on landing (during certain frames)
- Damage: 22/16 -> 16/10 (Grenegg->SideB)
! Armor on frames 1-4
+ Items can be ZDropped during dash animation
+ No longer uses Gold Feathers
+ Airspeed: 4.5 -> 5.0
+ Trajectory control
+ Landing hitbox (Active for 2 frames)
```

# Summary

With these specific changes, Banjo becomes much less of a linear character, especially when trying to seal a stock. His gameplan can now revolve around properly trapping opponents into uncomfortable situations, and pushing them offstage where his arsenal of tricks can gimp them, or deny them of returning to the stage. His more infamous tools have been greatly toned down, which should help playing as (and playing against) Banjo become a little more enjoyable.

## Buffs
His new nair helps push opponents further back, potentially leading them offstage where Banjo shines best. If your spacing is on point, you can also confirm a kill of the sweetspot of nair, similar to Marth/Lucina/Chrom/Roy, but more telegraphed. His original uptilt moves to a smash attack, and is granted a brand new uptilt. While this makes the new upsmash less safe on shield and less spammable, his new uptilt allows Banjo to box and juggle better. Burying opponents also leads closer to a kill, as upsmash can hit buried opponents.

## Nerfs
His new aerial side B might be more versatile as a recovery move, but it lacks any invincibility which allows opponents to spike and edgeguard him without fear. Opponents can also land on the stage without the fear of buffering an airdodge/attack right into an invincible kill move. The Beak Bayonet (Neutral B Stance Smash Attack) has the unique function of removing TAS Shot and Smash Turnaround Cancels. While it does let Breegull Blaster become slightly more safe in close range, Banjo won't be able to take a stock at 60% off this stance. Landing cancels still remain, though. In short, opponents no longer need to worry about "cheese"